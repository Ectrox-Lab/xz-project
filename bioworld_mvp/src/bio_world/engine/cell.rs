use crate::bio_world::memory::{cell_memory::CellMemory, lineage_memory::LineageMemory};

use super::dna::Dna;

#[derive(Clone, Debug)]
pub struct CooperationState {
    pub recent_partner_count: usize,
    pub synchrony_score: f64,
}

#[derive(Clone, Debug)]
pub struct SignalState {
    pub phase: f64,
    pub investment: f64,
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub id: u64,
    pub position: (i32, i32, i32),
    pub energy: f64,
    pub age: u32,
    pub alive: bool,
    pub dna: Dna,
    pub cell_memory: CellMemory,
    pub cooperation_state: CooperationState,
    pub signal_state: SignalState,
    pub lineage_id: u64,
    pub lineage_memory: LineageMemory,
    pub archive_samples_taken: u32,
}
