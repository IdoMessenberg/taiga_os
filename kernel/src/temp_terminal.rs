
use core::fmt::Write;

trait FrameBufferTrait {
    unsafe fn put_pixel(&self, pos_x: u32, pos_y: u32, colour: u32);
}
pub struct Terminal{
    frame_buffer:boot::GraphicsInfo,
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
}

impl FrameBufferTrait for boot::GraphicsInfo {
    unsafe fn put_pixel(&self, pos_x: u32, pos_y: u32, colour: u32) {
        if pos_x > self.horizontal_resolution || pos_y > self.vertical_resolution{
            return;
        }
        unsafe {
            core::ptr::write_volatile((self.framebuffer_base_address + 4 * self.pixels_per_scan_line as u64 * pos_y as u64 + pos_x as u64 * 4) as *mut u32, colour);
        }
    }
}
impl Write for Terminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.put_string(s);
        Ok(())
    }
}