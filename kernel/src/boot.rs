use crate::{efi_graphics_output, psf, memory_map};
#[repr(C)]
pub struct Info {
    pub graphics:     efi_graphics_output::Info,
    pub font:         psf::Info,
    pub mem_map_info: memory_map::Info,
    pub kernel_entry_address: u64,
    pub kernel_file_size: usize
}
