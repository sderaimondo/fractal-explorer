use app::App;

pub mod app;
pub mod color_scheme;
pub mod fractal;
pub mod render;

fn main() {
    App::new().run()
}
