//*/-bootloader/lib/uefi/src/protocols/loaded_images.rs
use crate::protocols::data_types::Guid;

pub const GUID: Guid = Guid(0x5B1B31A1, 0x9562, 0x11d2, [0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B]);

#[repr(C)]
pub struct Protocol {
    pub revision:      u32,
    pub parent_handle: *const core::ffi::c_void,
    system_table:      *const core::ffi::c_void,
    pub device_handle: *const core::ffi::c_void,
}
