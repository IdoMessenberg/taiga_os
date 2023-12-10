//*/-bootloader/lib/uefi/src/protocols/system.rs
use crate::protocols::{console_support::simple_text_output, data_types::Guid, system_services::boot_time};

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=169
#[repr(C)]
pub struct Table {
    pub hdr:                 TableHeader,
    pub firmware_vendor:     *const u16,
    pub firmware_revision:   u32,
    pub console_in_handle:   *const core::ffi::c_void,
    con_in:                  *const core::ffi::c_void,
    pub console_out_handle:  *const core::ffi::c_void,
    pub con_out:             &'static simple_text_output::Protocol,
    pub console_eror_handle: *const core::ffi::c_void,
    pub std_err:             &'static simple_text_output::Protocol,
    run_time_services:       *const core::ffi::c_void,
    pub boot_time_services:  &'static boot_time::Services,
}

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=167
#[repr(C)]
pub struct TableHeader {
    pub signature:   u64,
    pub revision:    u32,
    pub header_size: u32,
    pub crc32:       u32,
    pub reserved:    u32,
}

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=177
#[repr(C)]
pub struct ConfigurationTable {
    pub vendor_guid:  Guid,
    pub vendor_table: *const core::ffi::c_void,
}