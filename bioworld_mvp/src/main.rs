use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;

const W: i32 = 25;
const H: i32 = 25;
const D: i32 = 8;
const INITIAL_POP: usize = 120;
const MAX_AGE: u32 = 1500;
const MAX_POP: usize = 600;

#[derive(Clone, Copy)]
struct Dna {
    move_randomness: f64,
    move_taxis: f64,
    energy_reserve_threshold: f64,
    signal_investment: f64,
    freq_preference: f64,
    freq_plasticity: f64,
}

#[derive(Clone)]
struct Memory {
    recent_energy_delta: Vec<f64>,
}

#[derive(Clone, Copy)]
enum Action {
    Move,
    Wait,
    Signal,
    Reproduce,
}

#[derive(Clone)]
struct Cell {
    id: u64,
    position: (i32, i32, i32),
    energy: f64,
    age: u32,
    generation: u32,
    dna: Dna,
    memory: Memory,
    last_action: Action,
    local_signal_state: f64,
    lineage_id: u64,
}

#[derive(Clone, Copy)]
enum DeathReason {
    EnergyDepleted,
    OldAge,
}

struct DeathRecord {
    universe_id: usize,
    generation: u32,
    lineage_id: u64,
    reason: DeathReason,
}

struct EliteRecord {
    universe_id: usize,
    generation: u32,
    lineage_id: u64,
    dna: Dna,
    energy: f64,
}

struct Akashic {
    death_records: Vec<DeathRecord>,
    elite_records: Vec<EliteRecord>,
}

#[derive(Clone)]
struct Boss {
    level: u8,
    pos: (i32, i32, i32),
    radius: i32,
    min_attackers: usize,
    reward: f64,
    damage: f64,
}

#[derive(Clone)]
struct Config {
    seed: u64,
    ticks: u32,
    mutation_rate: f64,
    mutation_sigma: f64,
    base_energy_input: f64,
    pressure_multiplier: f64,
    akashic_on: bool,
    boss_scale: f64,
    universe_id: usize,
}

#[derive(Default, Clone)]
struct TickMetrics {
    tick: u32,
    generation: u32,
    population: usize,
    births: u32,
    deaths: u32,
    average_energy: f64,
    dna_variance: f64,
    lineage_count: usize,
    cooperation_rate: f64,
    mean_cluster_size: f64,
    multi_cell_boss_success_rate: f64,
    energy_transfer_count: u32,
    signal_synchrony: f64,
    mutation_count: u32,
    nonzero_mutation_generations: u32,
    elite_lineage_survival: f64,
    adaptation_gain: f64,
    extinction_events: u32,
    cdi: f64,
}

#[derive(Clone)]
struct RunSummary {
    name: String,
    final_metrics: TickMetrics,
    pressure_death_rate: f64,
    boss3_single_success: f64,
    boss3_multi_success: f64,
    directional_shift_move_taxis: f64,
}

struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self { Self(seed ^ 0x9E3779B97F4A7C15) }
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.0
    }
    fn f64(&mut self) -> f64 { ((self.next_u64() >> 11) as f64) / ((1u64 << 53) as f64) }
    fn bool(&mut self, p: f64) -> bool { self.f64() < p.clamp(0.0, 1.0) }
    fn range_i32(&mut self, lo: i32, hi: i32) -> i32 { lo + (self.next_u64() % ((hi - lo) as u64)) as i32 }
    fn range_f64(&mut self, lo: f64, hi: f64) -> f64 { lo + self.f64() * (hi - lo) }
    fn normal(&mut self, sigma: f64) -> f64 {
        let u1 = (self.f64() + 1e-9).clamp(1e-9, 1.0);
        let u2 = self.f64();
        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        z * sigma
    }
}

