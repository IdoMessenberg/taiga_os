//*/-kernel/src/terminal.rs
use crate::{FontInfo, GraphicsInfo};

pub struct Output {
    pub info:                  Info,
    frame_buffer_base_address: u64,
    pixels_per_scan_line:      u32,
    glyph_char_size:           u8,
    glyph_buffer_base_address: *const core::ffi::c_void,
    horizontal_resolution:     u32,
    vertical_resolution:       u32,
}

pub struct Info {
    x:                     u64,
    y:                     u64,
    pub background_colour: u32,
    pub foreground_colour: u32,
}

impl Output {
    pub fn new(graphics_info: &GraphicsInfo, font_info: FontInfo) -> Self {
        Output {
            info:                      Info { x: 0, y: 0, background_colour: 0x171819, foreground_colour: 0xB1AD8D },
            frame_buffer_base_address: graphics_info.frame_buffer_base_address,
            pixels_per_scan_line:      graphics_info.pixels_per_scan_line,
            glyph_char_size:           font_info.glyph_char_size,
            glyph_buffer_base_address: font_info.glyph_buffer_base_address,
            horizontal_resolution:     graphics_info.horizontal_resolution,
            vertical_resolution:       graphics_info.vertical_resolution,
        }
    }
    pub fn init(&self) { self.clear_screen() }

    unsafe fn put_char(&mut self, char: char) {
        match char {
            '\r' => self.info.x = 0,
            '\n' => self.info.y += self.glyph_char_size as u64,
            '\t' => {
                self.put_char(' ');
                self.put_char(' ');
                self.put_char(' ');
                self.put_char(' ');
            }
            _ => {
                if self.info.x > self.pixels_per_scan_line as u64 {
                    self.info.x = 0;
                    self.info.y += self.glyph_char_size as u64;
                }
                let mut glyph_buffer_char_ptr: *const u8 = self.glyph_buffer_base_address.add(char as usize * self.glyph_char_size as usize).cast::<u8>();
                for y in self.info.y..self.info.y + self.glyph_char_size as u64 {
                    for x in self.info.x..self.info.x + 8 {
                        match (*glyph_buffer_char_ptr) & (0b10000000 >> (x - self.info.x)) != 0 {
                            true => core::ptr::write_volatile((self.frame_buffer_base_address + 4 * self.pixels_per_scan_line as u64 * y + 4 * x) as *mut u32, self.info.foreground_colour),
                            false => core::ptr::write_volatile((self.frame_buffer_base_address + 4 * self.pixels_per_scan_line as u64 * y + 4 * x) as *mut u32, self.info.background_colour),
                        }
                    }
                    glyph_buffer_char_ptr = glyph_buffer_char_ptr.add(1);
                }
                self.info.x += 8;
            }
        }
    }

    pub unsafe fn put_usize(&mut self, num: &usize) {
        let mut i = 1;
        if i <= num / 10 {
            for _ in 0..17 {
                i *= 10;
                if i >= num / 10 {
                    break;
                }
            }
        }

        let mut temp: usize = *num;
        for _ in 0..17 {
            self.put_char((b'0' + (temp / i) as u8) as char);
            temp %= i;
            i /= 10;
            if i == 0 {
                break;
            }
        }
    }

    pub fn clear_screen(&self) {
        for y in 0..self.vertical_resolution as u64 {
            for x in 0..self.horizontal_resolution as u64 {
                unsafe { core::ptr::write_volatile((self.frame_buffer_base_address + 4 * self.pixels_per_scan_line as u64 * y + 4 * x) as *mut u32, self.info.background_colour) };
            }
        }
    }

    pub fn print(&mut self, string: &str) {
        for char in string.chars() {
            unsafe { self.put_char(char) };
        }
    }

    pub fn println(&mut self, string: &str) {
        self.print(string);
        self.print("\r\n");
    }

    pub fn set_cursor_position(&mut self, x: u64, y: u64){
        self.info.x = x * 8;
        self.info.y = y * self.glyph_char_size as u64;
    }
}
