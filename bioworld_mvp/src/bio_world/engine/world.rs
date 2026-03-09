use std::collections::HashSet;
use std::io::Write;

use crate::bio_world::akashic::akashic_archive::{AkashicArchive, DeathRecord, EliteDna};
use crate::bio_world::akashic::cross_universe_transfer::sample_akashic_dna;
use crate::bio_world::boss::boss_system::{build_bosses, Boss};
use crate::bio_world::engine::cell::{Cell, CooperationState, SignalState};
use crate::bio_world::engine::dna::{Dna, MutationEvent};
use crate::bio_world::engine::energy::EnergyLedger;
use crate::bio_world::engine::physics::{manhattan, wrap, GRID_X, GRID_Y, GRID_Z};
use crate::bio_world::evolution::cooperation::cooperation_success;
use crate::bio_world::evolution::mutation::mutate_dna;
use crate::bio_world::evolution::reproduction::can_reproduce;
use crate::bio_world::evolution::selection::survive;
use crate::bio_world::memory::access_guard::{
    AccessMode, AccessRequest, Accessor, MemoryAccessGuard, Target,
};
use crate::bio_world::memory::causal_archive::{
    ArchiveSamplingPolicy, CausalArchive, CausalArchiveRecord,
};
use crate::bio_world::memory::cell_memory::CellMemory;
use crate::bio_world::memory::constants::{
    ARCHIVE_SAMPLE_PROBABILITY, MAX_ARCHIVE_WRITE_RATE, MAX_CELL_MEMORY_WINDOW,
    MAX_DISTILLED_LESSONS, SAMPLES_PER_LIFETIME,
};
use crate::bio_world::memory::lineage_memory::LineageMemory;
use crate::bio_world::metrics::cdi::compute_cdi;
use crate::bio_world::metrics::stability::{extinction_probability, hazard_rate};
use crate::bio_world::output::csv_logger::CsvLoggers;

#[derive(Clone, Debug)]
pub struct Config {
    pub universe_id: usize,
    pub seed: u64,
    pub ticks: u32,
    pub pressure: f64,
    pub akashic_on: bool,
    // P1 Experiment parameters
    pub disable_lineage_memory: bool,     // P1-A: Memory KO
    pub cooperation_multiplier: f64,       // P1-B: Cooperation suppression
}

#[derive(Clone, Debug, Default)]
pub struct RunSummary {
    pub universe_id: usize,
    pub seed: u64,
    pub births: u64,
    pub deaths: u64,
    pub mutation_count: u64,
    pub single_boss_success: f64,
    pub multi_boss_success: f64,
    pub adaptation_gain: f64,
}

pub struct Rng(u64);
impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed ^ 0x9E3779B97F4A7C15)
    }
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.0
    }
    pub fn f64(&mut self) -> f64 {
        ((self.next_u64() >> 11) as f64) / ((1u64 << 53) as f64)
    }
    pub fn bool(&mut self, p: f64) -> bool {
        self.f64() < p.clamp(0.0, 1.0)
    }
    pub fn range_i32(&mut self, lo: i32, hi: i32) -> i32 {
        lo + (self.next_u64() % (hi - lo) as u64) as i32
    }
    pub fn range_f64(&mut self, lo: f64, hi: f64) -> f64 {
        lo + self.f64() * (hi - lo)
    }
    pub fn normal(&mut self, sigma: f64) -> f64 {
        let u1 = (self.f64() + 1e-9).clamp(1e-9, 1.0);
        let u2 = self.f64();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos() * sigma
    }
}

fn random_dna(rng: &mut Rng) -> Dna {
    Dna {
        move_speed: rng.range_f64(0.1, 1.0),
        sensing_radius: rng.range_i32(1, 8),
        attack_power: rng.range_f64(0.1, 1.0),
        defense: rng.range_f64(0.1, 1.0),
        cooperation_willingness: rng.range_f64(0.0, 1.0),
        signal_strength: rng.range_f64(0.0, 1.0),
        signal_frequency: rng.range_f64(0.0, 1.0),
        memory_capacity: rng.range_i32(2, 16) as usize,
        learning_rate: rng.range_f64(0.0, 1.0),
        mutation_rate: rng.range_f64(0.01, 0.12),
    }
}

fn local_energy(pos: (i32, i32, i32), tick: u32, pressure: f64) -> f64 {
    (((pos.0 * 13 + pos.1 * 7 + pos.2 * 3 + tick as i32) as f64).sin() + 1.3).max(0.0) * 1.4
        / pressure
}

