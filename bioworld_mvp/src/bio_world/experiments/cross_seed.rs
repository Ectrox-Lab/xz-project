use crate::bio_world::engine::world::RunSummary;

pub fn summarize_cross_seed(items: &[RunSummary]) -> (f64, f64, f64) {
    if items.is_empty() { return (0.0, 0.0, 0.0); }
    let mean_adapt = items.iter().map(|s| s.adaptation_gain).sum::<f64>() / items.len() as f64;
    let mean_multi = items.iter().map(|s| s.multi_boss_success).sum::<f64>() / items.len() as f64;
    let hazard_ratio = if items.iter().map(|s| s.deaths).sum::<u64>() == 0 { 0.0 } else {
        items.iter().map(|s| s.deaths as f64 / (s.births.max(1) as f64)).sum::<f64>() / items.len() as f64
    };
    (mean_adapt, mean_multi, hazard_ratio)
}
