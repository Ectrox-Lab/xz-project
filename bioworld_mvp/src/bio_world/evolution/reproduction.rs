use crate::bio_world::engine::cell::Cell;

pub fn can_reproduce(c: &Cell) -> bool {
    c.energy > 32.0 + c.dna.memory_capacity as f64 * 0.6
}
