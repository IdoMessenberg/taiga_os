//*/-bootloader/lib/uefi/src/protocols/loaded_image.rs
use crate::protocols::data_types::{Guid, MemoryType, Status};

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=363
pub const GUID: Guid = Guid(0x5B1B31A1, 0x9562, 0x11d2, [0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B]);

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=363
#[repr(C)]
pub struct Protocol {
    pub revision:      u32,
    pub parent_handle: *const core::ffi::c_void,
    system_table:      *const core::ffi::c_void,
    pub device_handle: *const core::ffi::c_void,
    file_path:         *const core::ffi::c_void,
    reserved:          *const core::ffi::c_void,
    load_options_size: u32,
    load_options:      *const core::ffi::c_void,
    image_base:        *const core::ffi::c_void,
    image_size:        u64,
    image_code_type:   MemoryType,
    image_data_type:   MemoryType,
    unload:      extern "efiapi" fn() -> Status,
}