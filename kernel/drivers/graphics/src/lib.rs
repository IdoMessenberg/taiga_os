#![no_std]

#[derive(Clone, Copy)]
pub struct Colour {
    red: u8,
    green: u8,
    blue: u8,
    pub alpha: u8
}
impl Colour {
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self{
            red,
            green,
            blue,
            alpha: 0xff,
        }
    }
    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self{
            red,
            green,
            blue,
            alpha
        }
    }
    pub fn to_hex(&self) -> u32 {
        (self.alpha as u32) << 24 | (self.red as u32) << 16 | (self.green as u32) << 8 | (self.blue as u32)
    }

    pub fn from_hex(hex: u32) -> Colour {
        Colour {
            alpha:{
                    let a =( (hex >> 24) & 0xFF) as u8;
                    if a != 0  {
                        a
                    }
                    else {
                        0xff
                    }
                },
            red: ((hex >> 16) & 0xFF) as u8,
            green: ((hex >> 8) & 0xFF) as u8,
            blue: (hex & 0xFF) as u8,
        }
    }

    pub fn alpha_composite(&self, over: &Colour) -> Colour {
        let alpha_self = self.alpha as f32 / 255.0;
        let alpha_over = over.alpha as f32 / 255.0;

        let composite_alpha = alpha_self + alpha_over * (1.0 - alpha_self);
        let composite_red = (self.red as f32 * alpha_self / composite_alpha + over.red as f32 * alpha_over * (1.0 - alpha_self) / composite_alpha) as u8;
        let composite_green = (self.green as f32 * alpha_self / composite_alpha + over.green as f32 * alpha_over * (1.0 - alpha_self) / composite_alpha) as u8;
        let composite_blue = (self.blue as f32 * alpha_self / composite_alpha + over.blue as f32 * alpha_over * (1.0 - alpha_self) / composite_alpha) as u8;

        Colour {
            red: composite_red,
            green: composite_green,
            blue: composite_blue,
            alpha: (composite_alpha * 255.0) as u8,
        }
    }
}

pub static mut GLOBAL_FRAME_BUFFER: FrameBuffer = FrameBuffer::const_default();

pub struct FrameBuffer {
    pub base_address: u64,
    pub pixels_per_scan_line: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32
}
impl FrameBuffer {
    pub const fn const_default() -> Self {
        Self { base_address: 0, pixels_per_scan_line: 0, horizontal_resolution: 0, vertical_resolution: 0 }
    }
    pub const fn const_init( base_address: u64, pixels_per_scan_line: u32, horizontal_resolution: u32, vertical_resolution: u32 ) -> Self {
        Self { base_address, pixels_per_scan_line, horizontal_resolution, vertical_resolution}
    }
}
pub unsafe trait PutPixel {
    unsafe fn put_pixel(&self, pos_x: u32, pos_y: u32, colour: &Colour);
    unsafe fn put_pixel_with_alpha_correction(&self, pos_x: u32, pos_y: u32, colour: &Colour);
}
unsafe impl PutPixel for FrameBuffer {
    unsafe fn put_pixel(&self, pos_x: u32, pos_y: u32, colour: &Colour) {
        core::ptr::write_volatile((self.base_address + 4 * self.pixels_per_scan_line as u64 * pos_y as u64 + 4 * pos_x as u64) as *mut u32, colour.to_hex());
    } 
    unsafe fn put_pixel_with_alpha_correction(&self, pos_x: u32, pos_y: u32, colour: &Colour) {
        if colour.alpha == 0 {
            return;
        }
        else if colour.alpha != 0xff {
            let bg_colour: Colour = Colour::from_hex(*((self.base_address + 4 * self.pixels_per_scan_line as u64 * pos_y as u64 + 4 * pos_x as u64) as *const u32));       
            self.put_pixel(pos_x, pos_y, &colour.alpha_composite(&bg_colour));
        }
        else {
            self.put_pixel(pos_x, pos_y, colour);
        }
    }
}

pub trait Functions {
    fn clear_screen(&self, colour: &Colour);
    fn clear_screen_with_alpha_correction(&self, colour: &Colour);
}
impl Functions for FrameBuffer {
    fn clear_screen(&self, colour: &Colour) {
        for y in 0..self.vertical_resolution {
            for x in 0..self.horizontal_resolution {
                unsafe {
                    self.put_pixel(x, y, &colour);                    
                }
            }
        }
    }
    fn clear_screen_with_alpha_correction(&self, colour: &Colour) {
        for y in 0..self.vertical_resolution {
            for x in 0..self.horizontal_resolution {
                unsafe {
                    self.put_pixel_with_alpha_correction(x, y, &colour);                    
                }
            }
        }
    }
}

pub trait PsfFontFunctions {
    fn put_char(&self, font: &psf::FontInfo, pos_x: u32, pos_y: u32, char: char, fg_colour: &Colour, bg_colour: &Colour);
    fn put_string(&self, font: &psf::FontInfo, pos_x: u32, pos_y: u32, string: &str, fg_colour: &Colour, bg_colour: &Colour);
}
impl<T: PutPixel> PsfFontFunctions for T {
    fn put_char(&self, font: &psf::FontInfo, pos_x: u32, pos_y: u32, char: char, fg_colour: &Colour, bg_colour: &Colour) {
        let mut glyph_char_scan_line_bitmap: u8 = unsafe{font.get_char_glyph_from_buffer(char, 0)};
        for p_y in pos_y..pos_y + font.char_size as u32 {
            for p_x in pos_x..pos_x + 8 {
                if glyph_char_scan_line_bitmap & 0b10000000 >> (p_x - pos_x) != 0 {
                    unsafe{self.put_pixel(p_x, p_y, fg_colour )}
                }
                else {
                    unsafe{self.put_pixel(p_x, p_y, bg_colour)} 
                }
            }
            glyph_char_scan_line_bitmap = unsafe {font.get_char_glyph_from_buffer(char, (p_y - pos_y + 1) as u8)};
        }
    }

    fn put_string(&self, font: &psf::FontInfo, pos_x: u32, pos_y: u32, string: &str, fg_colour: &Colour, bg_colour: &Colour) {
        for char in string.chars() {
            self.put_char(font, pos_x, pos_y, char, fg_colour, bg_colour);
        }
    }
}