//*/-bootloader/lib/uefi/src/protocols/system.rs
use crate::protocols::{console_support::simple_text_output, data_types::Guid, system_services::boot_time};

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

#[repr(C)]
pub struct TableHeader {
    pub signature:   u64,
    pub revision:    u32,
    pub header_size: u32,
    pub crc32:       u32,
    pub reserved:    u32,
}

#[repr(C)]
pub struct ConfigurationTable {
    pub vendor_guid:  Guid,
    pub vendor_table: *const core::ffi::c_void,
}
