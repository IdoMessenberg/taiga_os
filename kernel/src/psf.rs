use crate::graphics::{PutPixel, Status};

///# Safety
/// todo:
unsafe trait GetChar {
    unsafe fn get_char_glyph_from_buffer(&self, char: char, position: u8) -> u8;
}
pub trait RenderChar{
    fn put_char<T: PutPixel>(&self, graphics: &T , x: &u32, y: &u32, char: char, forground_color: u32, background_color: u32) -> Result<(), Status>;
}

impl RenderChar for boot::efl::psf::FontInfo {
    fn put_char<T: PutPixel>(&self, graphics: &T, x: &u32, y: &u32, char: char, forground_color: u32, background_color: u32) -> Result<(), Status>{
        let mut glyph_char_scan_line_bitmap: u8 = unsafe{self.get_char_glyph_from_buffer(char, 0)};
        for py in *y..*y + self.char_size as u32 {
            for px in *x..*x + 8 {
                if glyph_char_scan_line_bitmap & 0b10000000 >> (px - x) != 0 {
                    match unsafe{graphics.put_pixel(px, py, forground_color )} {
                        Ok(_) => (),
                        Err(err) => return Err(err)
                    }
                }
                else {
                    match unsafe{graphics.put_pixel(px, py, background_color)} {
                        Ok(_) => (),
                        Err(err) => return Err(err)
                    }
                }
            }
            glyph_char_scan_line_bitmap = unsafe {self.get_char_glyph_from_buffer(char, (py - y + 1) as u8)};
        }
        Ok(())
    }
}
unsafe impl GetChar for boot::efl::psf::FontInfo {
    unsafe fn get_char_glyph_from_buffer(&self, char: char, position: u8) -> u8 { *((self.glyph_buffer_base_address + char as u64 * self.char_size as u64 + position as u64) as *const u8) }
}