use std::collections::HashMap;

use super::constants::{MAX_CELL_MEMORY_WINDOW, MEMORY_DECAY_FACTOR};

#[derive(Clone, Debug, PartialEq)]
pub struct CellMemory {
    pub recent_energy_history: Vec<f32>,
    pub recent_signal_history: Vec<f32>,
    pub stress_level: f32,
    pub experience_window: Vec<String>,
    pub neighbor_trust: HashMap<u64, f32>,
}

impl Default for CellMemory {
    fn default() -> Self {
        Self {
            recent_energy_history: Vec::new(),
            recent_signal_history: Vec::new(),
            stress_level: 0.0,
            experience_window: Vec::new(),
            neighbor_trust: HashMap::new(),
        }
    }
}

impl CellMemory {
    pub fn record_experience(&mut self, event: impl Into<String>) {
        self.experience_window.push(event.into());
        while self.experience_window.len() > MAX_CELL_MEMORY_WINDOW {
            self.experience_window.remove(0);
        }
    }

    pub fn record_energy(&mut self, energy: f32) {
        self.recent_energy_history.push(energy);
        while self.recent_energy_history.len() > MAX_CELL_MEMORY_WINDOW {
            self.recent_energy_history.remove(0);
        }
    }

    pub fn record_signal(&mut self, phase: f32) {
        self.recent_signal_history.push(phase);
        while self.recent_signal_history.len() > MAX_CELL_MEMORY_WINDOW {
            self.recent_signal_history.remove(0);
        }
    }

    pub fn update_trust(&mut self, agent_id: u64, outcome: f32) {
        let previous = *self.neighbor_trust.get(&agent_id).unwrap_or(&0.5);
        let blended = (previous * 0.7) + (outcome.clamp(0.0, 1.0) * 0.3);
        self.neighbor_trust.insert(agent_id, blended);
    }

    pub fn decay(&mut self) {
        self.stress_level *= MEMORY_DECAY_FACTOR;
        for trust in self.neighbor_trust.values_mut() {
            *trust *= MEMORY_DECAY_FACTOR;
        }
    }

    pub fn utilization(&self) -> f32 {
        self.recent_energy_history.len() as f32 / MAX_CELL_MEMORY_WINDOW as f32
    }
}

#[cfg(test)]
mod tests {
    use super::CellMemory;

    #[test]
    fn test_rolling_window() {
        let mut mem = CellMemory::default();
        for i in 0..150 {
            mem.record_energy(i as f32);
        }
        assert_eq!(mem.recent_energy_history.len(), 100);
    }

    #[test]
    fn test_trust_decay() {
        let mut mem = CellMemory::default();
        mem.update_trust(1, 0.8);
        let before = mem.neighbor_trust[&1];
        mem.decay();
        assert!(mem.neighbor_trust[&1] < before);
    }
}
