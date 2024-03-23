//*/-bootloader/lib/uefi/src/alloc.rs
use crate::protocols::{
    data_types::{AllocateType, MemoryDescriptor, MemoryType, Status}, system, system_services::boot_time
};

#[global_allocator]
static mut BOOT_TIME_ALLOCATOR: BootTimeAllocator = BootTimeAllocator::default();

pub fn init(system_table: &system::Table) -> Status {
    //-safety: according to the Rust compiler changing/accessing a mutable static is unsafe
    unsafe {
        //-initialising the global allocator with the boot time services allocation protocols
        BOOT_TIME_ALLOCATOR = BootTimeAllocator::new(system_table.boot_time_services);

        //-making sure that the global allocator was initialised correctly
        if BOOT_TIME_ALLOCATOR.allocate_pool.is_none() || BOOT_TIME_ALLOCATOR.free_pool.is_none() {
            return system_table.con_out.println_status("Boot Time Allocator - UEFI Boot Time Allocation Services Is Not Found!", Status::LoadError);
        }
    }
    system_table.con_out.println_status("Boot Time Allocator - Boot Time Allocator Is Loaded!", Status::Success)
}

struct BootTimeAllocator {
    allocate_pool: Option<boot_time::AllocatePool>,
    free_pool:     Option<boot_time::FreePool>,
}
impl BootTimeAllocator {
    const fn default() -> Self { BootTimeAllocator { allocate_pool: None, free_pool: None } }
    fn new(boot_time_services: &boot_time::Services) -> Self { BootTimeAllocator { allocate_pool: Some(boot_time_services.allocate_pool), free_pool: Some(boot_time_services.free_pool) } }
}

//-safety: dynamic memory allocation is an unsafe operation so `core::alloc::GlobalAlloc` is an unsafe trait
unsafe impl core::alloc::GlobalAlloc for BootTimeAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        if let Some(alloc_pool) = self.allocate_pool {
            let buffer: *const core::ffi::c_void = core::ptr::null();
            if alloc_pool(MemoryType::LoaderData, layout.size(), &buffer).is_ok() {
                return buffer as *mut u8;
            }
        }
        core::ptr::null_mut()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        if let Some(free_pool) = self.free_pool {
            free_pool(ptr as *const core::ffi::c_void);
        }
    }
}

impl boot_time::Services {
    pub fn allocate_pages(&self, size: usize, memory_address: *const u64) -> Status { (self.allocate_pages)(AllocateType::AllocateAddress, MemoryType::LoaderData, size, memory_address) }
    pub fn allocate_pool(&self, size: usize, memory_address: *const *const core::ffi::c_void) -> Status { (self.allocate_pool)(MemoryType::LoaderData, size, memory_address) }

    pub fn copy_mem(&self, destination: *const core::ffi::c_void, source: *const core::ffi::c_void, length: usize) { (self.copy_mem)(destination, source, length); }

    pub fn get_memory_map(&self) -> Result<MemoryMapInfo, Status> {
        let mut size: usize = 0;
        let mut key: usize = 0;
        let mut descriptor_size: usize = 0;
        let mut descriptor_version: u32 = 0;
        let  map: *const MemoryDescriptor = core::ptr::null_mut();
        (self.get_memory_map)(&mut size, map, &mut key, &mut descriptor_size, &mut descriptor_version);
        for _ in 0..10 {
            if !map.is_null() && !(self.free_pool)(map as *const core::ffi::c_void).is_ok() {
                return Err(Status::Aborted);
            }
            if (self.allocate_pool)(MemoryType::LoaderData, size, &map as *const *const MemoryDescriptor as *const *const core::ffi::c_void).is_err() {
                return Err(Status::Aborted);
            }
            if (self.get_memory_map)(&mut size, map, &mut key, &mut descriptor_size, &mut descriptor_version).is_ok() && !map.is_null(){
                return Ok(MemoryMapInfo { address: map as u64, size, key, descriptor_size, descriptor_version });
            }
        }
        Err(Status::Aborted)
    }
}

#[repr(C)]
pub struct MemoryMapInfo {
    pub address:            u64,
    pub size:               usize,
    pub key:                usize,
    pub descriptor_size:    usize,
    pub descriptor_version: u32,
}
