//*/-bootloader/lib/uefi/src/protocols/media_access/simple_file_system.rs
use crate::protocols::{
    data_types::{Guid, Status}, media_access::file
};

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=585
pub const GUID: Guid = Guid(0x0964e5b22, 0x6459, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=585
#[repr(C)]
pub struct Protocol {
    pub revision:    u64,
    pub open_volume: extern "efiapi" fn(*const Self, root: *const *const file::Protocol) -> Status,
}
