//*/-bootloader/lib/uefi/src/protocols/console_support/graphics_output.rs
use crate::protocols::data_types::{BltOperation, BltPixel, Guid, PixelBitmask, PixelFormat, Status};

//https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=560
pub const GUID: Guid = Guid(0x9042a9de, 0x23dc, 0x4a38, [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a]);

//https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=560
#[repr(C)]
pub struct Protocol<'a> {
    pub query_mode: extern "efiapi" fn(*const Self, mode_number: u32, size_of_info: *const usize, info: *const *const ModeInformation) -> Status,
    pub set_mode:   extern "efiapi" fn(*const Self, mode_number: u32) -> Status,
    pub blt:        extern "efiapi" fn(*const Self, blt_buffer: *const BltPixel, blt_operation: BltOperation, source_x: usize, source_y: usize, destination_x: usize, destination_y: usize, width: usize, height: usize, delta: usize) -> Status,
    pub mode:       &'a Mode<'a>,
}

#[repr(C)]
pub struct Mode<'a> {
    pub max_mode:          u32,
    pub mode:              u32,
    pub info:              &'a ModeInformation,
    pub size_of_info:      usize,
    pub frame_buffer_base: u64,
    pub frame_buffer_size: usize,
}

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=561
#[repr(C)]
pub struct ModeInformation {
    pub version:               u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution:   u32,
    pub pixel_format:          PixelFormat,
    pub pixel_information:     PixelBitmask,
    pub pixels_per_scan_line:  u32,
}
