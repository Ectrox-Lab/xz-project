mod bio_world;

use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

use bio_world::engine::world::SentinelMode;
use bio_world::experiments::experiment_runner::{run_batch, BatchOptions};

fn main() {
    let args: Vec<String> = env::args().collect();
    let ticks = arg_u32(&args, "--ticks").unwrap_or(10_000);
    let universes = arg_usize(&args, "--universes").unwrap_or(16);

    fs::create_dir_all("runs").unwrap();
    fs::create_dir_all("akashic").unwrap();

    if let Some(mode_name) = arg_string(&args, "--sentinel-mode") {
        let mode = parse_sentinel_mode(&mode_name);
        run_single_sentinel(mode, ticks, universes);
        return;
    }

    if flag(&args, "--run-sentinel-suite") {
        run_sentinel_suite(ticks, universes);
        return;
    }

    let a = run_batch(
        "runs/a_survival",
        "akashic/a",
        &BatchOptions {
            ticks,
            universe_count: universes,
            pressure: 1.0,
            akashic_on: false,
            sentinel_mode: SentinelMode::BaselineFull,
        },
    );
    let b = run_batch(
        "runs/b_evolution",
        "akashic/b",
        &BatchOptions {
            ticks: ticks / 2,
            universe_count: universes,
            pressure: 1.1,
            akashic_on: false,
            sentinel_mode: SentinelMode::BaselineFull,
        },
    );
    let c_low = run_batch(
        "runs/c_pressure_low",
        "akashic/c_low",
        &BatchOptions {
            ticks: ticks / 2,
            universe_count: universes,
            pressure: 0.8,
            akashic_on: false,
            sentinel_mode: SentinelMode::BaselineFull,
        },
    );
    let c_high = run_batch(
        "runs/c_pressure_high",
        "akashic/c_high",
        &BatchOptions {
            ticks: ticks / 2,
            universe_count: universes,
            pressure: 1.5,
            akashic_on: false,
            sentinel_mode: SentinelMode::BaselineFull,
        },
    );
    let d = run_batch(
        "runs/d_cooperation",
        "akashic/d",
        &BatchOptions {
            ticks: ticks / 2,
            universe_count: universes,
            pressure: 1.2,
            akashic_on: false,
            sentinel_mode: SentinelMode::BaselineFull,
        },
    );
    let e_off = run_batch(
        "runs/e_akashic_off",
        "akashic/e_off",
        &BatchOptions {
            ticks: ticks / 2,
            universe_count: universes,
            pressure: 1.15,
            akashic_on: false,
            sentinel_mode: SentinelMode::BaselineFull,
        },
    );
    let e_on = run_batch(
        "runs/e_akashic_on",
        "akashic/e_on",
        &BatchOptions {
            ticks: ticks / 2,
            universe_count: universes,
            pressure: 1.15,
            akashic_on: true,
            sentinel_mode: SentinelMode::BaselineFull,
        },
    );

    write_experiment_summary(&[
        (&"A", &a),
        (&"B", &b),
        (&"C_LOW", &c_low),
        (&"C_HIGH", &c_high),
        (&"D", &d),
        (&"E_OFF", &e_off),
        (&"E_ON", &e_on),
    ]);
    build_top_level_metric_csvs();
}

fn run_single_sentinel(mode: SentinelMode, ticks: u32, universes: usize) {
    let label = mode.as_str();
    let runs_dir = format!("runs/sentinel/{}", label);
    let akashic_dir = format!("akashic/sentinel/{}", label);
    let _ = run_batch(
        &runs_dir,
        &akashic_dir,
        &BatchOptions {
            ticks,
            universe_count: universes,
            pressure: 1.15,
            akashic_on: true,
            sentinel_mode: mode,
        },
    );
}

fn run_sentinel_suite(ticks: u32, universes: usize) {
    let modes = [
        SentinelMode::BaselineFull,
        SentinelMode::NoL1,
        SentinelMode::NoL2,
        SentinelMode::L3Off,
        SentinelMode::L3RealP001,
        SentinelMode::L3ShuffledP001,
        SentinelMode::L3OverpoweredDirect,
    ];
    for mode in modes {
        run_single_sentinel(mode, ticks, universes);
    }
}

fn parse_sentinel_mode(value: &str) -> SentinelMode {
    match value {
        "baseline_full" => SentinelMode::BaselineFull,
        "no_L1" => SentinelMode::NoL1,
        "no_L2" => SentinelMode::NoL2,
        "L3_off" => SentinelMode::L3Off,
        "L3_real_p001" => SentinelMode::L3RealP001,
        "L3_shuffled_p001" => SentinelMode::L3ShuffledP001,
        "L3_overpowered_direct" => SentinelMode::L3OverpoweredDirect,
        _ => panic!("unknown sentinel mode: {value}"),
    }
}

fn arg_u32(args: &[String], key: &str) -> Option<u32> {
    args.iter()
        .position(|x| x == key)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
}
fn arg_usize(args: &[String], key: &str) -> Option<usize> {
    args.iter()
        .position(|x| x == key)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
}
fn arg_string(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|x| x == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}
fn flag(args: &[String], key: &str) -> bool {
    args.iter().any(|x| x == key)
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
        if idx + 1 == data.len() {
            writeln!(f, "{}", line).unwrap();
        } else {
            writeln!(f, "{},", line).unwrap();
        }
    }
    writeln!(f, "]").unwrap();
}

fn build_top_level_metric_csvs() {
    let src = "runs/a_survival/u0";
    let mapping = [
        ("population.csv", "population.csv"),
        ("cdi.csv", "cdi.csv"),
        ("mutation.csv", "mutation.csv"),
        ("boss.csv", "boss.csv"),
        ("extinction.csv", "extinction.csv"),
        ("memory.csv", "memory.csv"),
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
