pub mod grayscale;

pub trait ColorScheme {
    fn get_color(&self, intensity: f32) -> u32;
}
