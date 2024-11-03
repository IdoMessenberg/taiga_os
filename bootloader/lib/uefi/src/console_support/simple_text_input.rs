use crate::data_types::{Status, InputKey};

#[repr(C)]
pub struct Protocol {
    pub reset: extern "efiapi" fn(&Self, verification: bool) -> Status,
    pub read_key_stroke: extern "efiapi" fn(&Self, key: *const InputKey) -> Status,
    pub wait_for_key: *const core::ffi::c_void
}