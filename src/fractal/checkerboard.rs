use super::Fractal;

pub struct Checkerboard;

impl Fractal for Checkerboard {
    fn render(&self, x: f32, y: f32) -> u32 {
        match ((x * 10.0) as i32).rem_euclid(2) == ((y * 10.0) as i32).rem_euclid(2) {
            true => 63,
            false => 0,
        }
    }
}
