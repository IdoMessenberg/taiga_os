#[repr(C)]
pub struct PageDiractoryEntry(u64);
impl PageDiractoryEntry {
    pub fn get_flag(&self, flag: Flags) -> bool {
       (self.0 & (0b1 << flag as u8)) != 0
    }
    pub fn set_flag(&mut self, flag: Flags, value: bool){
        let flag = flag as u8;
        self.0 &= !(0b1 << flag);
        if value {
            self.0 |= 0b1 << flag;
        }
    }
    pub fn get_addr(&self) -> u64 {
        (self.0 & 0x000ffffffffff000) >> 12
    }
    pub fn set_addr(&mut self, addr: u64) {
        self.0 &= 0xfff0000000000fff;
        self.0 |= (addr & 0xffffffffff) << 12
    }
}

#[repr(u8)]
pub enum Flags {
    Present = 0,
    ReadWrite = 1,
    //UserSupervisor = 2,
    //WriteThrough = 3,
    //CacheDisable = 4,
    //Accessed = 5,
    //PageSize = 7,
    //Global = 8,
    //Available0 = 9,
    //Available1 = 10,
    //Available2 = 11,
    //ExecuteDisable = 63
}

#[repr(C, align(0x1000))]
pub struct PageTable([PageDiractoryEntry; 512]);

pub struct PageMapIndexer {
    pub pdp_i: usize,
    pub pd_i: usize,
    pub pt_i: usize,
    pub p_i: usize,
}
impl PageMapIndexer {
    pub fn new(virtual_addr: u64) -> Self {
        let mut  vr: u64 = virtual_addr.clone();
        vr >>= 12;
        let p_i: usize = (vr & 0x1ff) as usize;
        vr >>= 9;
        let pt_i: usize = (vr & 0x1ff) as usize;
        vr >>= 9;
        let pd_i: usize = (vr & 0x1ff) as usize;
        vr >>= 9;
        let pdp_i: usize= (vr & 0x1ff) as usize;
        PageMapIndexer { pdp_i, pd_i, pt_i, p_i }
    }
}

pub struct PageTableManager{pub pml4 :*mut PageTable}
impl PageTableManager {
    pub fn map_memory(&self, physical_addr: u64, virtual_addr: u64) {
        let indexer: PageMapIndexer = PageMapIndexer::new(virtual_addr);

        let pdp: *mut PageTable;
        let pde: &mut PageDiractoryEntry = unsafe{&mut (&mut *self.pml4).0[indexer.pdp_i]};
        if !pde.get_flag(Flags::Present) {
            pdp = unsafe{ pfa::GLOBAL_ALLOC.get_free_page().unwrap()}  as *mut PageTable; 
            unsafe{
                core::ptr::write_bytes(pdp, 0, 1);
            }
            pde.set_addr((pdp as u64) >> 12);
            pde.set_flag(Flags::Present, true);
            pde.set_flag(Flags::ReadWrite, true);
            
        }
        else {
            pdp = (pde.get_addr() << 12) as *mut PageTable
        }

        let pd:  *mut PageTable;
        let pde: &mut PageDiractoryEntry = unsafe{&mut (&mut *pdp).0[indexer.pd_i]};
        if !pde.get_flag(Flags::Present) {
            pd = unsafe{ pfa::GLOBAL_ALLOC.get_free_page().unwrap() } as *mut PageTable; 
            unsafe{
                core::ptr::write_bytes(pd , 0, 1);
            }
            pde.set_addr((pd as u64) >> 12);
            pde.set_flag(Flags::Present, true);
            pde.set_flag(Flags::ReadWrite, true);
        }
        else {
            pd = (pde.get_addr() << 12) as *mut PageTable
        }

        let pt: *mut PageTable;
        let pde: &mut PageDiractoryEntry = unsafe{&mut (&mut *pd).0[indexer.pt_i]};
        if !pde.get_flag(Flags::Present) {
            pt = unsafe{ pfa::GLOBAL_ALLOC.get_free_page().unwrap() } as *mut PageTable; 
            unsafe{
                core::ptr::write_bytes(pt , 0, 1);
            }
            pde.set_addr((pt as u64) >> 12);
            pde.set_flag(Flags::Present, true);
            pde.set_flag(Flags::ReadWrite, true);
        }
        else {
            pt = (pde.get_addr() << 12) as *mut PageTable
        }

        let pde: &mut PageDiractoryEntry = unsafe{&mut (&mut *pt).0[indexer.p_i]};
        pde.set_addr(physical_addr >> 12);
        pde.set_flag(Flags::Present, true);
        pde.set_flag(Flags::ReadWrite, true);
    }
}


pub static mut PML4: *mut PageTable = core::ptr::null_mut();
pub static mut PTM: PageTableManager = PageTableManager{pml4: core::ptr::null_mut()};
#[inline]
pub unsafe fn init(boot_info: &util::BootInfo) {
    PML4 = page_frame_allocator::GLOBAL_ALLOC.get_free_page().unwrap() as *mut PageTable; 
    core::ptr::write_bytes(PML4 , 0, 1);
    
    PTM = PageTableManager{pml4:PML4};
    
    for i in (0..boot_info.memory_map_info.get_available_memory_bytes() as u64).step_by(pfa::PAGE_SIZE) {  
        PTM.map_memory(i, i);
    }
    for i in (boot_info.graphics.frame_buffer_base_address..(boot_info.graphics.frame_buffer_base_address + boot_info.graphics.frame_buffer_size as u64)).step_by(pfa::PAGE_SIZE) {
        PTM.map_memory(i, i);
    }
    let cr0: u64 = 0;
    let v: u64 = 0x80000000;
    core::arch::
    asm!(
        "mov cr3, {x}",
        x = in(reg) PML4 as u64,
        options(nostack, preserves_flags)
    );
    core::arch::
    asm!(
        "mov {x}, cr0", 
        "or {x}, {v}",
        "mov cr0, {x}",
        x  = in(reg) cr0,
        v = in(reg) v,
        options(nostack)
    );
}