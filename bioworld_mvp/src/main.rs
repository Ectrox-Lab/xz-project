mod bio_world;

use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

use bio_world::experiments::experiment_runner::run_batch;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ticks = arg_u32(&args, "--ticks").unwrap_or(10_000);
    let universes = arg_usize(&args, "--universes").unwrap_or(16); // default smaller; protocol target 128

    fs::create_dir_all("runs").unwrap();
    fs::create_dir_all("akashic").unwrap();

    // A - CTRL (baseline)
    let a = run_batch("runs/a_survival", "akashic/a", ticks, universes, 1.0, false, false, 1.0);
    // B - Evolution pressure
    let b = run_batch("runs/b_evolution", "akashic/b", ticks / 2, universes, 1.1, false, false, 1.0);
    // C low/high - P1-C: Boss pressure
    let c_low = run_batch("runs/c_pressure_low", "akashic/c_low", ticks / 2, universes, 0.8, false, false, 1.0);
    let c_high = run_batch("runs/c_pressure_high", "akashic/c_high", ticks / 2, universes, 1.5, false, false, 1.0);
    // D - Cooperation
    let d = run_batch("runs/d_cooperation", "akashic/d", ticks / 2, universes, 1.2, false, false, 1.0);
    // E off/on
    let e_off = run_batch("runs/e_akashic_off", "akashic/e_off", ticks / 2, universes, 1.15, false, false, 1.0);
    let e_on = run_batch("runs/e_akashic_on", "akashic/e_on", ticks / 2, universes, 1.15, true, false, 1.0);

    write_experiment_summary(&[(&"A", &a), (&"B", &b), (&"C_LOW", &c_low), (&"C_HIGH", &c_high), (&"D", &d), (&"E_OFF", &e_off), (&"E_ON", &e_on)]);
    build_top_level_metric_csvs();
}

fn arg_u32(args: &[String], key: &str) -> Option<u32> {
    args.iter().position(|x| x == key).and_then(|i| args.get(i + 1)).and_then(|v| v.parse().ok())
}
fn arg_usize(args: &[String], key: &str) -> Option<usize> {
    args.iter().position(|x| x == key).and_then(|i| args.get(i + 1)).and_then(|v| v.parse().ok())
}

fn write_experiment_summary(data: &[(&str, &Vec<bio_world::engine::world::RunSummary>)]) {
    let mut f = File::create("runs/summary.json").unwrap();
    writeln!(f, "[").unwrap();
    for (idx, (name, rows)) in data.iter().enumerate() {
        let births: u64 = rows.iter().map(|r| r.births).sum();
        let deaths: u64 = rows.iter().map(|r| r.deaths).sum();
        let mutc: u64 = rows.iter().map(|r| r.mutation_count).sum();
        let s = rows.iter().map(|r| r.single_boss_success).sum::<f64>() / rows.len().max(1) as f64;
        let m = rows.iter().map(|r| r.multi_boss_success).sum::<f64>() / rows.len().max(1) as f64;
        let ag = rows.iter().map(|r| r.adaptation_gain).sum::<f64>() / rows.len().max(1) as f64;
        let line = format!("  {{\"experiment\":\"{}\",\"universes\":{},\"births\":{},\"deaths\":{},\"mutation_count\":{},\"single_boss_success\":{:.6},\"multi_boss_success\":{:.6},\"adaptation_gain\":{:.6}}}", name, rows.len(), births, deaths, mutc, s, m, ag);
        if idx + 1 == data.len() { writeln!(f, "{}", line).unwrap(); } else { writeln!(f, "{},", line).unwrap(); }
    }
    writeln!(f, "]").unwrap();
}

fn build_top_level_metric_csvs() {
    // For protocol compatibility, expose a top-level set from the A experiment universe 0.
    let src = "runs/a_survival/u0";
    let mapping = [
        ("population.csv", "population.csv"),
        ("cdi.csv", "cdi.csv"),
        ("mutation.csv", "mutation.csv"),
        ("boss.csv", "boss.csv"),
        ("extinction.csv", "extinction.csv"),
    ];
    for (from, to) in mapping {
        let in_path = format!("{}/{}", src, from);
        let out_path = format!("runs/{}", to);
        if let Ok(file) = File::open(&in_path) {
            let mut out = File::create(out_path).unwrap();
            for line in BufReader::new(file).lines() {
                writeln!(out, "{}", line.unwrap()).unwrap();
            }
        }
    }
}
