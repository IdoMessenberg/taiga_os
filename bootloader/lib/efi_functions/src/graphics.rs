use uefi::{console_support::graphics_output, data_types::Status, system_services::boot_time};

pub trait BootTimeGraphicsFunctions {
    fn get_graphics_output_protocol<'a>(&self) -> Result<&'a graphics_output::Protocol, Status>;
}
pub trait GraphicsFunctions {
    fn set_mode_to_resulotion(&self, width: u32, hight: u32) -> Status;
    fn get_graphics_info(&self) -> uefi::GraphicsInfo;
}


impl BootTimeGraphicsFunctions for boot_time::Services {
    fn get_graphics_output_protocol<'a>(&self) -> Result<&'a graphics_output::Protocol, Status> {
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

impl GraphicsFunctions for graphics_output::Protocol<'_> {
    fn set_mode_to_resulotion(&self, width: u32, hight: u32) -> Status {
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

    fn get_graphics_info(&self) -> uefi::GraphicsInfo {
        uefi::GraphicsInfo {
            frame_buffer_base_address: self.mode.frame_buffer_base,
            frame_buffer_size:         self.mode.frame_buffer_size,
            horizontal_resolution:     self.mode.info.horizontal_resolution,
            vertical_resolution:       self.mode.info.vertical_resolution,
            pixels_per_scan_line:      self.mode.info.pixels_per_scan_line,
            theme: uefi::ColourTheme::default()
        }
    }
}