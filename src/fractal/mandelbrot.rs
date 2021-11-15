use super::Fractal;

pub struct Mandelbrot;

/*
 * Algorithm:
 *   https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set#Op
 *   timized_escape_time_algorithms
*/
impl Fractal for Mandelbrot {
    fn render(&self, x0: f32, y0: f32) -> u32 {
        let max_iteration: u32 = 63;
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut x2: f32 = 0.0;
        let mut y2: f32 = 0.0;
        for i in 0..max_iteration {
            if x2 + y2 >= 4.0 {
                return i;
            }
            y = 2.0 * x * y + y0;
            x = x2 - y2 + x0;
            x2 = x * x;
            y2 = y * y;
        }
        return 63;
    }
}
