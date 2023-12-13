//*/-bootloader/lib/uefi/src/graphics.rs
use crate::protocols::{console_support::graphics_output, data_types::Status, system, system_services::boot_time};

pub fn init(system_table: &system::Table) -> Status {
    //let gop: &graphics_output::Protocol = if let Ok(gop) = system_table.boot_time_services.get_graphics_output_protocol() { gop } else { return system_table.con_out.println_status("Graphics - Graphics Output Protocol Is Not Found!", Status::NotFound) };
    if (system_table.con_out.reset)(system_table.con_out, true).is_err()
    /*|| gop.set_mode_to_hd().is_err()*/
    {
        return system_table.con_out.println_status("Graphics - Graphics Output Is Not Initialised!", Status::Aborted);
    }
    system_table.con_out.println_status("Graphics - Graphics Output Is Found and Initialised!", Status::Success)
}

#[repr(C)]
pub struct Info {
    pub frame_buffer_base_address: u64,
    pub frame_buffer_size:         usize,
    pub horizontal_resolution:     u32,
    pub vertical_resolution:       u32,
    pub pixels_per_scan_line:      u32,
}

impl boot_time::Services {
    fn get_graphics_output_protocol(&self) -> Result<&graphics_output::Protocol, Status> {
        let gop: *const graphics_output::Protocol = core::ptr::null();
        if (self.locate_protocol)(&graphics_output::GUID, core::ptr::null(), core::ptr::addr_of!(gop) as *const *const core::ffi::c_void).is_err() {
            return Err(Status::NotFound);
        }
        //safety:
        unsafe { Ok(&*gop) }
    }
    pub fn get_graphics_info(&self) -> Result<Info, Status> {
        let gop: &graphics_output::Protocol = if let Ok(gop) = self.get_graphics_output_protocol() {
            gop
        } else {
            return Err(Status::Aborted);
        };
        Ok(Info {
            frame_buffer_base_address: gop.mode.frame_buffer_base,
            frame_buffer_size:         gop.mode.frame_buffer_size,
            horizontal_resolution:     gop.mode.info.horizontal_resolution,
            vertical_resolution:       gop.mode.info.vertical_resolution,
            pixels_per_scan_line:      gop.mode.info.pixels_per_scan_line,
        })
    }
}

impl graphics_output::Protocol<'_> {
    #[allow(unused)]
    fn set_mode_to_hd(&self) -> Status {
        let mode_info: *const graphics_output::ModeInformation = core::ptr::null();
        for mode in 0..self.mode.max_mode {
            if (self.query_mode)(self, mode, &self.mode.size_of_info, core::ptr::addr_of!(mode_info)).is_err() {
                return Status::Aborted;
            };
            if mode_info.is_null() {
                return Status::Aborted;
            }
            //safety:
            if unsafe { (*mode_info).horizontal_resolution == 1920 && (*mode_info).vertical_resolution == 1080 } {
                return (self.set_mode)(self, mode);
            }
        }
        Status::NotFound
    }
}
