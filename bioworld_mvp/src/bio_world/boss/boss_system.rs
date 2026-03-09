use crate::bio_world::boss::difficulty_curve::{difficulty_for_level, BossDifficulty};

#[derive(Clone, Debug)]
pub struct Boss {
    pub id: usize,
    pub pos: (i32, i32, i32),
    pub difficulty: BossDifficulty,
}

pub fn build_bosses() -> Vec<Boss> {
    (1..=10)
        .enumerate()
        .map(|(i, l)| Boss {
            id: i,
            pos: ((5 * i as i32 + 3) % 50, (7 * i as i32 + 11) % 50, (2 * i as i32 + 1) % 16),
            difficulty: difficulty_for_level(l as u8),
        })
        .collect()
}
