use std::fs::{create_dir_all, File};
use std::io::Write;

pub struct CsvLoggers {
    pub population: File,
    pub cdi: File,
    pub mutation: File,
    pub boss: File,
    pub extinction: File,
    pub memory: File,
}

impl CsvLoggers {
    pub fn new(base: &str, u: usize) -> Self {
        let dir = format!("{}/u{}", base, u);
        create_dir_all(&dir).unwrap();
        let mut population = File::create(format!("{}/population.csv", dir)).unwrap();
        let mut cdi = File::create(format!("{}/cdi.csv", dir)).unwrap();
        let mut mutation = File::create(format!("{}/mutation.csv", dir)).unwrap();
        let mut boss = File::create(format!("{}/boss.csv", dir)).unwrap();
        let mut extinction = File::create(format!("{}/extinction.csv", dir)).unwrap();
        let mut memory = File::create(format!("{}/memory.csv", dir)).unwrap();
        writeln!(
            population,
            "tick,population,births,deaths,avg_energy,lineage_count"
        )
        .unwrap();
        writeln!(
            cdi,
            "tick,signal_diversity,cooperation_density,memory_usage,exploration_rate,cdi"
        )
        .unwrap();
        writeln!(mutation, "tick,mutation_count,dna_variance").unwrap();
        writeln!(
            boss,
            "tick,level,single_success,multi_success,energy_transfer_count"
        )
        .unwrap();
        writeln!(
            extinction,
            "tick,death_rate,cdi,hazard_rate,extinction_probability,extinction_events"
        )
        .unwrap();
        writeln!(
            memory,
            "tick,avg_stress_level,archive_record_count,archive_sample_attempts,archive_sample_successes,archive_influenced_births,lineage_diversity,top1_lineage_share,strategy_entropy,collapse_event_count"
        )
        .unwrap();
        Self {
            population,
            cdi,
            mutation,
            boss,
            extinction,
            memory,
        }
    }
}
