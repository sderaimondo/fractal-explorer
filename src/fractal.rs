pub mod checkerboard;

pub trait Fractal {
    fn render(&self, x: f32, y: f32) -> f32;
}
