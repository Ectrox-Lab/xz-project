use std::fs::{create_dir_all, File};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::bio_world::akashic::akashic_archive::AkashicArchive;
use crate::bio_world::engine::world::{run_universe, Config, RunSummary};
use crate::bio_world::experiments::cross_seed::summarize_cross_seed;
use crate::bio_world::output::json_export::write_akashic;

pub fn run_batch(base_runs: &str, base_akashic: &str, ticks: u32, universe_count: usize, pressure: f64, akashic_on: bool) -> Vec<RunSummary> {
    create_dir_all(base_runs).unwrap();
    let base_runs_owned = base_runs.to_string();
    let archive = Arc::new(Mutex::new(AkashicArchive::default()));
    let summaries = Arc::new(Mutex::new(Vec::<RunSummary>::new()));

    let mut handles = vec![];
    for u in 0..universe_count {
        let ar = archive.clone();
        let sm = summaries.clone();
        let br = base_runs_owned.clone();
        handles.push(thread::spawn(move || {
            let mut guard = ar.lock().unwrap();
            let cfg = Config { universe_id: u, seed: (u as u64) * 97 + 11, ticks, pressure, akashic_on };
            let sum = run_universe(&cfg, &mut guard, &br);
            drop(guard);
            sm.lock().unwrap().push(sum);
        }));
    }
    for h in handles { h.join().unwrap(); }

    let out = summaries.lock().unwrap().clone();
    let archive_final = archive.lock().unwrap().clone();
    write_akashic(base_akashic, &archive_final);

    let (mean_adapt, mean_multi, hazard_ratio) = summarize_cross_seed(&out);
    let mut f = File::create(format!("{}/cross_seed_summary.csv", base_runs)).unwrap();
    writeln!(f, "universes,mean_adaptation_gain,mean_multi_boss_success,hazard_ratio").unwrap();
    writeln!(f, "{},{:.6},{:.6},{:.6}", universe_count, mean_adapt, mean_multi, hazard_ratio).unwrap();

    out
}
