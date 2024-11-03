#![no_std]

pub const PSF_1_MAG_0: u8 = 0x36;
pub const PSF_1_MAG_1: u8 = 0x04;

///https://en.wikipedia.org/wiki/PC_Screen_Font
#[repr(C)]
pub struct Header {
    pub magic_bytes:   [u8; 2],
    pub font_mode:     u8,
    pub char_size: u8,
}

#[repr(C)]
pub struct FontInfo {
    pub char_size:                 u8,
    pub glyph_buffer_base_address: u64,
}
impl FontInfo {
    pub unsafe fn get_char_glyph_from_buffer(&self, char: char, position: u8) -> u8 { *((self.glyph_buffer_base_address + char as u64 * self.char_size as u64 + position as u64) as *const u8) }
}