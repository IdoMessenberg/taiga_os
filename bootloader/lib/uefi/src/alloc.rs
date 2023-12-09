//*/-bootloader/lib/uefi/src/alloc.rs
use crate::protocols::{
    data_types::{AllocateType, MemoryDescriptor, MemoryType, Status}, system, system_services::boot_time
};

#[global_allocator]
static mut BOOT_TIME_ALLOCATOR: BootTimeAllocator = BootTimeAllocator::default();

pub fn init(system_table: &system::Table) -> Status {
    unsafe {
        BOOT_TIME_ALLOCATOR = BootTimeAllocator::new(system_table.boot_time_services);
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

unsafe impl core::alloc::GlobalAlloc for BootTimeAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        if let Some(alloc_pool) = BOOT_TIME_ALLOCATOR.allocate_pool {
            let buffer: *const core::ffi::c_void = core::ptr::null();
            if alloc_pool(MemoryType::LoaderData, layout.size(), &buffer).is_ok() {
                return buffer as *mut u8;
            }
        }
        core::ptr::null_mut()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        if let Some(free_pool) = BOOT_TIME_ALLOCATOR.free_pool {
            free_pool(ptr as *const core::ffi::c_void);
        }
    }
}

impl boot_time::Services {
    pub fn allocate_pages(&self, size: usize, memory_address: *const u64) -> Status { (self.allocate_pages)(AllocateType::AllocateAddress, MemoryType::LoaderData, size, memory_address) }
    pub fn copy_mem(&self, destination: *const core::ffi::c_void, source: *const core::ffi::c_void, length: usize) { (self.copy_mem)(destination, source, length); }

    pub fn get_memory_map(&self) -> Result<MemoryMap, Status> {
        let mut memory_map: MemoryMap = MemoryMap { size: 0, key: 0, descriptor_size: 0, desc_version: 0, address: core::ptr::null() };
        (self.get_memory_map)(&memory_map.size, memory_map.address, &memory_map.key, &memory_map.descriptor_size, &memory_map.desc_version);
        memory_map.size += 2 * memory_map.desc_version as usize;
        memory_map.address = std_alloc::vec![0; memory_map.size].as_ptr() as *const MemoryDescriptor;
        if (self.allocate_pool)(MemoryType::LoaderData, memory_map.size, memory_map.address as *const *const core::ffi::c_void).is_err() {
            return Err(Status::Aborted);
        }
        (self.get_memory_map)(&memory_map.size, memory_map.address, &memory_map.key, &memory_map.descriptor_size, &memory_map.desc_version);
        Ok(memory_map)
    }
}
pub struct MemoryMap {
    pub size:            usize,
    pub key:             usize,
    pub descriptor_size: usize,
    pub desc_version:    u32,
    pub address:         *const MemoryDescriptor,
}
