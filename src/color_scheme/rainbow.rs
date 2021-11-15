use super::ColorScheme;

pub struct Rainbow;

impl ColorScheme for Rainbow {
    // approximate a smooth transition though the colors of the rainbow
    fn get_color(&self, hue: u32) -> u32 {
        if hue == 63 {
            return 0x000000;
        }
        let offset: u32 = hue % 16 * 16;
        match hue / 16 {
            0 => 0xFF0000 | offset << 8,        // Red to Yellow
            1 => 0x00FF00 | 255 - offset << 16, // Yellow to Green
            2 => 0x00FF00 | offset,             // Green to Cyan
            3 => 0x0000FF | 255 - offset << 8,  // Cyan to Blue
            _ => unreachable!(),
        }
        //match hue / 11 {
        //    0 => 0x0000FF | offset << 16,       // Blue to Magenta
        //    1 => 0xFF0000 | 255 - offset,       // Magenta to Red
        //    2 => 0xFF0000 | offset << 8,        // Red to Yellow
        //    3 => 0x00FF00 | 255 - offset << 16, // Yellow to Green
        //    4 => 0x00FF00 | offset,             // Green to Cyan
        //    5 => 0x0000FF | 255 - offset << 8,  // Cyan to Blue
        //    _ => unreachable!(),
        //}
        //match hue {
        //    0..=10  => 0xFF0000 | offset << 8,         // Red to Yellow
        //    11..=20 => 0x00FF00 | 255 - offset << 16, // Yellow to Green
        //    21..=31 => 0x00FF00 | offset,             // Green to Cyan
        //    32..=42 => 0x0000FF | 255 - offset << 8,  // Cyan to Blue
        //    43..=52 => 0x0000FF | offset << 16,       // Blue to Magenta
        //    53..=63 => 0xFF0000 | 255 - offset,       // Magenta to Red
        //    _ => unreachable!(),
        //}
    }
}
