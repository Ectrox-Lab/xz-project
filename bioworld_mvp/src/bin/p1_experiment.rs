// P1 Causal Validation Experiment Runner
// 4 Groups: CTRL, P1-A (Memory KO), P1-B (Coop suppression), P1-C (Boss×1.5)

use std::env;
use std::fs;
use bioworld_mvp::bio_world::experiments::experiment_runner::run_batch;
use bioworld_mvp::bio_world::engine::world::RunSummary;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Parse arguments
    let group = arg_str(&args, "--group").unwrap_or_else(|| "CTRL".to_string());
    let seed_base = arg_u64(&args, "--seed").unwrap_or(100);
    let ticks = arg_u32(&args, "--ticks").unwrap_or(5000);
    let universes = arg_usize(&args, "--universes").unwrap_or(32);
    let output_dir = arg_str(&args, "--output-dir").unwrap_or_else(|| format!("p1_experiments/{}", group.to_lowercase()));
    
    fs::create_dir_all(&output_dir).unwrap();
    fs::create_dir_all(format!("{}/akashic", output_dir)).unwrap();
    
    println!("═══════════════════════════════════════════════════════════");
    println!("  P1 Causal Validation Experiment");
    println!("  Group: {}, Seeds: {}-{}, Generations: {}", 
        group, seed_base, seed_base + 2, ticks);
    println!("═══════════════════════════════════════════════════════════");
    
    // Configure parameters based on group
    let (pressure, disable_lineage_memory, cooperation_multiplier, group_name) = match group.as_str() {
        "CTRL" | "ctrl" => (1.0, false, 1.0, "CTRL"),
        "P1A" | "p1a" | "P1-A" | "p1-a" => (1.0, true, 1.0, "P1-A"),
        "P1B" | "p1b" | "P1-B" | "p1-b" => (1.0, false, 0.3, "P1-B"),
        "P1C" | "p1c" | "P1-C" | "p1-c" => (1.5, false, 1.0, "P1-C"),
        _ => {
            eprintln!("Unknown group: {}. Use CTRL, P1A, P1B, or P1C", group);
            std::process::exit(1);
        }
    };
    
    println!("▶ Parameters:");
    println!("    Pressure: {}", pressure);
    println!("    Memory KO: {}", disable_lineage_memory);
    println!("    Cooperation multiplier: {}", cooperation_multiplier);
    println!("");
    
    // Run 3 seeds for this group
    let mut all_summaries: Vec<(String, Vec<RunSummary>)> = Vec::new();
    
    for i in 0..3 {
        let seed = seed_base + i as u64;
        let run_dir = format!("{}/seed_{}", output_dir, seed);
        let akashic_dir = format!("{}/akashic/seed_{}", output_dir, seed);
        
        println!("▶ Running seed {} ({}/3)...", seed, i + 1);
        
        let summaries = run_batch(
            &run_dir,
            &akashic_dir,
            ticks,
            universes,
            pressure,
            false, // akashic_on
            disable_lineage_memory,
            cooperation_multiplier,
        );
        
        all_summaries.push((format!("{}_seed{}", group_name, seed), summaries));
        
        // Write individual seed summary
        let seed_summary = format!("{}/summary.json", run_dir);
        write_seed_summary(&seed_summary, seed, &all_summaries.last().unwrap().1);
        
        println!("  ✓ Complete: {} universes", all_summaries.last().unwrap().1.len());
    }
    
    // Write group summary
    let group_summary_path = format!("{}/group_summary.json", output_dir);
    write_group_summary(&group_summary_path, group_name, &all_summaries);
    
    println!("");
    println!("═══════════════════════════════════════════════════════════");
    println!("  P1 {} Complete!", group_name);
    println!("  Output: {}", output_dir);
    println!("═══════════════════════════════════════════════════════════");
}

