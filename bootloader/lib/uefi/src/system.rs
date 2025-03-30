use crate::{console_support::{simple_text_input, simple_text_output}, data_types::Guid, system_services::{boot_time, run_time}};
use core::ffi::c_void;

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=169
#[repr(C)]
pub struct Table<'a> {
    pub hdr:                  TableHeader,
    pub firmware_vendor:      *const u16,
    pub firmware_revision:    u32,
    pub console_in_handle:    *const c_void,
    pub con_in:               &'a simple_text_input::Protocol,
    pub console_out_handle:   *const c_void,
    pub con_out:              &'a simple_text_output::Protocol<'a>,
    pub console_error_handle: *const c_void,
    pub std_err:              &'a simple_text_output::Protocol<'a>,
    pub run_time_services:    &'a run_time::Services,
    pub boot_time_services:   &'a boot_time::Services,
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
    pub vendor_table: *const c_void,
}
