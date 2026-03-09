#[derive(Clone, Debug)]
pub struct BossDifficulty {
    pub level: u8,
    pub min_attackers: usize,
    pub synchrony_threshold: f64,
    pub signal_threshold: f64,
    pub damage: f64,
    pub reward: f64,
}

pub fn difficulty_for_level(level: u8) -> BossDifficulty {
    let l = level.clamp(1, 10) as f64;
    BossDifficulty {
        level,
        min_attackers: (1.0 + l / 1.8).round() as usize,
        synchrony_threshold: (0.35 + l * 0.045).clamp(0.35, 0.92),
        signal_threshold: (0.12 + l * 0.05).clamp(0.12, 0.85),
        damage: 5.0 + 2.5 * l,
        reward: 12.0 + 6.0 * l,
    }
}
