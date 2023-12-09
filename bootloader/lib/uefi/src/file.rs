//*/-bootloader/lib/uefi/src/file.rs
use crate::protocols::{
    data_types::Status, loaded_image, media_access::{file, simple_file_system}, system
};

pub fn get_root(handle: *const core::ffi::c_void, system_table: &system::Table) -> Result<&file::Protocol, Status> {
    let loaded_image: *const loaded_image::Protocol = core::ptr::null();
    if (system_table.boot_time_services.handle_protocol)(handle, &loaded_image::GUID, core::ptr::addr_of!(loaded_image) as *const *const core::ffi::c_void).is_err() {
        return Err(Status::Aborted);
    }
    let file_system: *const simple_file_system::Protocol = core::ptr::null();
    if (system_table.boot_time_services.handle_protocol)(unsafe { (*loaded_image).device_handle }, &simple_file_system::GUID, core::ptr::addr_of!(file_system) as *const *const core::ffi::c_void).is_err() {
        return Err(Status::Aborted);
    }
    let root: *const file::Protocol = core::ptr::null();
    if unsafe { ((*file_system).open_volume)(file_system, core::ptr::addr_of!(root)) }.is_err() {
        return Err(Status::Aborted);
    }
    unsafe { Ok(&*root) }
}

impl file::Protocol {
    pub fn load_file(&self, file_name: *const u16) -> Result<std_alloc::vec::Vec<u8>, Status> {
        let file_handle: &file::Protocol = if let Ok(new_file_handle) = self.open_file(file_name) { new_file_handle } else { return Err(Status::Aborted) };
        let data = if let Ok(data) = file_handle.read_file() { data } else { return Err(Status::Aborted) };
        self.close_file(file_handle);
        Ok(data)
    }
    pub fn open_file(&self, file_name: *const u16) -> Result<&file::Protocol, Status> {
        let file_handle: *const file::Protocol = core::ptr::null();
        if (self.open)(self, core::ptr::addr_of!(file_handle), file_name, file::READ_MODE, file::READ_ONLY | file::HIDDEN | file::SYSTEM).is_err() {
            return Err(Status::Aborted);
        }
        if file_handle.is_null() {
            return Err(Status::Aborted);
        }
        unsafe { Ok(&*file_handle) }
    }

    fn read_file(&self) -> Result<std_alloc::vec::Vec<u8>, Status> {
        let mut file_info_size: usize = 0;
        if (self.get_info)(self, &file::INFO_GUID, &mut file_info_size, core::ptr::null()) != Status::BufferTooSmall {
            return Err(Status::BadBufferSize);
        }
        //file_info_size += 1000;
        let file_info: *const file::Info = core::ptr::null();
        if (self.get_info)(self, &file::INFO_GUID, &mut file_info_size, file_info as *const core::ffi::c_void).is_err() {
            return Err(Status::Aborted);
        }
        let file_size: usize = unsafe { (*file_info).file_size } as usize;
        //file_size.wrapping_add(rhs)
        let data: std_alloc::vec::Vec<u8> = std_alloc::vec![0; file_size];
        if (self.read)(self, &file_size, (&data).as_ptr() as *const core::ffi::c_void).is_err() {
            return Err(Status::LoadError);
        }
        return Ok(data);
    }
    pub fn close_file(&self, file_handle: *const file::Protocol) -> Status { (self.close)(file_handle) }
}
