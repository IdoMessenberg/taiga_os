use crate::GLOBAL_ALLOC;

#[repr(C)]
#[derive(Clone, Copy)]
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
    UserSupervisor = 2,
    WriteThrough = 3,
    CacheDisable = 4,
    Accessed = 5,
    PageSize = 7,
    Available0 = 9,
    Available1 = 10,
    Available2 = 11,
    ExecuteDisable = 63
}

#[repr(C, align(0x1000))]
#[derive(Clone, Copy)]
pub struct PageTable([PageDiractoryEntry; 512]);

pub struct PageMapIndexer {
    pub pdp_i: usize,
    pub pd_i: usize,
    pub pt_i: usize,
    pub p_i: usize,
}
impl PageMapIndexer {
    pub fn new(virtual_addr: u64) -> Self {
        let mut  vr: usize = virtual_addr as usize;
        vr >>= 12;
        let p_i: usize = vr & 0x1ff;
        vr >>= 9;
        let pt_i: usize = vr & 0x1ff;
        vr >>= 9;
        let pd_i: usize = vr & 0x1ff;
        vr >>= 9;
        let pdp_i: usize= vr & 0x1ff;
        PageMapIndexer { pdp_i, pd_i, pt_i, p_i }
    }
}

pub struct PageTableManager{pub pml4 :*mut PageTable}
impl PageTableManager {
    pub fn map_memory(&self, physical_addr: u64, virtual_addr: u64) {
        let indexer = PageMapIndexer::new(virtual_addr);

        let mut pde = unsafe{*self.pml4}.0[indexer.pdp_i];
        let pdp: *mut PageTable;
        if !pde.get_flag(Flags::Present) {
            pdp = unsafe{GLOBAL_ALLOC.get_free_page().unwrap()} as *mut PageTable; 
            unsafe{
                core::ptr::write_bytes(pdp, 0, boot::PAGE_SIZE);
            }
            pde.set_addr((pdp as u64) >> 12);
            pde.set_flag(Flags::Present, true);
            pde.set_flag(Flags::ReadWrite, true);
            unsafe{(*self.pml4).0[indexer.pdp_i] = pde}
        }
        else {
            pdp = (pde.get_addr() << 12) as *mut PageTable
        }

        pde = unsafe{*pdp}.0[indexer.pd_i];
        let pd:  *mut PageTable;
        if !pde.get_flag(Flags::Present) {
            pd = unsafe{GLOBAL_ALLOC.get_free_page().unwrap()} as *mut PageTable; 
            unsafe{
                core::ptr::write_bytes(pd, 0, boot::PAGE_SIZE);
            }
            pde.set_addr((pd as u64) >> 12);
            pde.set_flag(Flags::Present, true);
            pde.set_flag(Flags::ReadWrite, true);
            unsafe{(*pdp).0[indexer.pd_i] = pde}
        }
        else {
            pd = (pde.get_addr() << 12) as *mut PageTable
        }

        pde = unsafe{*pd}.0[indexer.pt_i];
        let pt: *mut PageTable;
        if !pde.get_flag(Flags::Present) {
            pt = unsafe{GLOBAL_ALLOC.get_free_page().unwrap()} as *mut PageTable; 
            unsafe{
                core::ptr::write_bytes(pt, 0, boot::PAGE_SIZE);
            }
            pde.set_addr((pt as u64) >> 12);
            pde.set_flag(Flags::Present, true);
            pde.set_flag(Flags::ReadWrite, true);
            unsafe{(*pd).0[indexer.pt_i] = pde}
        }
        else {
            pt = (pde.get_addr() << 12) as *mut PageTable
        }

        pde = unsafe {*pt}.0[indexer.p_i];
        pde.set_addr(physical_addr >> 12);
        pde.set_flag(Flags::Present, true);
        pde.set_flag(Flags::ReadWrite, true);
        unsafe{*pt}.0[indexer.p_i] = pde;
    }
}