fn wrap(v: i32, m: i32) -> i32 { ((v % m) + m) % m }
fn distance(a: (i32, i32, i32), b: (i32, i32, i32)) -> i32 { (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() }

fn random_dna(rng: &mut Rng) -> Dna {
    Dna {
        move_randomness: rng.range_f64(0.1, 0.9),
        move_taxis: rng.range_f64(0.1, 0.9),
        energy_reserve_threshold: rng.range_f64(22.0, 45.0),
        signal_investment: rng.range_f64(0.1, 0.8),
        freq_preference: rng.range_f64(0.0, 1.0),
        freq_plasticity: rng.range_f64(0.05, 0.6),
    }
}

fn mutate_dna(dna: Dna, cfg: &Config, rng: &mut Rng) -> (Dna, u32) {
    let mut m = 0;
    let mut mutate = |v: f64, lo: f64, hi: f64| {
        if rng.bool(cfg.mutation_rate) { m += 1; (v + rng.normal(cfg.mutation_sigma)).clamp(lo, hi) } else { v }
    };
    (Dna {
        move_randomness: mutate(dna.move_randomness, 0.0, 1.0),
        move_taxis: mutate(dna.move_taxis, 0.0, 1.0),
        energy_reserve_threshold: mutate(dna.energy_reserve_threshold, 15.0, 60.0),
        signal_investment: mutate(dna.signal_investment, 0.0, 1.0),
        freq_preference: mutate(dna.freq_preference, 0.0, 1.0),
        freq_plasticity: mutate(dna.freq_plasticity, 0.0, 1.0),
    }, m)
}

fn energy_field(x: i32, y: i32, z: i32, tick: u32, cfg: &Config) -> f64 {
    let a = ((x * 13 + y * 7 + z * 3 + tick as i32) as f64).sin();
    ((a + 1.2) * cfg.base_energy_input / cfg.pressure_multiplier).max(0.01)
}

fn run_simulation(name: &str, cfg: &Config, akashic: &mut Akashic) -> RunSummary {
    let mut rng = Rng::new(cfg.seed);
    let mut cells: Vec<Cell> = Vec::new();
    let mut occupancy: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut next_id = 1u64;

    for _ in 0..INITIAL_POP {
        let mut pos = (rng.range_i32(0, W), rng.range_i32(0, H), rng.range_i32(0, D));
        while occupancy.contains(&pos) { pos = (rng.range_i32(0, W), rng.range_i32(0, H), rng.range_i32(0, D)); }
        occupancy.insert(pos);
        let dna = if cfg.akashic_on && !akashic.elite_records.is_empty() && rng.bool(0.12) {
            let idx = (rng.next_u64() as usize) % akashic.elite_records.len();
            mutate_dna(akashic.elite_records[idx].dna, cfg, &mut rng).0
        } else { random_dna(&mut rng) };
        cells.push(Cell { id: next_id, position: pos, energy: rng.range_f64(18.0, 30.0), age: 0, generation: 0, dna, memory: Memory { recent_energy_delta: vec![] }, last_action: Action::Wait, local_signal_state: 0.0, lineage_id: next_id });
        next_id += 1;
    }

    let bosses = vec![
        Boss { level: 1, pos: (4,4,1), radius: 2, min_attackers: 1, reward: 20.0, damage: 6.0 * cfg.boss_scale },
        Boss { level: 2, pos: (12,12,3), radius: 3, min_attackers: 2, reward: 30.0, damage: 10.0 * cfg.boss_scale },
        Boss { level: 3, pos: (20,20,5), radius: 4, min_attackers: 3, reward: 45.0, damage: 16.0 * cfg.boss_scale },
    ];

    let mut csv = File::create(format!("runs/{}.csv", name)).unwrap();
    writeln!(csv, "tick,generation,population,births,deaths,average_energy,dna_variance,lineage_count,cooperation_rate,mean_cluster_size,multi_cell_boss_success_rate,energy_transfer_count,signal_synchrony,mutation_count,nonzero_mutation_generations,elite_lineage_survival,adaptation_gain,extinction_events,cdi").unwrap();

    let mut nonzero_mut: HashSet<u32> = HashSet::new();
    let mut extinctions = 0;
    let mut b3s_attempt = 0u32;
    let mut b3s_success = 0u32;
    let mut b3m_attempt = 0u32;
    let mut b3m_success = 0u32;
    let mut final_metrics = TickMetrics::default();
    let mut initial_mean_taxis = cells.iter().map(|c| c.dna.move_taxis).sum::<f64>() / cells.len() as f64;

    for tick in 0..cfg.ticks {
        if cells.is_empty() { extinctions += 1; break; }
        occupancy.clear();
        for c in &cells { occupancy.insert(c.position); }

        let mut births = 0u32;
        let mut deaths = 0u32;
        let mut mutation_count = 0u32;
        let mut energy_transfer_count = 0u32;
        let mut boss_success = 0u32;
        let mut coop_events = 0u32;
        let mut signal_sync_vals: Vec<f64> = vec![];

        let original_len = cells.len();
        for i in 0..original_len {
            if i >= cells.len() { break; }
            let mut new_child: Option<Cell> = None;
            {
                let cell = &mut cells[i];
                cell.age += 1;
                let old = cell.energy;
                cell.energy += energy_field(cell.position.0, cell.position.1, cell.position.2, tick, cfg);
                cell.energy -= 0.8 * cfg.pressure_multiplier;

                let near = energy_field(wrap(cell.position.0 + 1, W), cell.position.1, cell.position.2, tick, cfg);
                let grad = (near - old).max(0.0);
                let move_prob = (cell.dna.move_randomness + cell.dna.move_taxis * grad).clamp(0.0, 1.0);
                if rng.bool(move_prob) {
                    let np = (
                        wrap(cell.position.0 + rng.range_i32(-1, 2), W),
                        wrap(cell.position.1 + rng.range_i32(-1, 2), H),
                        wrap(cell.position.2 + rng.range_i32(-1, 2), D),
                    );
                    if !occupancy.contains(&np) {
                        occupancy.remove(&cell.position);
                        cell.position = np;
                        occupancy.insert(np);
                        cell.energy -= 0.6;
                        cell.last_action = Action::Move;
                    }
                }

                if rng.bool(cell.dna.signal_investment * 0.15) {
                    cell.local_signal_state = (cell.local_signal_state + cell.dna.freq_preference * cell.dna.freq_plasticity).fract();
                    cell.energy -= 0.3 + cell.dna.signal_investment;
                    cell.last_action = Action::Signal;
                }

                if cell.energy > cell.dna.energy_reserve_threshold {
                    let np = (
                        wrap(cell.position.0 + rng.range_i32(-1, 2), W),
                        wrap(cell.position.1 + rng.range_i32(-1, 2), H),
                        wrap(cell.position.2 + rng.range_i32(-1, 2), D),
                    );
                    if !occupancy.contains(&np) {
                        let (child_dna, m) = mutate_dna(cell.dna, cfg, &mut rng);
                        if m > 0 { nonzero_mut.insert(cell.generation + 1); }
                        mutation_count += m;
                        let child_energy = cell.energy * 0.42;
                        cell.energy *= 0.58;
                        births += 1;
                        new_child = Some(Cell { id: next_id, position: np, energy: child_energy, age: 0, generation: cell.generation + 1, dna: child_dna, memory: Memory { recent_energy_delta: vec![] }, last_action: Action::Reproduce, local_signal_state: cell.local_signal_state, lineage_id: cell.lineage_id });
                        next_id += 1;
                        occupancy.insert(np);
                    }
                }

                let delta = cell.energy - old;
                cell.memory.recent_energy_delta.push(delta);
                if cell.memory.recent_energy_delta.len() > 8 { cell.memory.recent_energy_delta.remove(0); }
            }
            if let Some(ch) = new_child { cells.push(ch); }
        }

        // local transfer
        let len = cells.len();
        for i in 0..len {
            if cells[i].energy > 28.0 && rng.bool(0.05) {
                let pos = cells[i].position;
                for j in 0..len {
                    if i != j && distance(pos, cells[j].position) <= 2 && cells[j].energy < 12.0 {
                        cells[i].energy -= 1.0;
                        cells[j].energy += 1.0;
                        energy_transfer_count += 1;
                        break;
                    }
                }
            }
        }

        for boss in &bosses {
            let idx: Vec<usize> = (0..cells.len()).filter(|i| distance(cells[*i].position, boss.pos) <= boss.radius).collect();
            if idx.is_empty() { continue; }
            let attackers = idx.len();
            let mean_signal = idx.iter().map(|i| cells[*i].local_signal_state).sum::<f64>() / attackers as f64;
            let synchrony = idx.iter().map(|i| 1.0 - (cells[*i].local_signal_state - mean_signal).abs()).sum::<f64>() / attackers as f64;
            signal_sync_vals.push(synchrony.clamp(0.0, 1.0));
            let invested = idx.iter().map(|i| cells[*i].dna.signal_investment).sum::<f64>() / attackers as f64;
            let success = attackers >= boss.min_attackers && synchrony > 0.55 && invested > 0.25;
            if boss.level == 3 {
                if attackers == 1 { b3s_attempt += 1; } else { b3m_attempt += 1; }
            }
            if success {
                boss_success += 1;
                if attackers > 1 { coop_events += 1; }
                for i in &idx { cells[*i].energy += boss.reward / attackers as f64; }
                if boss.level == 3 {
                    if attackers == 1 { b3s_success += 1; } else { b3m_success += 1; }
                }
            } else {
                for i in &idx { cells[*i].energy -= boss.damage / attackers as f64; }
            }
        }

        let mut alive = Vec::with_capacity(cells.len());
        for c in cells.drain(..) {
            if c.energy <= 0.0 || c.age > MAX_AGE {
                deaths += 1;
                akashic.death_records.push(DeathRecord { universe_id: cfg.universe_id, generation: c.generation, lineage_id: c.lineage_id, reason: if c.energy <= 0.0 { DeathReason::EnergyDepleted } else { DeathReason::OldAge } });
            } else {
                if c.energy > 55.0 {
                    akashic.elite_records.push(EliteRecord { universe_id: cfg.universe_id, generation: c.generation, lineage_id: c.lineage_id, dna: c.dna, energy: c.energy });
                }
                alive.push(c);
            }
        }
        if alive.len() > MAX_POP {
            alive.sort_by(|a,b| b.energy.partial_cmp(&a.energy).unwrap());
            alive.truncate(MAX_POP);
        }
        cells = alive;

        let pop = cells.len();
        let avg_energy = if pop > 0 { cells.iter().map(|c| c.energy).sum::<f64>() / pop as f64 } else { 0.0 };
        let mut dna_values = Vec::new();
        for c in &cells {
            dna_values.push(c.dna.move_randomness);
            dna_values.push(c.dna.move_taxis);
            dna_values.push(c.dna.energy_reserve_threshold / 60.0);
            dna_values.push(c.dna.signal_investment);
            dna_values.push(c.dna.freq_preference);
            dna_values.push(c.dna.freq_plasticity);
        }
        let dv = if dna_values.is_empty() { 0.0 } else {
            let mean = dna_values.iter().sum::<f64>() / dna_values.len() as f64;
            dna_values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / dna_values.len() as f64
        };
        let mut lineages = HashSet::new();
        for c in &cells { lineages.insert(c.lineage_id); }
        let mean_cluster_size = if pop == 0 { 0.0 } else {
            let mut occ = HashSet::new();
            for c in &cells { occ.insert(c.position); }
            let mut total = 0.0;
            for c in &cells {
                let mut n = 1;
                let dirs = [(1,0,0),(-1,0,0),(0,1,0),(0,-1,0),(0,0,1),(0,0,-1)];
                for d in dirs {
                    let p = (wrap(c.position.0+d.0,W),wrap(c.position.1+d.1,H),wrap(c.position.2+d.2,D));
                    if occ.contains(&p) { n += 1; }
                }
                total += n as f64;
            }
            total / pop as f64
        };
        let cooperation_rate = if boss_success == 0 { 0.0 } else { coop_events as f64 / boss_success as f64 };
        let signal_synchrony = if signal_sync_vals.is_empty() { 0.0 } else { signal_sync_vals.iter().sum::<f64>() / signal_sync_vals.len() as f64 };
        let elite_lineage_survival = if akashic.elite_records.is_empty() { 0.0 } else {
            let mut elite = HashSet::new();
            let mut alive_l = HashSet::new();
            for e in &akashic.elite_records { elite.insert(e.lineage_id); }
            for c in &cells { alive_l.insert(c.lineage_id); }
            let overlap = elite.intersection(&alive_l).count();
            if elite.is_empty() { 0.0 } else { overlap as f64 / elite.len() as f64 }
        };
        let adaptation_gain = avg_energy - 22.0;
        let memory_depth = if pop == 0 { 0.0 } else { cells.iter().map(|c| c.memory.recent_energy_delta.len() as f64).sum::<f64>() / pop as f64 / 8.0 };
        let action_diversity = 0.6 * cooperation_rate + 0.4 * if births + deaths > 0 { 1.0 } else { 0.0 };
        let cooperation_index = 0.5 * cooperation_rate + 0.5 * signal_synchrony;
        let survival_adaptation = (adaptation_gain / 20.0).clamp(-1.0, 2.0);
        let cdi = 0.25 * memory_depth + 0.25 * action_diversity + 0.25 * cooperation_index + 0.25 * survival_adaptation;

        final_metrics = TickMetrics {
            tick,
            generation: cells.iter().map(|c| c.generation).max().unwrap_or(0),
            population: pop,
            births,
            deaths,
            average_energy: avg_energy,
            dna_variance: dv,
            lineage_count: lineages.len(),
            cooperation_rate,
            mean_cluster_size,
            multi_cell_boss_success_rate: if b3m_attempt == 0 { 0.0 } else { b3m_success as f64 / b3m_attempt as f64 },
            energy_transfer_count,
            signal_synchrony,
            mutation_count,
            nonzero_mutation_generations: nonzero_mut.len() as u32,
            elite_lineage_survival,
            adaptation_gain,
            extinction_events: extinctions,
            cdi,
        };

        writeln!(csv, "{},{},{},{},{},{:.4},{:.6},{},{:.4},{:.4},{:.4},{},{:.4},{},{},{:.4},{:.4},{},{:.4}",
            final_metrics.tick, final_metrics.generation, final_metrics.population, final_metrics.births, final_metrics.deaths,
            final_metrics.average_energy, final_metrics.dna_variance, final_metrics.lineage_count, final_metrics.cooperation_rate,
            final_metrics.mean_cluster_size, final_metrics.multi_cell_boss_success_rate, final_metrics.energy_transfer_count,
            final_metrics.signal_synchrony, final_metrics.mutation_count, final_metrics.nonzero_mutation_generations,
            final_metrics.elite_lineage_survival, final_metrics.adaptation_gain, final_metrics.extinction_events, final_metrics.cdi).unwrap();
    }

    let mean_taxis = if cells.is_empty() { initial_mean_taxis } else { cells.iter().map(|c| c.dna.move_taxis).sum::<f64>() / cells.len() as f64 };
    RunSummary {
        name: name.to_string(),
        pressure_death_rate: if final_metrics.population + final_metrics.deaths as usize == 0 { 0.0 } else { final_metrics.deaths as f64 / (final_metrics.population as f64 + final_metrics.deaths as f64) },
        boss3_single_success: if b3s_attempt == 0 { 0.0 } else { b3s_success as f64 / b3s_attempt as f64 },
        boss3_multi_success: if b3m_attempt == 0 { 0.0 } else { b3m_success as f64 / b3m_attempt as f64 },
        directional_shift_move_taxis: mean_taxis - initial_mean_taxis,
        final_metrics,
    }
}

fn summary_json(s: &RunSummary) -> String {
    format!(
        "{{\"name\":\"{}\",\"pressure_death_rate\":{:.6},\"boss3_single_success\":{:.6},\"boss3_multi_success\":{:.6},\"directional_shift_move_taxis\":{:.6},\"final_metrics\":{{\"tick\":{},\"generation\":{},\"population\":{},\"births\":{},\"deaths\":{},\"average_energy\":{:.6},\"dna_variance\":{:.6},\"lineage_count\":{},\"cooperation_rate\":{:.6},\"mean_cluster_size\":{:.6},\"multi_cell_boss_success_rate\":{:.6},\"energy_transfer_count\":{},\"signal_synchrony\":{:.6},\"mutation_count\":{},\"nonzero_mutation_generations\":{},\"elite_lineage_survival\":{:.6},\"adaptation_gain\":{:.6},\"extinction_events\":{},\"cdi\":{:.6}}}}}",
        s.name, s.pressure_death_rate, s.boss3_single_success, s.boss3_multi_success, s.directional_shift_move_taxis,
        s.final_metrics.tick, s.final_metrics.generation, s.final_metrics.population, s.final_metrics.births, s.final_metrics.deaths,
        s.final_metrics.average_energy, s.final_metrics.dna_variance, s.final_metrics.lineage_count, s.final_metrics.cooperation_rate,
        s.final_metrics.mean_cluster_size, s.final_metrics.multi_cell_boss_success_rate, s.final_metrics.energy_transfer_count,
        s.final_metrics.signal_synchrony, s.final_metrics.mutation_count, s.final_metrics.nonzero_mutation_generations,
        s.final_metrics.elite_lineage_survival, s.final_metrics.adaptation_gain, s.final_metrics.extinction_events, s.final_metrics.cdi
    )
}

fn main() {
    fs::create_dir_all("runs").unwrap();
    fs::create_dir_all("akashic").unwrap();
    let mut akashic = Akashic { death_records: vec![], elite_records: vec![] };

    let summaries = vec![
        run_simulation("experiment_a_survival", &Config { seed: 1, ticks: 10_000, mutation_rate: 0.01, mutation_sigma: 0.08, base_energy_input: 1.8, pressure_multiplier: 1.0, akashic_on: false, boss_scale: 1.0, universe_id: 1 }, &mut akashic),
        run_simulation("experiment_b_evolution", &Config { seed: 2, ticks: 1_000, mutation_rate: 0.02, mutation_sigma: 0.12, base_energy_input: 1.6, pressure_multiplier: 1.1, akashic_on: false, boss_scale: 1.1, universe_id: 2 }, &mut akashic),
        run_simulation("experiment_c_pressure_low", &Config { seed: 3, ticks: 1_000, mutation_rate: 0.01, mutation_sigma: 0.08, base_energy_input: 2.1, pressure_multiplier: 0.8, akashic_on: false, boss_scale: 0.8, universe_id: 3 }, &mut akashic),
        run_simulation("experiment_c_pressure_high", &Config { seed: 4, ticks: 1_000, mutation_rate: 0.01, mutation_sigma: 0.08, base_energy_input: 1.3, pressure_multiplier: 1.5, akashic_on: false, boss_scale: 1.5, universe_id: 4 }, &mut akashic),
        run_simulation("experiment_d_cooperation", &Config { seed: 5, ticks: 1_000, mutation_rate: 0.012, mutation_sigma: 0.09, base_energy_input: 1.6, pressure_multiplier: 1.2, akashic_on: false, boss_scale: 1.3, universe_id: 5 }, &mut akashic),
        run_simulation("experiment_e_akashic_off", &Config { seed: 6, ticks: 1_000, mutation_rate: 0.012, mutation_sigma: 0.09, base_energy_input: 1.5, pressure_multiplier: 1.2, akashic_on: false, boss_scale: 1.25, universe_id: 6 }, &mut akashic),
        run_simulation("experiment_e_akashic_on", &Config { seed: 7, ticks: 1_000, mutation_rate: 0.012, mutation_sigma: 0.09, base_energy_input: 1.5, pressure_multiplier: 1.2, akashic_on: true, boss_scale: 1.25, universe_id: 7 }, &mut akashic),
    ];

    let mut out = File::create("runs/summary.json").unwrap();
    writeln!(out, "[").unwrap();
    for (i, s) in summaries.iter().enumerate() {
        if i + 1 == summaries.len() { writeln!(out, "  {}", summary_json(s)).unwrap(); }
        else { writeln!(out, "  {},", summary_json(s)).unwrap(); }
    }
    writeln!(out, "]").unwrap();

    let mut ak = File::create("akashic/akashic_archive.json").unwrap();
    writeln!(ak, "{{\"death_records\":{},\"elite_records\":{}}}", akashic.death_records.len(), akashic.elite_records.len()).unwrap();
}
