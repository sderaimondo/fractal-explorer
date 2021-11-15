use super::ColorScheme;

pub struct Grayscale;

impl ColorScheme for Grayscale {
    fn get_color(&self, intensity: u32) -> u32 {
        intensity << 18 | intensity << 10 | intensity << 2
    }
}
