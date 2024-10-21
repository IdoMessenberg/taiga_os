use crate::protocols::data_types::{Guid, Status};

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=587
#[repr(C)]
pub struct Protocol {
    pub revision: u64,
    pub open:     extern "efiapi" fn(*const Self, new_handle: *const *const Self, file_name: *const u16, open_mode: u64, attributes: u64) -> Status,
    pub close:    extern "efiapi" fn(*const Self) -> Status,
    delete:       extern "efiapi" fn() -> Status,
    pub read:     extern "efiapi" fn(*const Self, buffer_size: *const usize, buffer: *const core::ffi::c_void) -> Status,
    write:        extern "efiapi" fn() -> Status,
    get_position: extern "efiapi" fn() -> Status,
    set_position: extern "efiapi" fn() -> Status,
    pub get_info: extern "efiapi" fn(*const Self, information_type: *const Guid, buffer_size: *mut usize, buffer: *const core::ffi::c_void) -> Status,
    set_info:     extern "efiapi" fn() -> Status,
    flush:        extern "efiapi" fn() -> Status,
    open_ex:      extern "efiapi" fn() -> Status,
    read_ex:      extern "efiapi" fn() -> Status,
    write_ex:     extern "efiapi" fn() -> Status,
    flush_ex:     extern "efiapi" fn() -> Status,
}

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=605
pub const INFO_GUID: Guid = Guid(0x09576E92, 0x6D3F, 0x11D2, [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B]);

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
#[repr(C)]
pub struct Info {
    pub size:          u64,
    pub file_size:     u64,
    pub physical_size: u64,
    create_time:       u16,
    last_access_time:  u16,
    modification_time: u16,
    pub attribute:     u64,
    pub file_name:     *const u16,
}

//https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=590
//Open file modes
pub const READ_MODE: u64 = 0x0000000000000001;
//Open file attribtes
pub const READ_ONLY: u64 = 0x0000000000000001;
pub const HIDDEN: u64 = 0x0000000000000002;
pub const SYSTEM: u64 = 0x0000000000000004;
