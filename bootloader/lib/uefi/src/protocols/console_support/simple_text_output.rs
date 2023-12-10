//*/-bootloader/lib/uefi/src/protocols/console_support/simple_text_output.rs
use crate::protocols::data_types::Status;

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=524
#[repr(C)]
pub struct Protocol {
    pub reset:               extern "efiapi" fn(&Self, verification: bool) -> Status,
    pub output_string:       extern "efiapi" fn(&Self, string: *const u16) -> Status,
    pub test_string:         extern "efiapi" fn(&Self, string: *const u16) -> Status,
    pub query_mode:          extern "efiapi" fn(&Self, mode: usize, columns: &usize, rows: &usize) -> Status,
    pub set_mode:            extern "efiapi" fn(&Self, mode: usize) -> Status,
    pub set_attribute:       extern "efiapi" fn(&Self, attribute: usize) -> Status,
    pub clear_screen:        extern "efiapi" fn(&Self) -> Status,
    pub set_cursor_position: extern "efiapi" fn(&Self, column: usize, row: usize) -> Status,
    pub enable_cursor:       extern "efiapi" fn(&Self, enable: bool) -> Status,
    pub mode:                &'static Mode,
}

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=525
#[repr(C)]
pub struct Mode {
    pub max_mode:       i32,
    pub mode:           i32,
    pub attribute:      i32,
    pub cursor_column:  i32,
    pub cursor_row:     i32,
    pub cursor_visable: bool,
}
