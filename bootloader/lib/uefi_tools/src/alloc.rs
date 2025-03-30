use core::{alloc::GlobalAlloc, cell::UnsafeCell, ffi::c_void, mem::MaybeUninit, ptr::{null, null_mut}, sync::atomic::AtomicBool};
use uefi::{data_types::{MemoryDescriptor, MemoryMapInfo, AllocateType, MemoryType ,Status}, system_services::boot_time, system};


#[global_allocator]
pub static BOOT_TIME_GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator::empty();

pub fn init(system_table: &system::Table) -> Status {
    match BOOT_TIME_GLOBAL_ALLOCATOR.init::<_, Status>(|| Result::Ok(EfiAllocator::new(system_table.boot_time_services)), Status::LoadError){
        Ok(_) => Status::Success,
        Err(e) => e
    }
}

pub type GlobalAllocator = OnceLock<EfiAllocator>;
impl GlobalAllocator {
    const fn empty() -> Self {
        OnceLock::new()
    }
}
#[derive(Debug)]
pub struct EfiAllocator {
    alloc_pool: boot_time::AllocatePool,
    free_pool:  boot_time::FreePool,
}
impl EfiAllocator {
    fn new(boot_time_services: &boot_time::Services) -> Self {
        EfiAllocator{
            alloc_pool: boot_time_services.allocate_pool,
            free_pool: boot_time_services.free_pool
        }
    }
}

unsafe impl GlobalAlloc for GlobalAllocator{
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        if let Some(val) = BOOT_TIME_GLOBAL_ALLOCATOR.get(){
            let buffer: *const c_void = null();
            if (val.alloc_pool)(MemoryType::LoaderData, layout.size(), core::ptr::addr_of!(buffer)).is_ok() {
                return buffer as *mut u8
            }
        }
        null_mut()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        if let Some(val) = BOOT_TIME_GLOBAL_ALLOCATOR.get(){
            (val.free_pool)(ptr as *const c_void);
        }
    } 
}

pub trait BootTimeAllocFunctions {
    fn alloc_pages(&self, size: usize, memory_address: *const u64) -> Status;
    fn alloc_pool(&self, size: usize, memory_address: *const *const c_void) -> Status;
    fn copy_mem(&self, destination: *const c_void, source: *const c_void, size: usize);
    fn get_memory_map(&self) -> Result<MemoryMapInfo, Status>;
}

impl BootTimeAllocFunctions for boot_time::Services {
    //boot time services abstraction as functions
    fn alloc_pages(&self, size: usize, memory_address: *const u64) -> Status {(self.allocate_pages)(AllocateType::AllocateAddress, MemoryType::LoaderCode, size, memory_address)}
    fn alloc_pool(&self, size: usize, memory_address: *const *const c_void) -> Status {(self.allocate_pool)(MemoryType::LoaderData, size, memory_address)}
    fn copy_mem(&self, destination: *const c_void, source: *const c_void, size: usize) {(self.copy_mem)(destination, source, size);}

    fn get_memory_map(&self) -> Result<MemoryMapInfo, Status> {
        let memory_map: MemoryMapInfo = MemoryMapInfo::default();
        (self.get_memory_map)(&memory_map.size, memory_map.address as *const MemoryDescriptor, &memory_map.key, &memory_map.descriptor_size, &memory_map.descriptor_version);
        self.alloc_pool(memory_map.size, core::ptr::addr_of!(memory_map.address) as *const *const c_void);
        match (self.get_memory_map)(&memory_map.size, memory_map.address as *const MemoryDescriptor, &memory_map.key, &memory_map.descriptor_size, &memory_map.descriptor_version) {
            Status::Success => Ok(memory_map),
            err => Err(err)
        }
    }
}

#[derive(Debug)]
pub struct OnceLock<T>{
    once: AtomicBool,
    value: UnsafeCell<MaybeUninit<T>>
}
impl<T> OnceLock<T> {
    pub const fn new() -> Self {
        Self {
            once: AtomicBool::new(false),
            value: UnsafeCell::new(MaybeUninit::uninit())
        }
    }
    
    fn is_initialized(&self) -> bool {
        self.once.load(core::sync::atomic::Ordering::Relaxed)
    }

    fn initialize<F, E>(&self, f: F) -> Result<(), E>
    where
        F: FnOnce() -> Result<T, E>
    {
        match f() {
            Ok(val) => {
                unsafe{
                    self.value.get().as_mut().unwrap().write(val);
                    self.once.store(true, core::sync::atomic::Ordering::Relaxed);
                }
            },
            Err(e) => {
                return Err(e);
            }
        }
        Ok(())
    }

    unsafe fn get_value_unchecked(&self) -> &T {
        unsafe{(&*self.value.get()).assume_init_ref()}
    }

    pub fn get(&self) -> Option<&T> {
        match self.is_initialized() {
            true => unsafe{Some(self.get_value_unchecked())},
            false => None
        }
    }

    pub fn get_or_try_init<F, E> (&self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<T, E>
    {
        match self.get() {
            Some(val) => {return Ok(val)},
            None => {self.initialize(f)?;},
        }
        Ok(unsafe{self.get_value_unchecked()})
    }

    pub fn get_or_init<F> (&self, f: F) -> &T
    where
        F: FnOnce() -> T
    {
        match self.get_or_try_init(|| Ok::<T, ()>(f())) {
            Ok(val) => val,
            Err(_handler) => unreachable!(),
        }
    }

    pub fn init<F, E> (&self, f: F, error_code: E) -> Result<(), E>
    where
        F: FnOnce() -> Result<T,E>,
    {
        match self.is_initialized() {
            true => Err(error_code),
            false => self.initialize(f)
        }
    }
}
unsafe impl<T> Sync for OnceLock<T>{}