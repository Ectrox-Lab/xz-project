#[derive(Clone, Debug, Default)]
pub struct EnergyLedger {
    pub input_environment: f64,
    pub input_food: f64,
    pub input_boss_reward: f64,
    pub cost_movement: f64,
    pub cost_signal: f64,
    pub cost_memory: f64,
    pub cost_reproduction: f64,
}

impl EnergyLedger {
    pub fn total_input(&self) -> f64 {
        self.input_environment + self.input_food + self.input_boss_reward
    }
    pub fn total_cost(&self) -> f64 {
        self.cost_movement + self.cost_signal + self.cost_memory + self.cost_reproduction
    }
}
