use crate::protocols::{
    data_types::{ResetType, Status}, system
};

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=174&zoom=100,96,822
#[repr(C)]
pub struct Services {
    pub hdr:                        system::TableHeader,
    get_time:                       extern "efiapi" fn() -> Status,
    set_time:                       extern "efiapi" fn() -> Status,
    get_wakeup_time:                extern "efiapi" fn() -> Status,
    set_wakeup_time:                extern "efiapi" fn() -> Status,
    set_virtual_address_map:        extern "efiapi" fn() -> Status,
    convert_map:                    extern "efiapi" fn() -> Status,
    get_variable:                   extern "efiapi" fn() -> Status,
    get_next_variable_name:         extern "efiapi" fn() -> Status,
    set_variable:                   extern "efiapi" fn() -> Status,
    get_next_high_monotonic_count:  extern "efiapi" fn() -> Status,
    pub reset_system:               extern "efiapi" fn(reset_type: ResetType, reset_status: Status, data_size: usize) -> Status,
}