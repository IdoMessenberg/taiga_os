use core::ptr::{null_mut, write_bytes, NonNull};

use crate::GLOBAL_PAGE_FRAME_ALLOCATOR;


#[repr(u8)]
pub enum EntryFlags{
    Present,
    ReadWrite,
    UserSupervisor,
    WriteThrough,
    CacheDisable,
    Accessed,
    LargePages = 7,
    Custom0 = 9,
    Custom1,
    Custom2,
    NX = 63
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PageDirectoryEntry(u64);
impl PageDirectoryEntry {
    pub fn set_flag(&mut self, flag: EntryFlags, value: bool) {
        let bit_selector: u64 = 1 << flag as u8;
        self.0 &= ! bit_selector;
        if value {
            self.0 |= bit_selector;
        }
    }

    pub fn get_flag(&self, flag: EntryFlags) -> bool {
        let bit_selector: u64 = 1 << flag as u8;
        self.0 & bit_selector != 0
    }

    pub fn set_addr(&mut self, addr: u64) {
        let addr = addr & 0x000000ffffffffff;
        self.0 &= 0xfff0000000000fff;
        self.0 |= addr << 12;

    }

    pub fn addr(&self) -> u64 {
        self.0 & 0x000ffffffffff000 >> 12
    }
}

#[repr(C, align(0x1000))]
#[derive(Clone, Copy)]
pub struct PageTable(pub [PageDirectoryEntry; 512]);

pub struct PageMapIndexer{
    pub pdp_i: usize,
    pub pd_i: usize,
    pub pt_i: usize,
    pub p_i: usize,
}
impl PageMapIndexer {
    pub fn new(virtual_addr: u64) -> Self {
        Self { 
            p_i: ((virtual_addr >> 12) & 0x1ff) as usize, 
            pt_i: ((virtual_addr >> 21) & 0x1ff) as usize, 
            pd_i: ((virtual_addr >> 30) & 0x1ff) as usize, 
            pdp_i: ((virtual_addr >> 39) & 0x1ff) as usize
        }
    }
}

pub struct PageTableManager<'a>{
    pml4: &'a mut  PageTable
}
impl<'a> PageTableManager<'a> {
    pub fn new(pml4_addr: &'a mut PageTable) -> Self {
        Self { pml4: pml4_addr }
    }

    pub fn map_mem(&mut self, virtual_addr: u64, physical_addr: u64) {
        let indexer: PageMapIndexer = PageMapIndexer::new(virtual_addr);
        let mut pde: PageDirectoryEntry = self.pml4.0[indexer.pdp_i];

        let pdp: &mut PageTable;
        if !pde.get_flag(EntryFlags::Present) {
            pdp = unsafe {
                NonNull::new_unchecked(GLOBAL_PAGE_FRAME_ALLOCATOR.get_mut().unwrap().get_free_page().unwrap() as *mut PageTable).as_mut()
            }; 
            unsafe{
                core::ptr::write_bytes(pdp, 0, 1);
            }
            pde.set_addr((pdp.0.as_ptr() as u64) >> 12);
            pde.set_flag(EntryFlags::Present, true);
            pde.set_flag(EntryFlags::ReadWrite, true);
            self.pml4.0[indexer.pdp_i] = pde;
            
        }
        else {
            pdp = unsafe{
                NonNull::new_unchecked( (pde.addr() << 12) as *mut PageTable).as_mut()
            }
        }

        let pd:&mut PageTable;
        let mut pde: PageDirectoryEntry = pdp.0[indexer.pd_i];
        if !pde.get_flag(EntryFlags::Present) {
            pd = unsafe {
                NonNull::new_unchecked(GLOBAL_PAGE_FRAME_ALLOCATOR.get_mut().unwrap().get_free_page().unwrap() as *mut PageTable).as_mut()
            }; 
            unsafe{
                core::ptr::write_bytes(pd, 0, 1);
            }
            pde.set_addr((pd.0.as_ptr() as u64) >> 12);
            pde.set_flag(EntryFlags::Present, true);
            pde.set_flag(EntryFlags::ReadWrite, true);
            pdp.0[indexer.pd_i] = pde;
            
        }
        else {
            pd = unsafe{
                NonNull::new_unchecked( (pde.addr() << 12) as *mut PageTable).as_mut()
            }
        }

        let pt:&mut PageTable;
        let mut pde: PageDirectoryEntry = pdp.0[indexer.pt_i];
        if !pde.get_flag(EntryFlags::Present) {
            pt = unsafe {
                NonNull::new_unchecked(GLOBAL_PAGE_FRAME_ALLOCATOR.get_mut().unwrap().get_free_page().unwrap() as *mut PageTable).as_mut()
            }; 
            unsafe{
                core::ptr::write_bytes(pt, 0, 1);
            }
            pde.set_addr((pt.0.as_ptr() as u64) >> 12);
            pde.set_flag(EntryFlags::Present, true);
            pde.set_flag(EntryFlags::ReadWrite, true);
            pdp.0[indexer.pt_i] = pde;
            
        }
        else {
            pt = unsafe{
                NonNull::new_unchecked( (pde.addr() << 12) as *mut PageTable).as_mut()
            }
        }


        let mut pde: PageDirectoryEntry = pt.0[indexer.p_i];
        pde.set_addr(physical_addr >> 12);
        pde.set_flag(EntryFlags::Present, true);
        pde.set_flag(EntryFlags::ReadWrite, true);
        pt.0[indexer.p_i] = pde;
    }
}