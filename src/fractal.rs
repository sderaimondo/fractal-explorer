pub mod checkerboard;
pub mod mandelbrot;

pub trait Fractal {
    fn render(&self, x: f32, y: f32) -> u32;
}
