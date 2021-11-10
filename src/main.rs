use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = WIDTH * 9 / 16;

fn main() {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut window =
        Window::new("Fractal Explorer", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
