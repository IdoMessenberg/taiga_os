use crate::{drivers::efi_graphics_output, fonts::psf, memory};
#[repr(C)]
pub struct Info {
    pub graphics:     efi_graphics_output::Info,
    pub font:         psf::Info,
    pub mem_map_info: memory::map::Info,
}