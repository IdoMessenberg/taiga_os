use taiga64::{data_types::Point, drivers::efi_graphics_output, psf};
use crate::BootInfo;
pub struct Output{
    graphics_output: efi_graphics_output::Info,
    font: psf::Info,
    cursor_position: Point<u32>,
    pub background_colour: u32,
    pub foreground_colour: u32,
}

impl Output {
    pub fn new(boot_info: &BootInfo) -> Self {
        Output { graphics_output: boot_info.graphics, font: boot_info.font, cursor_position: Point { x: 0, y: 0 }, background_colour: 0x171819, foreground_colour: 0xB1AD8D }
    }
    pub fn put_char(&mut self, char: char){
        match char {
            '\r' => self.cursor_position.x = 0,
            '\n' => self.cursor_position.y += 1,
            '\t' => {
                self.put_char(' ');
                self.put_char(' ');
                self.put_char(' ');
                self.put_char(' ');
            }
            _ => {
                if self.cursor_position.x * 8 > self.graphics_output.horizontal_resolution {
                    self.cursor_position.x = 0;
                    self.cursor_position.y += 1;
                }
                unsafe{
                    self.font.print_char(&self.graphics_output, Point { x: self.cursor_position.x * 8, y: self.cursor_position.y * self.font.glyph_size as u32 }, char, self.background_colour, self.foreground_colour)
                }
                self.cursor_position.x += 1;
            }
        }
    }

    pub fn put_usize(&mut self, num: &usize) {
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
        for y in 0..self.graphics_output.vertical_resolution {
            for x in 0..self.graphics_output.horizontal_resolution {
                unsafe { self.graphics_output.put_pixel(self.background_colour, Point { x, y })}
            }
        }
    }
    
    pub fn print(&mut self, string: &str) {
        for char in string.chars() {
            self.put_char(char);
        }
    }

    pub fn println(&mut self, string: &str) {
        self.print(string);
        self.print("\r\n");
    }

    pub fn set_cursor_position(&mut self, new_position: Point<u32>){
        self.cursor_position = new_position
    }

}