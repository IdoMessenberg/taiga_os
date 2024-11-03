#![no_std]

#[repr(C)]
pub struct BootInfo {
    pub graphics:             uefi::GraphicsInfo,
    pub memory_map_info:      uefi::MemoryMapInfo,
    pub font:                 psf::FontInfo,
}