use crate::bio_world::engine::dna::Dna;

#[derive(Clone, Debug)]
pub struct EliteDna {
    pub source_universe: usize,
    pub generation: u32,
    pub lineage_id: u64,
    pub dna: Dna,
}

#[derive(Clone, Debug)]
pub struct DeathRecord {
    pub source_universe: usize,
    pub generation: u32,
    pub lineage_id: u64,
    pub reason: String,
}

#[derive(Clone, Debug, Default)]
pub struct AkashicArchive {
    pub elite_dna: Vec<EliteDna>,
    pub death_records: Vec<DeathRecord>,
    pub strategy_patterns: Vec<String>,
}
