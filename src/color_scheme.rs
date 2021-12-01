pub mod grayscale;
pub mod rainbow;

pub trait ColorScheme {
    // intensity [0..63]
    fn get_color(&self, intensity: u32) -> u32;
}
