use crate::bio_world::engine::cell::Cell;

pub fn survive(c: &Cell) -> bool {
    c.alive && c.energy > 0.0 && c.age < 5000
}
