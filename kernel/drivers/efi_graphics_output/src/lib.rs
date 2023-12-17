#![no_std]
extern crate utility;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Info {
    frame_buffer_base_address: u64,
    frame_buffer_size: usize,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pixels_per_scan_line: u32
}

impl Info {
    pub unsafe fn put_pixel(&self, colour: u32, position : utility::Point<u32>) {
        if position.x < self.horizontal_resolution && position.y < self.vertical_resolution {
            core::ptr::write_volatile((self.frame_buffer_base_address + 4 * self.pixels_per_scan_line as u64 * position.y as u64 + 4 * position.x as u64) as *mut u32, colour)
        }
    }   
}