use super::ColorScheme;

pub struct Purple;

impl ColorScheme for Purple {

    fn get_color(&self, hue: u32) -> u32 {
        if hue == 63 {
            return 0x9A48D0;
        }
        let offset: u32 = hue % 16 * 16;
        match hue / 16 {
            0 => 0x63458A | offset << 8,        
            1 => 0xB288C0 | 255 - offset << 16, 
            2 => 0x7E5A9B | offset,             
            3 => 0xE4B7E5 | 255 - offset << 8,  
            _ => unreachable!(),
        }
    }
}
