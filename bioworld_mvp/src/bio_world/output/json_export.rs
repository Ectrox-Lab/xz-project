use std::fs::{create_dir_all, File};
use std::io::Write;

use crate::bio_world::akashic::akashic_archive::AkashicArchive;

pub fn write_akashic(path: &str, archive: &AkashicArchive) {
    create_dir_all(path).unwrap();
    let mut f = File::create(format!("{}/akashic_archive.json", path)).unwrap();
    writeln!(f, "{{").unwrap();
    writeln!(f, "  \"elite_dna_count\": {},", archive.elite_dna.len()).unwrap();
    writeln!(f, "  \"death_records_count\": {},", archive.death_records.len()).unwrap();
    writeln!(f, "  \"strategy_patterns\": [{}]", archive.strategy_patterns.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(",")).unwrap();
    writeln!(f, "}}").unwrap();
}
