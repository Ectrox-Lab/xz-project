use super::dna::Dna;

#[derive(Clone, Debug)]
pub struct Memory {
    pub local_energy_trace: Vec<f64>,
    pub local_signal_trace: Vec<f64>,
}

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
    pub memory: Memory,
    pub cooperation_state: CooperationState,
    pub signal_state: SignalState,
    pub lineage_id: u64,
}
