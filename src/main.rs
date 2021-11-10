use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = WIDTH * 9 / 16;

fn main() {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut window =
        Window::new("Fractal Explorer", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let mut zoom = 100.0f32;
    let (mut cx, mut cy) = (0.5f32, 0.0f32);
    let mut last_mouse_pos = None;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.get_mouse_down(MouseButton::Left) {
            let new_mouse_pos = window.get_mouse_pos(MouseMode::Pass);
            if let (Some((x1, y1)), Some((x2, y2))) = (last_mouse_pos, new_mouse_pos) {
                cx -= (x2 - x1) / zoom;
                cy -= (y2 - y1) / zoom;
            }
            last_mouse_pos = new_mouse_pos;
        } else {
            last_mouse_pos = None;
        }

        if let Some((_, dy)) = window.get_scroll_wheel() {
            zoom *= 1.2f32.powf(dy);
        }

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let px = (x as f32 - WIDTH as f32 / 2.0) / zoom + cx;
                let py = (y as f32 - HEIGHT as f32 / 2.0) / zoom + cy;
                let color =
                    if ((px * 10.0) as i32).rem_euclid(2) == ((py * 10.0) as i32).rem_euclid(2) {
                        0xFFFFFF
                    } else {
                        0x000000
                    };
                buffer[y * WIDTH + x] = color;
            }
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
