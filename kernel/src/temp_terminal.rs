
use core::fmt::Write;
use crate::temp_graphics::FrameBufferTrait;
pub struct Terminal{
    pub frame_buffer:boot::GraphicsInfo,
    font: boot::psf::FontInfo,
    theme: boot::TerminalTheme,
    curs_pos_x: u16,
    curs_pos_y: u16,
}
impl Terminal {
    const SIDE_BUFFER: u16 = 4;
    const TOP_BUFFER: u16 = Self::SIDE_BUFFER / 2;
    pub fn new(boot_info: &boot::_Info_) -> Self {
        Self { frame_buffer: boot_info.graphics, font: boot_info.font, theme: boot_info.theme, curs_pos_x: 0, curs_pos_y: 0 }
    }
    pub fn put_char(&mut self, char: char){
        for y in self.curs_pos_y * self.font.char_size as u16 + Self::TOP_BUFFER..(self.curs_pos_y + 1) * self.font.char_size as u16 + Self::TOP_BUFFER {
            let glyph_char_scan_line_bitmap: u8 = self.font.get_char_glyph_from_buffer(char, (y - (self.curs_pos_y * self.font.char_size as u16) - 1) as u8);
            for x in self.curs_pos_x * boot::psf::FontInfo::CHAR_WIDTH as u16 + Self::SIDE_BUFFER..(self.curs_pos_x + 1) * boot::psf::FontInfo::CHAR_WIDTH as u16 + Self::SIDE_BUFFER {
                match glyph_char_scan_line_bitmap & 0b10000000 >> (x - self.curs_pos_x * boot::psf::FontInfo::CHAR_WIDTH as u16 - 4) != 0 {
                    true => unsafe {
                        self.frame_buffer.put_pixel(x as u32, y as u32, self.theme.white);
                    },
                    false => unsafe {
                        self.frame_buffer.put_pixel(x as u32, y as u32, self.theme.black);
                    }
                }
            }
        }
        if self.curs_pos_x as u32 + 1 >= (self.frame_buffer.horizontal_resolution/boot::psf::FontInfo::CHAR_WIDTH as u32) - 1{
            self.curs_pos_x = 0;
            self.curs_pos_y += 1;
            if self.curs_pos_y as u32 >=  (self.frame_buffer.vertical_resolution/ self.font.char_size as u32) - 1 {
                for y in 0..(self.frame_buffer.vertical_resolution - self.font.char_size as u32) {
                    for x in 0..self.frame_buffer.horizontal_resolution {
                        unsafe {
                            let colour = *((self.frame_buffer.framebuffer_base_address + 4 * self.frame_buffer.pixels_per_scan_line as u64 * (y + self.font.char_size as u32) as u64 + 4 * x as u64) as *mut u32);
                            core::ptr::write_volatile((self.frame_buffer.framebuffer_base_address + 4 * self.frame_buffer.pixels_per_scan_line as u64 * y as u64 + 4 * x as u64) as *mut u32, colour);                  
                        }
                    }
                }
                self.curs_pos_y -= 1;

            }
        }
        else {
            self.curs_pos_x += 1;
        }
    }

    pub fn put_string(&mut self, string: &str){
        for char in string.chars() {
            match char {
                '\0' => return,
                '\r' => { self.curs_pos_x = 0 },
                '\n' => { self.curs_pos_y += 1},
                _ => self.put_char(char)
            }
        }
    }

    pub fn clear_screen(&self) {
        self.frame_buffer.clear_screen(self.theme.black);
    }
}

impl Write for Terminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.put_string(s);
        Ok(())
    }
}