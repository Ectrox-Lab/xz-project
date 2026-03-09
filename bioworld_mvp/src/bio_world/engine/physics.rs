pub const GRID_X: i32 = 50;
pub const GRID_Y: i32 = 50;
pub const GRID_Z: i32 = 16;

pub fn wrap(v: i32, m: i32) -> i32 { ((v % m) + m) % m }

pub fn manhattan(a: (i32, i32, i32), b: (i32, i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}
