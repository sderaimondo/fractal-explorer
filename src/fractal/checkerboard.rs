use super::Fractal;

pub struct Checkerboard;

impl Fractal for Checkerboard {
    fn render(&self, x: f32, y: f32) -> f32 {
        match ((x * 10.0) as i32).rem_euclid(2) == ((y * 10.0) as i32).rem_euclid(2) {
            true => 255.0,
            false => 0.0,
        }
    }
}
