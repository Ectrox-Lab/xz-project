pub fn compute_cdi(signal_diversity: f64, cooperation_density: f64, memory_usage: f64, exploration_rate: f64) -> f64 {
    signal_diversity.max(0.0) * cooperation_density.max(0.0) * memory_usage.max(0.0) * exploration_rate.max(0.0)
}
