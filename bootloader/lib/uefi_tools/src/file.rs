use core::{ffi::c_void, ptr::{addr_of, null}};
use std_alloc::vec::Vec;
use uefi::{boot_time, loaded_image, media_access::{file, simple_file_system}, system, Status};
use crate::alloc::BootTimeAllocFunctions;
use std_alloc::vec;

pub trait FileFunctions {
    fn get_file(&self, file_path: *const u16, system_table: &system::Table) -> Result<Vec<u8>,Status>;
    fn open_file(&self, file_path: *const u16) -> Result<&file::Protocol, Status>;
    fn close_file(&self, file_handle: &file::Protocol) -> Status;
    fn read_file(&self, boot_time_services: &boot_time::Services) -> Result<Vec<u8>, Status>;
}
pub trait Thing {
    fn get_root(&self, handle: *const c_void) -> Result<&file::Protocol, Status>;
}

impl Thing for boot_time::Services {
    fn get_root(&self, handle: *const c_void) -> Result<&file::Protocol, Status> {
        let loaded_image: *const loaded_image::Protocol = null();
        match (self.handle_protocol)(handle, &loaded_image::GUID, addr_of!(loaded_image) as *const *const c_void) {
            Status::Success => {
                if loaded_image.is_null() {
                    return Err(Status::Aborted)
                }
            }
            err => {return Err(err)},
        }

        let file_system: *const simple_file_system::Protocol = null();
        match (self.handle_protocol)(unsafe {&*loaded_image}.device_handle, &simple_file_system::GUID, addr_of!(file_system) as *const *const c_void) {
            Status::Success => {
                if file_system.is_null() {
                    return Err(Status::Aborted)
                }
            }
            err => {return Err(err)},
        }

        let root: *const file::Protocol = null();
        match (unsafe {&*file_system}.open_volume)(file_system, addr_of!(root)) {
            Status::Success => {
                if root.is_null() {
                    return Err(Status::Aborted)
                }
            }
            err => {return Err(err)},
        }
        Ok(unsafe { &*root })
    }
}
impl FileFunctions for file::Protocol {
    fn get_file(&self, file_path: *const u16, system_table: &system::Table) -> Result<Vec<u8>, Status> {
        let file_handle:&file::Protocol = match self.open_file(file_path){
            Ok(v) => v,
            Err(e) => return Err(e)
        };
        let data: Vec<u8> = match file_handle.read_file(system_table.boot_time_services) {
            Ok(v) => v,
            Err(e) => return Err(e) 
        };
        self.close_file(file_handle);
        Ok(data)
    }
    fn open_file(&self, file_path: *const u16) -> Result<&file::Protocol, Status> {
        let file_handle: *const file::Protocol = null();
        match (self.open)(self, addr_of!(file_handle), file_path, file::READ_MODE, file::READ_ONLY | file::HIDDEN | file::SYSTEM){
            Status::Success => Ok(unsafe {&*file_handle}),
            err => Err(err)
        }
    }

    fn read_file(&self, boot_time_services: &boot_time::Services) -> Result<Vec<u8>, Status> {
        let mut file_info_size: usize = 0;
        let file_info: *const file::Info = null();
        (self.get_info)(self, &file::INFO_GUID, &mut file_info_size, null());
        boot_time_services.alloc_pool(file_info_size, addr_of!(file_info) as *const *const c_void);
        match (self.get_info)(self, &file::INFO_GUID, &mut file_info_size, file_info as *const c_void) {
            Status::Success => {
                let file_size: usize = unsafe { (*file_info).file_size.clone() } as usize;
                let file_data: Vec<u8> = vec![0; file_size];
                match (self.read)(self, &file_size, file_data.as_ptr() as *const c_void){
                    Status::Success => Ok(file_data),
                    err => Err(err),
                }
            },
            err => return Err(err),
        }
    }

    fn close_file(&self, file_handle: &file::Protocol) -> Status { (self.close)(file_handle) }
}
