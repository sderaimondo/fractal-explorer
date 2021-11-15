use crate::{
    color_scheme::{grayscale::Grayscale, ColorScheme},
    fractal::{checkerboard::Checkerboard, Fractal},
    render::render_fractal,
};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

pub struct App {
    fractal: Box<dyn Fractal>,
    color_scheme: Box<dyn ColorScheme>,
    center: (f32, f32),
    zoom: f32,
}

impl App {
    pub fn on_left_click_drag(&mut self, delta: (f32, f32)) {
        self.center.0 -= delta.0 / self.zoom;
        self.center.1 -= delta.1 / self.zoom;
    }

    pub fn on_scroll(&mut self, delta_y: f32) {
        self.zoom *= 1.2f32.powf(delta_y);
    }

    pub fn draw(&mut self, onto: &mut [u32], size: (usize, usize)) {
        render_fractal(
            &*self.fractal,
            &*self.color_scheme,
            onto,
            size,
            self.center,
            self.zoom,
        );
    }

    pub fn new() -> Self {
        Self {
            color_scheme: Box::new(Grayscale),
            fractal: Box::new(Checkerboard),
            center: (0.0, 0.0),
            zoom: 100.0,
        }
    }

    pub fn run(mut self) {
        const WIDTH: usize = 640;
        const HEIGHT: usize = WIDTH * 9 / 16;
        const SIZE: (usize, usize) = (WIDTH, HEIGHT);

        let mut buffer = vec![0u32; WIDTH * HEIGHT];
        let mut window =
            Window::new("Fractal Explorer", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
        let mut last_mouse_pos = None;

        self.draw(&mut buffer, SIZE);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        while window.is_open() && !window.is_key_down(Key::Escape) {
            if window.get_mouse_down(MouseButton::Left) {
                let new_mouse_pos = window.get_mouse_pos(MouseMode::Pass);
                if let (Some((x1, y1)), Some((x2, y2))) = (last_mouse_pos, new_mouse_pos) {
                    let (dx, dy) = (x2 - x1, y2 - y1);
                    self.on_left_click_drag((dx, dy));
                    self.draw(&mut buffer, SIZE);
                    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
                    last_mouse_pos = new_mouse_pos;
                    continue;
                }
                last_mouse_pos = new_mouse_pos;
            } else {
                last_mouse_pos = None;
            }

            if let Some((_, dy)) = window.get_scroll_wheel() {
                self.on_scroll(dy);
                self.draw(&mut buffer, SIZE);
                window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
                continue;
            }
            window.update();
        }
    }
}
