use graphics_deriver::{Functions, PsfFontFunctions};
use util::BootInfo;

pub struct Theme{

    pub dark_mode: bool,
    pub white: graphics_deriver::Colour,
    pub black: graphics_deriver::Colour,
    pub red: graphics_deriver::Colour,
    pub green: graphics_deriver::Colour,
    pub blue: graphics_deriver::Colour,
    pub yellow : graphics_deriver::Colour,
    pub orange: graphics_deriver::Colour,
    pub purple: graphics_deriver::Colour,
    pub gray: graphics_deriver::Colour,
    pub dark_gray: graphics_deriver::Colour,
    pub light_red: graphics_deriver::Colour,
    pub light_green: graphics_deriver::Colour,
    pub light_blue: graphics_deriver::Colour,
    pub light_yellow: graphics_deriver::Colour,
    pub light_orange: graphics_deriver::Colour,
    pub light_purple: graphics_deriver::Colour,
}

pub static mut GLOBAL_TERMINAL: Terminal = Terminal::const_default();

pub struct Terminal {
    pub pos_x: u32,
    pub pos_y: u32,
    pub theme: Theme,
    pub font: psf::FontInfo,
    pub frame_buffer: graphics_deriver::FrameBuffer,
    pub fg_colour: graphics_deriver::Colour,
    pub bg_colour: graphics_deriver::Colour,
}
impl Terminal {
    pub const fn const_default() -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
            frame_buffer: graphics_deriver::FrameBuffer::const_default(),
            font: psf::FontInfo::const_default(),
            fg_colour: graphics_deriver::Colour::const_default(),
            bg_colour: graphics_deriver::Colour::const_default(),
            theme: Theme {
                dark_mode: true, 
                white: graphics_deriver::Colour::const_default(), 
                black:  graphics_deriver::Colour::const_default(), 
                red:  graphics_deriver::Colour::const_default(), 
                green:  graphics_deriver::Colour::const_default(), 
                blue:  graphics_deriver::Colour::const_default(), 
                yellow:  graphics_deriver::Colour::const_default(), 
                orange:  graphics_deriver::Colour::const_default(), 
                purple:  graphics_deriver::Colour::const_default(), 
                gray:  graphics_deriver::Colour::const_default(), 
                dark_gray:  graphics_deriver::Colour::const_default(), 
                light_red:  graphics_deriver::Colour::const_default(), 
                light_green:  graphics_deriver::Colour::const_default(), 
                light_blue:  graphics_deriver::Colour::const_default(), 
                light_yellow:  graphics_deriver::Colour::const_default(), 
                light_orange:  graphics_deriver::Colour::const_default(), 
                light_purple:  graphics_deriver::Colour::const_default(), 
            }
        }
    }
    pub const fn new(boot_info: & BootInfo, frame_buffer: graphics_deriver::FrameBuffer) -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
            frame_buffer,
            font: boot_info.font,
            fg_colour: if boot_info.graphics.theme.dark_mode { graphics_deriver::Colour::from_hex(boot_info.graphics.theme.white) }else{ graphics_deriver::Colour::from_hex(boot_info.graphics.theme.black) },
            bg_colour: if boot_info.graphics.theme.dark_mode { graphics_deriver::Colour::from_hex(boot_info.graphics.theme.black) }else{ graphics_deriver::Colour::from_hex(boot_info.graphics.theme.white) },

            theme: Theme {
                dark_mode: boot_info.graphics.theme.dark_mode, 
                white: graphics_deriver::Colour::from_hex(boot_info.graphics.theme.white), 
                black: graphics_deriver::Colour::from_hex(boot_info.graphics.theme.black), 
                red: graphics_deriver::Colour::from_hex(boot_info.graphics.theme.red), 
                green: graphics_deriver::Colour::from_hex(boot_info.graphics.theme.green), 
                blue: graphics_deriver::Colour::from_hex(boot_info.graphics.theme.blue), 
                yellow: graphics_deriver::Colour::from_hex(boot_info.graphics.theme.yellow), 
                orange: graphics_deriver::Colour::from_hex(boot_info.graphics.theme.orange), 
                purple: graphics_deriver::Colour::from_hex(boot_info.graphics.theme.purple), 
                gray: graphics_deriver::Colour::from_hex(boot_info.graphics.theme.gray), 
                dark_gray: graphics_deriver::Colour::from_hex(boot_info.graphics.theme.dark_gray), 
                light_red: graphics_deriver::Colour::from_hex(boot_info.graphics.theme.light_red), 
                light_green: graphics_deriver::Colour::from_hex(boot_info.graphics.theme.light_green), 
                light_blue: graphics_deriver::Colour::from_hex(boot_info.graphics.theme.light_blue), 
                light_yellow: graphics_deriver::Colour::from_hex(boot_info.graphics.theme.light_yellow), 
                light_orange: graphics_deriver::Colour::from_hex(boot_info.graphics.theme.light_orange), 
                light_purple: graphics_deriver::Colour::from_hex(boot_info.graphics.theme.light_purple), 
            }
        }
    }
    fn put_char(&mut self, char: char) {
        match char {
            ' ' => {
                self.pos_x += 1;
            }
            '\t' => {
                self.pos_x += 4;
            }
            '\r' => {
                self.pos_x = 0;
            }
            '\n' => {
                self.pos_y += 1
            }
            _ => {
                self.frame_buffer.put_char(
                    &self.font, 
                    self.pos_x * psf::FontInfo::CHAR_WIDTH as u32, 
                    self.pos_y * self.font.char_size as u32 , 
                    char, 
                    &self.fg_colour, 
                    &self.bg_colour
                );
                self.pos_x += 1;
            }
        }
        if self.pos_x * psf::FontInfo::CHAR_WIDTH as u32 >= self.frame_buffer.horizontal_resolution {
            self.pos_x = 0;
            self.pos_y += 1;
        }
    }
    pub fn put_num<T>(&mut self, num: &T) 
    where
        T:
         core::ops::Div<Output = T>
         + core::ops::RemAssign
         + core::cmp::PartialEq
         + core::cmp::PartialOrd
         + Into<usize>
         + Copy
         + Clone ,
         u8: Into<T>,
         usize: Into<T>{

        let mut i: usize = 1;
        if i.into() <= *num / 10u8.into() {
            while i.into() <= *num / 10u8.into()  {
                i *= 10;
            }
        }
    
        let mut temp: T = *num;
        for _ in 0..17 {
            self.put_char((b'0' + ((temp / i.into()).into() as u8)) as char);
            temp %= i.into();
            i /= 10;
            if i == 0 {
                break;
            }
        }    
    }
    pub fn print(&mut self, string: &str) {
        for char in string.chars() {
            self.put_char(char);
        }
    }
    pub fn clear_screen(&self) {
        self.frame_buffer.clear_screen(&(if self.theme.dark_mode {self.theme.black}else{self.theme.white}));
    }
    pub fn fill_screen(&self, colour: &graphics_deriver::Colour) {
        self.frame_buffer.clear_screen(colour);
    }
}