pub fn run_universe(cfg: &Config, archive: &mut AkashicArchive, base_runs: &str) -> RunSummary {
    let mut rng = Rng::new(cfg.seed);
    let mut cells = Vec::<Cell>::new();
    let mut occ = HashSet::new();
    let mut next_id = 1u64;
    let max_pop = 3000usize;
    let mut log = CsvLoggers::new(base_runs, cfg.universe_id);
    let bosses = build_bosses();
    let mut mutation_history: Vec<MutationEvent> = Vec::new();
    let mut memory_archive = CausalArchive::default();
    let mut access_guard = MemoryAccessGuard::default();

    for _ in 0..300 {
        let mut p = (
            rng.range_i32(0, GRID_X),
            rng.range_i32(0, GRID_Y),
            rng.range_i32(0, GRID_Z),
        );
        while occ.contains(&p) {
            p = (
                rng.range_i32(0, GRID_X),
                rng.range_i32(0, GRID_Y),
                rng.range_i32(0, GRID_Z),
            );
        }
        occ.insert(p);
        let dna = if cfg.akashic_on {
            sample_akashic_dna(archive, &mut rng).unwrap_or_else(|| random_dna(&mut rng))
        } else {
            random_dna(&mut rng)
        };
        cells.push(Cell {
            id: next_id,
            position: p,
            energy: rng.range_f64(18.0, 35.0),
            age: 0,
            alive: true,
            dna,
            cell_memory: CellMemory::default(),
            cooperation_state: CooperationState {
                recent_partner_count: 0,
                synchrony_score: 0.0,
            },
            signal_state: SignalState {
                phase: rng.f64(),
                investment: 0.0,
            },
            lineage_id: next_id,
            lineage_memory: LineageMemory::new(next_id),
            archive_samples_taken: 0,
        });
        next_id += 1;
    }

    let mut total_births = 0u64;
    let mut total_deaths = 0u64;
    let mut total_mut = 0u64;
    let mut b_single_attempt = 0u64;
    let mut b_single_success = 0u64;
    let mut b_multi_attempt = 0u64;
    let mut b_multi_success = 0u64;
    let baseline_energy = cells.iter().map(|c| c.energy).sum::<f64>() / cells.len() as f64;
    let mut extinction_events = 0u32;

    for tick in 0..cfg.ticks {
        if cells.is_empty() {
            extinction_events += 1;
            break;
        }
        let mut ledger = EnergyLedger::default();
        occ.clear();
        for c in &cells {
            occ.insert(c.position);
        }

        let mut births = 0u32;
        let mut deaths = 0u32;
        let mut mut_count = 0u32;
        let mut transfer = 0u32;
        let mut moved = 0u32;
        let mut archive_sample_attempts = 0u32;
        let mut archive_sample_successes = 0u32;
        let mut archive_influenced_births = 0u32;

        let current_len = cells.len();
        for i in 0..current_len {
            if i >= cells.len() {
                break;
            }
            let mut child: Option<Cell> = None;
            {
                let c = &mut cells[i];
                c.age += 1;
                let e = local_energy(c.position, tick, cfg.pressure);
                c.energy += e;
                ledger.input_environment += e;

                if rng.bool(0.03) {
                    c.energy += 3.0;
                    ledger.input_food += 3.0;
                }

                if rng.bool(c.dna.move_speed * 0.5) {
                    let np = (
                        wrap(c.position.0 + rng.range_i32(-1, 2), GRID_X),
                        wrap(c.position.1 + rng.range_i32(-1, 2), GRID_Y),
                        wrap(c.position.2 + rng.range_i32(-1, 2), GRID_Z),
                    );
                    if !occ.contains(&np) {
                        occ.remove(&c.position);
                        c.position = np;
                        occ.insert(np);
                        moved += 1;
                        c.energy -= 0.6;
                        ledger.cost_movement += 0.6;
                    }
                }

                c.signal_state.phase =
                    (c.signal_state.phase + c.dna.signal_frequency * 0.07).fract();
                c.signal_state.investment = c.dna.signal_strength * c.dna.cooperation_willingness;
                c.energy -= c.signal_state.investment * 0.3;
                ledger.cost_signal += c.signal_state.investment * 0.3;

                c.cell_memory.record_energy(c.energy as f32);
                c.cell_memory.record_signal(c.signal_state.phase as f32);
                c.cell_memory.record_experience(format!("tick:{}", tick));
                c.cell_memory.stress_level = (1.0 - (c.energy as f32 / 100.0)).clamp(0.0, 1.0);
                c.cell_memory.decay();
                debug_assert!(c.cell_memory.recent_energy_history.len() <= MAX_CELL_MEMORY_WINDOW);
                let mcost = c.cell_memory.recent_energy_history.len() as f64 * 0.01;
                c.energy -= mcost;
                ledger.cost_memory += mcost;

                if can_reproduce(c) && rng.bool(0.2 + c.dna.learning_rate * 0.2) {
                    let np = (
                        wrap(c.position.0 + rng.range_i32(-1, 2), GRID_X),
                        wrap(c.position.1 + rng.range_i32(-1, 2), GRID_Y),
                        wrap(c.position.2 + rng.range_i32(-1, 2), GRID_Z),
                    );
                    if !occ.contains(&np) {
                        let (ndna, ev) = mutate_dna(&c.dna, &mut rng, tick, c.lineage_id);
                        mut_count += ev.len() as u32;
                        mutation_history.extend(ev);
                        let e_child = c.energy * 0.4;
                        c.energy *= 0.6;
                        let rcost = 0.8;
                        c.energy -= rcost;
                        ledger.cost_reproduction += rcost;
                        births += 1;
                        let mut child_lineage =
                            LineageMemory::inherit_from(&c.lineage_memory, &mut rng);
                        if c.archive_samples_taken < SAMPLES_PER_LIFETIME {
                            archive_sample_attempts += 1;
                            access_guard
                                .validate(AccessRequest {
                                    accessor: Accessor::Lineage(c.lineage_id),
                                    target: Target::Archive,
                                    mode: AccessMode::Read,
                                    sample_probability: Some(ARCHIVE_SAMPLE_PROBABILITY),
                                })
                                .expect("lineage read of archive must obey guard");
                            if let Some(record) = memory_archive
                                .random_sample(&mut rng, &ArchiveSamplingPolicy::default())
                            {
                                child_lineage
                                    .push_lesson(CausalArchive::compress_to_lesson(record));
                                archive_sample_successes += 1;
                                archive_influenced_births += 1;
                                c.archive_samples_taken += 1;
                            }
                        }
                        debug_assert!(
                            child_lineage.distilled_lessons.len() <= MAX_DISTILLED_LESSONS
                        );
                        child = Some(Cell {
                            id: next_id,
                            position: np,
                            energy: e_child,
                            age: 0,
                            alive: true,
                            dna: ndna,
                            cell_memory: CellMemory::default(),
                            cooperation_state: CooperationState {
                                recent_partner_count: 0,
                                synchrony_score: 0.0,
                            },
                            signal_state: SignalState {
                                phase: rng.f64(),
                                investment: 0.0,
                            },
                            lineage_id: c.lineage_id,
                            lineage_memory: child_lineage,
                            archive_samples_taken: 0,
                        });
                        next_id += 1;
                        occ.insert(np);
                    }
                }
            }
            if let Some(ch) = child {
                cells.push(ch);
            }
        }

        for b in &bosses {
            resolve_boss(
                b,
                &mut cells,
                &mut ledger,
                &mut transfer,
                &mut b_single_attempt,
                &mut b_single_success,
                &mut b_multi_attempt,
                &mut b_multi_success,
            );
        }

        let mut alive = Vec::with_capacity(cells.len());
        for c in cells.drain(..) {
            if survive(&c) {
                if c.energy > 80.0 {
                    archive.elite_dna.push(EliteDna {
                        source_universe: cfg.universe_id,
                        generation: tick,
                        lineage_id: c.lineage_id,
                        dna: c.dna.clone(),
                    });
                }
                alive.push(c);
            } else {
                deaths += 1;
                let death_reason = if c.energy <= 0.0 {
                    "energy".to_string()
                } else {
                    "age".to_string()
                };
                archive.death_records.push(DeathRecord {
                    source_universe: cfg.universe_id,
                    generation: tick,
                    lineage_id: c.lineage_id,
                    reason: death_reason.clone(),
                });
                memory_archive.queue_record(CausalArchiveRecord {
                    generation: tick,
                    lineage_id: c.lineage_id,
                    event_type: "death".to_string(),
                    payload: death_reason,
                });
            }
        }
        if alive.len() > max_pop {
            alive.sort_by(|a, b| b.energy.partial_cmp(&a.energy).unwrap());
            alive.truncate(max_pop);
        }
        cells = alive;

        if tick % 100 == 0 {
            memory_archive.reset_rate_window();
        }
        debug_assert!(MAX_ARCHIVE_WRITE_RATE <= 1);
        debug_assert!(access_guard
            .validate(AccessRequest {
                accessor: Accessor::Cell(0),
                target: Target::Archive,
                mode: AccessMode::Read,
                sample_probability: None,
            })
            .is_err());
        debug_assert!(access_guard
            .validate(AccessRequest {
                accessor: Accessor::Archive,
                target: Target::CellMemory(0),
                mode: AccessMode::Write,
                sample_probability: None,
            })
            .is_err());
        memory_archive.process_queue();
        memory_archive.compress_old_records();

        // simple local transfer
        let l = cells.len();
        for i in 0..l {
            if cells[i].energy > 50.0 && rng.bool(0.03) {
                for j in 0..l {
                    if i != j
                        && manhattan(cells[i].position, cells[j].position) <= 2
                        && cells[j].energy < 15.0
                    {
                        cells[i].energy -= 1.2;
                        cells[j].energy += 1.2;
                        transfer += 1;
                        break;
                    }
                }
            }
        }

        let pop = cells.len();
        if pop == 0 {
            extinction_events += 1;
        }
        let avg_energy = if pop == 0 {
            0.0
        } else {
            cells.iter().map(|c| c.energy).sum::<f64>() / pop as f64
        };
        let mut lineage = HashSet::new();
        let mut all_dna = Vec::new();
        let mut lineage_counts = std::collections::HashMap::<u64, usize>::new();
        let mut strategy_counts = std::collections::HashMap::<String, usize>::new();
        for c in &cells {
            lineage.insert(c.lineage_id);
            all_dna.extend(c.dna.as_vec());
            *lineage_counts.entry(c.lineage_id).or_insert(0) += 1;
            *strategy_counts
                .entry(c.lineage_memory.preferred_strategy.clone())
                .or_insert(0) += 1;
        }
        let dna_var = variance(&all_dna);

        let signal_div = signal_diversity(&cells);
        let coop_density = (transfer as f64 / (pop.max(1) as f64)).clamp(0.0, 1.0);
        let mem_use = if pop == 0 {
            0.0
        } else {
            cells
                .iter()
                .map(|c| c.cell_memory.utilization() as f64)
                .sum::<f64>()
                / pop as f64
        };
        let avg_stress_level = if pop == 0 {
            0.0
        } else {
            cells
                .iter()
                .map(|c| c.cell_memory.stress_level as f64)
                .sum::<f64>()
                / pop as f64
        };
        let lineage_diversity = if pop == 0 {
            0.0
        } else {
            lineage_counts.len() as f64 / pop as f64
        };
        let top1_lineage_share = if pop == 0 {
            0.0
        } else {
            let max_lineage = lineage_counts.values().copied().max().unwrap_or(0);
            max_lineage as f64 / pop as f64
        };
        let strategy_entropy = if pop == 0 {
            0.0
        } else {
            strategy_counts
                .values()
                .map(|count| {
                    let p = *count as f64 / pop as f64;
                    if p <= f64::EPSILON {
                        0.0
                    } else {
                        -p * p.ln()
                    }
                })
                .sum::<f64>()
        };
        let collapse_event_count = extinction_events as u64;

        let explore = moved as f64 / pop.max(1) as f64;
        let cdi = compute_cdi(signal_div, coop_density, mem_use, explore);
        let h = hazard_rate(cdi, 0.05);
        let p_ext = extinction_probability(h, 1.0);
        let death_rate = deaths as f64 / (pop as f64 + deaths as f64 + 1e-9);

        writeln!(
            log.population,
            "{},{},{},{},{:.4},{}",
            tick,
            pop,
            births,
            deaths,
            avg_energy,
            lineage.len()
        )
        .unwrap();
        writeln!(
            log.cdi,
            "{},{:.5},{:.5},{:.5},{:.5},{:.6}",
            tick, signal_div, coop_density, mem_use, explore, cdi
        )
        .unwrap();
        writeln!(log.mutation, "{},{},{:.6}", tick, mut_count, dna_var).unwrap();
        writeln!(
            log.memory,
            "{},{:.5},{},{},{},{},{:.6},{:.6},{:.6},{}",
            tick,
            avg_stress_level,
            memory_archive.record_count(),
            archive_sample_attempts,
            archive_sample_successes,
            archive_influenced_births,
            lineage_diversity,
            top1_lineage_share,
            strategy_entropy,
            collapse_event_count
        )
        .unwrap();
        writeln!(
            log.boss,
            "{},{},{:.6},{:.6},{}",
            tick,
            10,
            if b_single_attempt == 0 {
                0.0
            } else {
                b_single_success as f64 / b_single_attempt as f64
            },
            if b_multi_attempt == 0 {
                0.0
            } else {
                b_multi_success as f64 / b_multi_attempt as f64
            },
            transfer
        )
        .unwrap();
        writeln!(
            log.extinction,
            "{},{:.6},{:.6},{:.6},{:.6},{}",
            tick, death_rate, cdi, h, p_ext, extinction_events
        )
        .unwrap();

        total_births += births as u64;
        total_deaths += deaths as u64;
        total_mut += mut_count as u64;

        let drift = if cfg.pressure > 1.2 {
            "high_pressure_drift"
        } else {
            "low_pressure_drift"
        };
        if tick % 100 == 0 && archive.strategy_patterns.len() < 200 {
            archive.strategy_patterns.push(drift.to_string());
        }
        let _ = ledger.total_input() - ledger.total_cost();
    }

    let multi = if b_multi_attempt == 0 {
        0.0
    } else {
        b_multi_success as f64 / b_multi_attempt as f64
    };
    let single = if b_single_attempt == 0 {
        0.0
    } else {
        b_single_success as f64 / b_single_attempt as f64
    };
    let final_energy = if cells.is_empty() {
        0.0
    } else {
        cells.iter().map(|c| c.energy).sum::<f64>() / cells.len() as f64
    };

    // mutation history dump per universe
    let mut f = std::fs::File::create(format!(
        "{}/u{}/mutation_history.csv",
        base_runs, cfg.universe_id
    ))
    .unwrap();
    writeln!(f, "tick,lineage_id,parameter,delta").unwrap();
    for e in mutation_history.iter().take(50_000) {
        writeln!(f, "{},{},{},{}", e.tick, e.lineage_id, e.parameter, e.delta).unwrap();
    }

    RunSummary {
        universe_id: cfg.universe_id,
        seed: cfg.seed,
        births: total_births,
        deaths: total_deaths,
        mutation_count: total_mut,
        single_boss_success: single,
        multi_boss_success: multi,
        adaptation_gain: final_energy - baseline_energy,
    }
}

