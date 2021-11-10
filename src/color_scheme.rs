pub trait ColorScheme {
    fn get_color(&self, intensity: f32) -> u32;
}

pub struct Grayscale;
impl ColorScheme for Grayscale {
    fn get_color(&self, intensity: f32) -> u32 {
        let value: u32 = intensity.clamp(0.0, 255.0) as u32;
        value << 16 | value << 8 | value
    }
}
