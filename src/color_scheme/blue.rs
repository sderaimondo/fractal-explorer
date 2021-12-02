use super::ColorScheme;

pub struct Blue;

impl ColorScheme for Blue {

    fn get_color(&self, hue: u32) -> u32 {
        if hue == 63 {
            return 0x246B94;
        }
        let offset: u32 = hue % 16 * 16;
        match hue / 16 {
            0 => 0x88CCF1 | offset << 8,        
            1 => 0x3587A4 | 255 - offset << 16, 
            2 => 0x2D848A | offset,             
            3 => 0x2D898B | 255 - offset << 8,  
            _ => unreachable!(),
        }
    }
}
