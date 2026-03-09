use std::fs::{create_dir_all, File};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::bio_world::akashic::akashic_archive::AkashicArchive;
use crate::bio_world::engine::world::{run_universe, Config, RunSummary, SentinelMode};
use crate::bio_world::experiments::cross_seed::summarize_cross_seed;
use crate::bio_world::output::json_export::write_akashic;

#[derive(Clone, Debug)]
pub struct BatchOptions {
    pub ticks: u32,
    pub universe_count: usize,
    pub pressure: f64,
    pub akashic_on: bool,
    pub sentinel_mode: SentinelMode,
}

pub fn run_batch(base_runs: &str, base_akashic: &str, options: &BatchOptions) -> Vec<RunSummary> {
    create_dir_all(base_runs).unwrap();
    let base_runs_owned = base_runs.to_string();
    let archive = Arc::new(Mutex::new(AkashicArchive::default()));
    let summaries = Arc::new(Mutex::new(Vec::<RunSummary>::new()));

    let mut handles = vec![];
    for u in 0..options.universe_count {
        let ar = archive.clone();
        let sm = summaries.clone();
        let br = base_runs_owned.clone();
        let cfg_options = options.clone();
        handles.push(thread::spawn(move || {
            let mut guard = ar.lock().unwrap();
            let cfg = Config {
                universe_id: u,
                seed: (u as u64) * 97 + 11,
                ticks: cfg_options.ticks,
                pressure: cfg_options.pressure,
                akashic_on: cfg_options.akashic_on,
                sentinel_mode: cfg_options.sentinel_mode,
            };
            let sum = run_universe(&cfg, &mut guard, &br);
            drop(guard);
            sm.lock().unwrap().push(sum);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }

    let out = summaries.lock().unwrap().clone();
    let archive_final = archive.lock().unwrap().clone();
    // Sentinel runs do not require full archive persistence for analysis closure.
    if !base_akashic.contains("sentinel") {
        write_akashic(base_akashic, &archive_final);
    }

    let (mean_adapt, mean_multi, hazard_ratio) = summarize_cross_seed(&out);
    let mut f = File::create(format!("{}/cross_seed_summary.csv", base_runs)).unwrap();
    writeln!(
        f,
        "universes,mean_adaptation_gain,mean_multi_boss_success,hazard_ratio"
    )
    .unwrap();
    writeln!(
        f,
        "{},{:.6},{:.6},{:.6}",
        options.universe_count, mean_adapt, mean_multi, hazard_ratio
    )
    .unwrap();

    let mut per_run = File::create(format!("{}/per_run.csv", base_runs)).unwrap();
    writeln!(
        per_run,
        "universe_id,seed,sentinel_mode,births,deaths,mutation_count,archive_sample_attempts,archive_sample_successes,archive_influenced_births,lineage_diversity,top1_lineage_share,strategy_entropy,collapse_event_count"
    )
    .unwrap();
    for row in &out {
        writeln!(
            per_run,
            "{},{},{},{},{},{},{},{},{},{:.6},{:.6},{:.6},{}",
            row.universe_id,
            row.seed,
            options.sentinel_mode.as_str(),
            row.births,
            row.deaths,
            row.mutation_count,
            row.archive_sample_attempts,
            row.archive_sample_successes,
            row.archive_influenced_births,
            row.lineage_diversity,
            row.top1_lineage_share,
            row.strategy_entropy,
            row.collapse_event_count
        )
        .unwrap();
    }

    out
}