fn arg_str(args: &[String], key: &str) -> Option<String> {
    args.iter().position(|x| x == key)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_u32(args: &[String], key: &str) -> Option<u32> {
    args.iter().position(|x| x == key)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
}

fn arg_u64(args: &[String], key: &str) -> Option<u64> {
    args.iter().position(|x| x == key)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
}

fn arg_usize(args: &[String], key: &str) -> Option<usize> {
    args.iter().position(|x| x == key)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
}

fn write_seed_summary(path: &str, seed: u64, summaries: &[RunSummary]) {
    use std::io::Write;
    let mut f = std::fs::File::create(path).unwrap();
    
    let births: u64 = summaries.iter().map(|r| r.births).sum();
    let deaths: u64 = summaries.iter().map(|r| r.deaths).sum();
    let mutations: u64 = summaries.iter().map(|r| r.mutation_count).sum();
    let avg_single: f64 = summaries.iter().map(|r| r.single_boss_success).sum::<f64>() / summaries.len().max(1) as f64;
    let avg_multi: f64 = summaries.iter().map(|r| r.multi_boss_success).sum::<f64>() / summaries.len().max(1) as f64;
    let avg_adapt: f64 = summaries.iter().map(|r| r.adaptation_gain).sum::<f64>() / summaries.len().max(1) as f64;
    
    writeln!(f, "{{").unwrap();
    writeln!(f, "  \"seed\": {},", seed).unwrap();
    writeln!(f, "  \"universes\": {},", summaries.len()).unwrap();
    writeln!(f, "  \"total_births\": {},", births).unwrap();
    writeln!(f, "  \"total_deaths\": {},", deaths).unwrap();
    writeln!(f, "  \"total_mutations\": {},", mutations).unwrap();
    writeln!(f, "  \"avg_single_boss_success\": {:.6},", avg_single).unwrap();
    writeln!(f, "  \"avg_multi_boss_success\": {:.6},", avg_multi).unwrap();
    writeln!(f, "  \"avg_adaptation_gain\": {:.6}", avg_adapt).unwrap();
    writeln!(f, "}}").unwrap();
}

fn write_group_summary(path: &str, group: &str, data: &[(String, Vec<RunSummary>)]) {
    use std::io::Write;
    let mut f = std::fs::File::create(path).unwrap();
    
    writeln!(f, "{{").unwrap();
    writeln!(f, "  \"group\": \"{}\",", group).unwrap();
    writeln!(f, "  \"seeds\": [").unwrap();
    
    for (idx, (name, summaries)) in data.iter().enumerate() {
        let births: u64 = summaries.iter().map(|r| r.births).sum();
        let deaths: u64 = summaries.iter().map(|r| r.deaths).sum();
        let mutations: u64 = summaries.iter().map(|r| r.mutation_count).sum();
        let avg_single: f64 = summaries.iter().map(|r| r.single_boss_success).sum::<f64>() / summaries.len().max(1) as f64;
        let avg_multi: f64 = summaries.iter().map(|r| r.multi_boss_success).sum::<f64>() / summaries.len().max(1) as f64;
        let avg_adapt: f64 = summaries.iter().map(|r| r.adaptation_gain).sum::<f64>() / summaries.len().max(1) as f64;
        
        writeln!(f, "    {{").unwrap();
        writeln!(f, "      \"name\": \"{}\",", name).unwrap();
        writeln!(f, "      \"universes\": {},", summaries.len()).unwrap();
        writeln!(f, "      \"total_births\": {},", births).unwrap();
        writeln!(f, "      \"total_deaths\": {},", deaths).unwrap();
        writeln!(f, "      \"total_mutations\": {},", mutations).unwrap();
        writeln!(f, "      \"avg_single_boss_success\": {:.6},", avg_single).unwrap();
        writeln!(f, "      \"avg_multi_boss_success\": {:.6},", avg_multi).unwrap();
        writeln!(f, "      \"avg_adaptation_gain\": {:.6}", avg_adapt).unwrap();
        if idx + 1 == data.len() {
            writeln!(f, "    }}").unwrap();
        } else {
            writeln!(f, "    }},").unwrap();
        }
    }
    
    writeln!(f, "  ]").unwrap();
    writeln!(f, "}}").unwrap();
}
