use crate::{
    color_scheme::{grayscale::Grayscale, rainbow::Rainbow, ColorScheme},
    fractal::{checkerboard::Checkerboard, mandelbrot::Mandelbrot, Fractal},
    render::render_fractal,
};
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use std::collections::vec_deque::VecDeque;

pub struct App {
    color_schemes: VecDeque<Box<dyn ColorScheme>>,
    fractals: VecDeque<Box<dyn Fractal>>,
    center: (f32, f32),
    zoom: f32,
    redraw: bool,
}

impl App {
    pub fn on_left_click_drag(&mut self, delta: (f32, f32)) {
        self.center.0 -= delta.0 / self.zoom;
        self.center.1 -= delta.1 / self.zoom;
        self.redraw = true;
    }

    pub fn on_scroll(&mut self, delta_y: f32) {
        self.zoom *= 1.2f32.powf(delta_y);
        self.redraw = true;
    }

    //pub fn rotate<T>(&mut self, mut ring: &VecDeque<T>) {
    //    ring.shift_left(1);
    //    self.redraw = true;
    //}

    pub fn draw(&mut self, onto: &mut [u32], size: (usize, usize)) {
        let color_scheme = self.color_schemes.front();
        let fractal = self.fractals.front();
        if let (Some(cs), Some(f)) = (color_scheme, fractal) {
            render_fractal(
                &**f,
                &**cs,
                onto,
                size,
                self.center,
                self.zoom,
            );
        }
    }

    pub fn new() -> Self {
        let mut app = Self {
            color_schemes: VecDeque::new(),
            fractals: VecDeque::new(),
            center: (0.0, 0.0),
            zoom: 100.0,
            redraw: true,
        };
        app.color_schemes.push_back(Box::new(Rainbow));
        app.color_schemes.push_back(Box::new(Grayscale));
        app.fractals.push_back(Box::new(Mandelbrot));
        app.fractals.push_back(Box::new(Checkerboard));
        app
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
                }
                last_mouse_pos = new_mouse_pos;
            } else {
                last_mouse_pos = None;
            }
            if window.is_key_pressed(Key::Space, KeyRepeat::No) {
                self.color_schemes.rotate_left(1);
                self.redraw = true;
            }
            if window.is_key_pressed(Key::Tab, KeyRepeat::No) {
                self.fractals.rotate_left(1);
                self.redraw = true;
            }
            if let Some((_, dy)) = window.get_scroll_wheel() {
                self.on_scroll(dy);
            }
            if self.redraw {
                self.redraw = false;
                self.draw(&mut buffer, SIZE);
                window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
            } else {
                window.update();
            }
        }
    }
}
