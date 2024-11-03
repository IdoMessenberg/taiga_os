#![no_std]

pub mod data_types;
pub mod loaded_image;
pub mod system;

pub mod console_support {
    pub mod graphics_output;
    pub mod simple_text_input;
    pub mod simple_text_output;
}

pub mod media_access {
    pub mod file;
    pub mod simple_file_system;
}
pub mod system_services {
    pub mod boot_time;
    pub mod run_time;
}

pub use data_types::{Guid, Status, InputKey, ResetType};
pub use system_services::boot_time;


#[repr(u8)]
pub enum Colour {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    White      = 15,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct MemoryMapInfo {
    pub address:            u64,
    pub size:               usize,
    pub key:                usize,
    pub descriptor_size:    usize,
    pub descriptor_version: u32,
}
impl MemoryMapInfo {
    pub fn get_available_memory_bytes(&self) -> usize {
        self.get_pages() * 0x1000
    }
    pub fn get_pages(&self) -> usize{
        let mut pages: usize = 0;
        for i in 0..self.size/self.descriptor_size {
            let desc: &data_types::MemoryDescriptor = unsafe {
                &*((self.address + self.descriptor_size as u64 * i as u64) as *const data_types::MemoryDescriptor)
            };
            pages += desc.number_of_pages as usize;
        };
        pages
    }
}

#[repr(C)]
pub struct GraphicsInfo {
    pub frame_buffer_base_address: u64,
    pub frame_buffer_size:         usize,
    pub horizontal_resolution:     u32,
    pub vertical_resolution:       u32,
    pub pixels_per_scan_line:      u32,
    pub theme: ColourTheme
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct ColourTheme{
    pub dark_mode: bool,
    pub white: u32,
    pub black: u32,
    pub red: u32,
    pub green: u32,
    pub blue: u32,
    pub yellow : u32,
    pub orange: u32,
    pub purple: u32,
    pub gray: u32,
    pub dark_gray: u32,
    pub light_red: u32,
    pub light_green: u32,
    pub light_blue: u32,
    pub light_yellow: u32,
    pub light_orange: u32,
    pub light_purple: u32,
}