#[derive(Clone, Debug)]
pub struct Dna {
    pub move_speed: f64,
    pub sensing_radius: i32,
    pub attack_power: f64,
    pub defense: f64,
    pub cooperation_willingness: f64,
    pub signal_strength: f64,
    pub signal_frequency: f64,
    pub memory_capacity: usize,
    pub learning_rate: f64,
    pub mutation_rate: f64,
}

impl Dna {
    pub fn as_vec(&self) -> [f64; 10] {
        [
            self.move_speed,
            self.sensing_radius as f64 / 8.0,
            self.attack_power,
            self.defense,
            self.cooperation_willingness,
            self.signal_strength,
            self.signal_frequency,
            self.memory_capacity as f64 / 16.0,
            self.learning_rate,
            self.mutation_rate,
        ]
    }
}

#[derive(Clone, Debug)]
pub struct MutationEvent {
    pub tick: u32,
    pub lineage_id: u64,
    pub parameter: &'static str,
    pub delta: f64,
}
