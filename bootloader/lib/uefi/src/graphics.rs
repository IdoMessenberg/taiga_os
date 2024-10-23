use crate::protocols::{console_support::graphics_output, data_types::{PixelBitmask, PixelFormat, Status}, system_services::boot_time};

#[repr(C)]
pub struct Info {
    pub frame_buffer_base_address: u64,
    pub frame_buffer_size:         usize,
    pub horizontal_resolution:     u32,
    pub vertical_resolution:       u32,
    pub pixels_per_scan_line:      u32,
    pub theme: Theme
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct Theme{
    pub dark_mode: bool,
    pub white: u32,
    pub black: u32,
    pub red: u32,
    pub green: u32,
    pub blue: u32,
    pub yellow : u32,
    pub orange: u32,
    pub purple: u32,
    pub gray: u32,
    pub dark_gray: u32,
    pub light_red: u32,
    pub light_green: u32,
    pub light_blue: u32,
    pub light_yellow: u32,
    pub light_orange: u32,
    pub light_purple: u32,
}

impl boot_time::Services {
    pub fn get_graphics_output_protocol<'a>(&self) -> Result<&'a graphics_output::Protocol, Status> {
        let gop: *const graphics_output::Protocol = core::ptr::null();
        let status = (self.locate_protocol)(&graphics_output::GUID, core::ptr::null(), core::ptr::addr_of!(gop) as *const *const core::ffi::c_void);
        if !status.is_ok() {
            return Err(status)
        }
        if gop.is_null() {
            return  Err(Status::NotFound)
        }
        //todo:
        //safety:
        //
        unsafe { Ok(&*gop) }
    }
}

impl graphics_output::Protocol<'_> {
    pub fn set_mode_to_resulotion(&self, width: u32, hight: u32) -> Status {
        let mode_info: &graphics_output::ModeInformation = &graphics_output::ModeInformation::default();
        for mode in 0..self.mode.max_mode {
            if !(self.query_mode)(self, mode, &self.mode.size_of_info, core::ptr::addr_of!(mode_info)).is_ok() {
                return Status::Aborted
            };
            if mode_info.horizontal_resolution == width && mode_info.vertical_resolution == hight {
                return (self.set_mode)(self, mode);
            }
        }
        Status::NotFound
    }

    pub fn get_graphics_info(&self) -> Info {
        Info {
            frame_buffer_base_address: self.mode.frame_buffer_base,
            frame_buffer_size:         self.mode.frame_buffer_size,
            horizontal_resolution:     self.mode.info.horizontal_resolution,
            vertical_resolution:       self.mode.info.vertical_resolution,
            pixels_per_scan_line:      self.mode.info.pixels_per_scan_line,
            theme: Theme::default()
        }
    }
}

impl Default for graphics_output::ModeInformation {
    fn default() -> Self {
        graphics_output::ModeInformation{
            version:               0,
            horizontal_resolution: 0,
            vertical_resolution:   0,
            pixel_format:          PixelFormat::PixelRedGreenBlueReserved8BitPerColor,
            pixel_information:     PixelBitmask{
                red_mask:      0,
                green_mask:    0,
                blue_mask:     0,
                reserved_mask: 0,
            },
            pixels_per_scan_line:  0,
        }
    }
}