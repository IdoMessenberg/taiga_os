#![no_std]

pub use uefi as efi;
pub use efi_file_loader as efl;

pub const PAGE_SIZE: usize = 4096;

#[repr(C)]
pub struct Info {
    pub graphics:             efi::graphics::Info,
    pub memory_map_info:      efi::alloc::MemoryMapInfo,
    pub font:                 efl::psf::FontInfo,
}