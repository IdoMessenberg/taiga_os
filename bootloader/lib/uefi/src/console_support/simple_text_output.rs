use crate::data_types::Status;

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=524
#[repr(C)]
pub struct Protocol<'a> {
    pub reset:               extern "efiapi" fn(&Self, verification: bool) -> Status,
    pub output_string:       extern "efiapi" fn(&Self, string: *const u16) -> Status,
    pub test_string:         extern "efiapi" fn(&Self, string: *const u16) -> Status,
    pub query_mode:          extern "efiapi" fn(&Self, mode: usize, columns: &usize, rows: &usize) -> Status,
    pub set_mode:            extern "efiapi" fn(&Self, mode: usize) -> Status,
    pub set_attribute:       extern "efiapi" fn(&Self, attribute: usize) -> Status,
    pub clear_screen:        extern "efiapi" fn(&Self) -> Status,
    pub set_cursor_position: extern "efiapi" fn(&Self, column: usize, row: usize) -> Status,
    pub enable_cursor:       extern "efiapi" fn(&Self, enable: bool) -> Status,
    pub mode:                &'a Mode,
}

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=525
#[repr(C)]
pub struct Mode {
    pub max_mode:       i32,
    pub mode:           i32,
    pub attribute:      i32,
    pub cursor_column:  i32,
    pub cursor_row:     i32,
    pub cursor_visible: bool,
}

#[repr(u8)]
pub enum Colour {
    Black        = 0x00,
    Blue         = 0x01,
    Green        = 0x02,
    Cyan         = 0x03,
    Red          = 0x04,
    Magenta      = 0x05,
    Brown        = 0x06,
    LightGray    = 0x07,
    DarkGray     = 0x08,
    LightBlue    = 0x09,
    LightGreen   = 0x0a,
    LightCyan    = 0x0b,
    LightRed     = 0x0c,
    LightMagenta = 0x0d,
    Yellow       = 0x0e,
    White        = 0x0f,

    BackgroundBlue      = 0x10,
    BackgroundGreen     = 0x20,
    BackgroundCyan      = 0x30,
    BackgroundRed       = 0x40,
    BackgroundMagenta   = 0x50,
    BackgroundBrown     = 0x60,
    BackgroundLightGray = 0x70,
}
