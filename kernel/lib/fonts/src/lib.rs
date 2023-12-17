#![no_std]
extern crate efi_graphics_output;
extern crate utility;

pub mod psf {
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Info {
        pub glyph_size: u8,
        pub glyph_buffer_base_address: u64
    }
    impl Info {
        pub unsafe fn print_char(&self, graphics_output: &efi_graphics_output::Info, position: utility::Point<u32>, char: char, background_colour: u32, foreground_colour: u32){
            let mut glyph_char_scan_line_bitmap: u8 = self.get_char_glyph_from_buffer(char, 0);
            for y in position.y..position.y+self.glyph_size as u32 {
                for x in position.x..position.x+8 {
                    match glyph_char_scan_line_bitmap & (0b10000000 >> x - position.x) != 0 {
                        true => graphics_output.put_pixel(foreground_colour, utility::Point { x, y }),
                        false => graphics_output.put_pixel(background_colour, utility::Point { x, y }),
                    }
                }
                glyph_char_scan_line_bitmap = self.get_char_glyph_from_buffer(char, (y - position.y + 1) as u8);
            }
        }

        unsafe fn get_char_glyph_from_buffer(&self, char: char, position: u8) -> u8 {
            core::ptr::read_volatile((self.glyph_buffer_base_address + char as u64 * self.glyph_size as u64 + position as u64) as *const u8)
        }
    }
}