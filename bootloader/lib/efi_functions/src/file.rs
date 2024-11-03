use uefi::{
    data_types::Status, loaded_image, media_access::{file, simple_file_system}, system_services::boot_time
};
use crate::alloc::BootTimeAllocFunctions;
use core::ptr::{null, addr_of};
//todo: clean up

pub trait FileFunctions {
    fn load_file(&self, boot_time_services: &boot_time::Services, file_name: *const u16) -> Result<std_alloc::vec::Vec<u8>, Status>;
    fn open_file(&self, file_name: *const u16) -> Result<&file::Protocol, Status>;
    fn read_file(&self, boot_time_services: &boot_time::Services) -> Result<std_alloc::vec::Vec<u8>, Status>;
    fn close_file(&self, file_handle: &file::Protocol) -> Status;
}

/// unfortunately this function is'nt very safe as the file protocol requires quite a bit of nulls,
/// this is contained and i do try to check if the output is null but this is still an unsafe operation
pub fn get_root<'a>(handle: *const core::ffi::c_void, boot_time_services: &boot_time::Services) -> Result<&'a file::Protocol, Status> {
    //-getting the loaded image protocol to get the device handle
    let loaded_image: *const loaded_image::Protocol = null();
    if !(boot_time_services.handle_protocol)(handle, &loaded_image::GUID, addr_of!(loaded_image) as *const *const core::ffi::c_void).is_ok() {
        return Err(Status::Aborted);
    }
    //making sure that the loaded image is not null to make sure the next part is safe and not accessing a null pointer
    if loaded_image.is_null() {
        return Err(Status::Aborted);
    }
    //-getting the simple file system protocol to open the system root
    let file_system: *const simple_file_system::Protocol = null();
    //TODO
    //safety: 
    //this is getting the file system and should be safe as i checked that the loaded image is not null before
    //and the unsafe is needed to derefrance the loaded image to access the device handle
    if !(boot_time_services.handle_protocol)(unsafe {(*loaded_image).device_handle}, &simple_file_system::GUID, addr_of!(file_system) as *const *const core::ffi::c_void).is_ok() {
        return Err(Status::Aborted);
    }
    //making sure that the file system is not null to make sure the next part is safe and not accessing a null pointer
    if file_system.is_null() {
        return Err(Status::Aborted);
    }
    //-getting the root file protocol
    let root: *const file::Protocol = null();
    //TODO
    //safety:
    //this is getting the root and should be safe as i checked that the file system is not null before
    //and the unsafe is needed to derefrance the file system to access the open volume function
    if ! unsafe {(*file_system).open_volume}(file_system, addr_of!(root)).is_ok() {
        return Err(Status::Aborted);
    }
    //making sure that the root is not null to make sure the next part is safe and not accessing a null pointer
    if root.is_null() {
        return Err(Status::Aborted);
    }
    //TODO
    //safety: 
    //this is returning the root file protocol and should be safe as i checked that the root is not null before
    //and the unsafe is needed to derefrance the root so it could be borrowd
    unsafe { Ok(&*root) }
}

impl FileFunctions for file::Protocol {
    fn load_file(&self, boot_time_services: &boot_time::Services, file_name: *const u16) -> Result<std_alloc::vec::Vec<u8>, Status> {
        let file_handle: &file::Protocol = if let Ok(new_file_handle) = self.open_file(file_name) { new_file_handle } else { return Err(Status::Aborted) };
        let data: std_alloc::vec::Vec<u8> = if let Ok(data) = file_handle.read_file(boot_time_services) { data } else { return Err(Status::Aborted) };
        self.close_file(file_handle);
        Ok(data)
    }
    fn open_file(&self, file_name: *const u16) -> Result<&file::Protocol, Status> {
        let file_handle: *const file::Protocol = null();
        if !(self.open)(self, addr_of!(file_handle), file_name, file::READ_MODE, file::READ_ONLY | file::HIDDEN | file::SYSTEM).is_ok() || file_handle.is_null() {
            return Err(Status::Aborted);
        }
        
        //TODO
        //safety:
        //this is returning the file's file protocol and should be safe as i checked that the file is not null before
        //and the unsafe is needed to derefrance the file handle so it could be borrowd
        unsafe { Ok(&*file_handle) }
    }

    fn read_file(&self, boot_time_services: &boot_time::Services) -> Result<std_alloc::vec::Vec<u8>, Status> {
        let mut file_info_size: usize = 0;
        let file_info: *const file::Info = null();
        (self.get_info)(self, &file::INFO_GUID, &mut file_info_size, null());
        boot_time_services.alloc_pool(file_info_size, addr_of!(file_info) as *const *const core::ffi::c_void);
        if !(self.get_info)(self, &file::INFO_GUID, &mut file_info_size, file_info as *const core::ffi::c_void).is_ok() {
            return Err(Status::Aborted);
        }
        //TODO
        //safety:
        //this is getting the file_size and should be safe as i checked that the get info is not an error before
        //and the unsafe is needed to derefrance the file size so it could be used
        let file_size: usize = unsafe { (*file_info).file_size.clone() } as usize;
        let data: std_alloc::vec::Vec<u8> = std_alloc::vec![0; file_size];
        if !(self.read)(self, &file_size, data.as_ptr() as *const core::ffi::c_void).is_ok() {
            return Err(Status::LoadError);
        }
        Ok(data)
    }

    fn close_file(&self, file_handle: &file::Protocol) -> Status { (self.close)(file_handle) }
}
