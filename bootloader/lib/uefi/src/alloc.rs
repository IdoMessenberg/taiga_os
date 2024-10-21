use crate::protocols::{
    data_types::{AllocateType, MemoryDescriptor, MemoryType, Status}, system, system_services::boot_time
};
use core::{alloc::GlobalAlloc, ffi::c_void};
use  core::ptr::{null, null_mut};

#[global_allocator]
static mut BOOT_TIME_EFI_ALLOCATOR: BootTimeEfiAllocator = BootTimeEfiAllocator::default();

pub fn init(system_table: &system::Table) -> Status {
    //safety: 
    // The using and modifying of a mutable static is an unsafe operation
    // this operation is safe as it accesses a mutable static that is only used in one thread
    unsafe{
        BOOT_TIME_EFI_ALLOCATOR = BootTimeEfiAllocator::new(system_table.boot_time_services);
        
        if BOOT_TIME_EFI_ALLOCATOR.alloc_pool.is_none() || BOOT_TIME_EFI_ALLOCATOR.free_pool.is_none() {
            return Status::NotFound
        }
    }
    Status::Success
}

struct BootTimeEfiAllocator {
    alloc_pool: Option<boot_time::AllocatePool>,
    free_pool:     Option<boot_time::FreePool>,
}
impl BootTimeEfiAllocator {
    const fn default() -> Self { BootTimeEfiAllocator { alloc_pool: None, free_pool: None } }
    fn new(boot_time_services: &boot_time::Services) -> Self { BootTimeEfiAllocator { alloc_pool: Some(boot_time_services.allocate_pool), free_pool: Some(boot_time_services.free_pool) } }
}

//safety: 
// `core::alloc::GlobalAlloc` is an unsafe trait
// global allocation is an unsafe operation as it uses a direct access to memory
// these function should be safe as they use UEFI's allocation operations to access memory
// and uses multiple safety checks
unsafe impl GlobalAlloc for BootTimeEfiAllocator{
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        if let Some(alloc_pool) = self.alloc_pool {
            let buffer: *const c_void = null();
            if alloc_pool(MemoryType::LoaderData, layout.size(), core::ptr::addr_of!(buffer)).is_ok() {
                return buffer as *mut u8
            }
        }
        null_mut()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        if let Some(free_pool) = self.free_pool {
            free_pool(ptr as *const c_void);
        }
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct MemoryMapInfo {
    pub address:            u64,
    pub size:               usize,
    pub key:                usize,
    pub descriptor_size:    usize,
    pub descriptor_version: u32,
}

impl boot_time::Services {
    //boot time services abstaction as functions
    pub fn alloc_pages(&self, size: usize, memory_address: *const u64) -> Status {(self.allocate_pages)(AllocateType::AllocateAddress, MemoryType::LoaderCode, size, memory_address)}
    pub fn alloc_pool(&self, size: usize, memory_address: *const *const c_void) -> Status {(self.allocate_pool)(MemoryType::LoaderData, size, memory_address)}

    pub fn copy_mem(&self, destination: *const c_void, source: *const c_void, size: usize) {(self.copy_mem)(destination, source, size);}

    pub fn get_memory_map(&self) -> Result<MemoryMapInfo, Status> {
        let memory_map = MemoryMapInfo::default();
        (self.get_memory_map)(&memory_map.size, memory_map.address as *const MemoryDescriptor, &memory_map.key, &memory_map.descriptor_size, &memory_map.descriptor_version);
        self.alloc_pool(memory_map.size, core::ptr::addr_of!(memory_map.address) as *const *const c_void);
        match (self.get_memory_map)(&memory_map.size, memory_map.address as *const MemoryDescriptor, &memory_map.key, &memory_map.descriptor_size, &memory_map.descriptor_version).is_ok() {
            true => Ok(memory_map),
            false => Err(Status::NotFound)
        }
    }
}