fn resolve_boss(
    b: &Boss,
    cells: &mut [Cell],
    ledger: &mut EnergyLedger,
    transfer: &mut u32,
    s_attempt: &mut u64,
    s_success: &mut u64,
    m_attempt: &mut u64,
    m_success: &mut u64,
    cooperation_multiplier: f64,
) {
    let idx: Vec<usize> = (0..cells.len())
        .filter(|i| manhattan(cells[*i].position, b.pos) <= 4)
        .collect();
    if idx.is_empty() {
        return;
    }
    let n = idx.len();
    let mean_phase = idx
        .iter()
        .map(|i| cells[*i].signal_state.phase)
        .sum::<f64>()
        / n as f64;
    let synchrony = idx
        .iter()
        .map(|i| 1.0 - (cells[*i].signal_state.phase - mean_phase).abs())
        .sum::<f64>()
        / n as f64;
    let signal = idx
        .iter()
        .map(|i| cells[*i].signal_state.investment)
        .sum::<f64>()
        / n as f64;
    let success = cooperation_success(
        n,
        synchrony,
        signal,
        b.difficulty.min_attackers,
        b.difficulty.synchrony_threshold,
        b.difficulty.signal_threshold,
    );
    if n == 1 {
        *s_attempt += 1;
    } else {
        *m_attempt += 1;
    }
    for i in &idx {
        cells[*i].cooperation_state.recent_partner_count = n.saturating_sub(1);
        cells[*i].cooperation_state.synchrony_score = synchrony;
    }
    if success {
        for i in &idx {
            cells[*i].energy += b.difficulty.reward / n as f64;
        }
        ledger.input_boss_reward += b.difficulty.reward;
        if n == 1 {
            *s_success += 1;
        } else {
            *m_success += 1;
            *transfer += 1;
        }
    } else {
        for i in &idx {
            cells[*i].energy -=
                (b.difficulty.damage / n as f64) * (1.0 - cells[*i].dna.defense * 0.3);
        }
    }
}

fn signal_diversity(cells: &[Cell]) -> f64 {
    if cells.is_empty() {
        return 0.0;
    }
    let mean = cells.iter().map(|c| c.signal_state.phase).sum::<f64>() / cells.len() as f64;
    let var = cells
        .iter()
        .map(|c| (c.signal_state.phase - mean).powi(2))
        .sum::<f64>()
        / cells.len() as f64;
    var.sqrt().clamp(0.0, 1.0)
}

fn variance(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64
}
