use crate::GLOBAL_PAGE_FRAME_ALLOCATOR;
use core::ptr::{addr_of_mut, write_bytes, NonNull};

#[repr(u8)]
enum PageDirectoryFlag {
    Present = 0,
    ReadWrite = 1,
    //UserSupervisor = 2,
    //WriteThrough = 3,
    //CacheDisable = 4,
    //Accessed = 5,
    //Available = 6,
    //LargePage = 7,
    //ExecuteDisable = 63
}


#[repr(C)]
#[derive(Clone, Copy)]
pub struct PageDirectoryEntry(u64);
impl PageDirectoryEntry {
    fn new() -> Self {
        Self(0)
    }
    fn get_flag(&self, flag: PageDirectoryFlag) -> bool {
        self.0 & (1 << flag as u8) != 0
    }

    fn set_flag(&mut self, flag: PageDirectoryFlag, value: bool) {
        let value_mask: u64 = 1 << flag as u8;
        if value {
            self.0 |= value_mask;
        }
        else {
            self.0 &= !value_mask;
        }
    }


    fn get_addr(&self) -> u64 {
        self.0 & 0x000f_ffff_ffff_f000
    }

    fn set_addr(&mut self, addr: u64) {
        let virt_add: u64 = addr & 0x0000_00ff_ffff_ffff;

        self.0 &= 0xfff0_0000_0000_0fff;
        self.0 |= virt_add << 12;
    }
}


#[repr(align(0x1000))]
pub struct PageTable([PageDirectoryEntry; 512]);
impl PageTable {
    fn new() -> Self {
        Self([PageDirectoryEntry::new(); 512])
    }
}
impl core::ops::Index<u16> for PageTable {
    type Output = PageDirectoryEntry;
    
    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize]
    }
}
impl core::ops::IndexMut<u16> for PageTable {
    
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

struct PageTableIndexer {
    offset: u16,
    page: u16,
    page_table: u16,
    page_dir: u16,
}
impl PageTableIndexer {
    fn new(virt_addr: u64) -> Self {
        Self {
            offset: ((virt_addr >> 12) & 0x1ff) as u16, 
            page: ((virt_addr >> 21) & 0x1ff) as u16, 
            page_table: ((virt_addr >> 30) & 0x1ff) as u16, 
            page_dir: ((virt_addr >> 39) & 0x1ff) as u16 
        }
    }
}

pub struct PageTableManager {
    pub pml4: PageTable
}
impl PageTableManager {
    pub fn new() -> Self {
        Self {
            pml4: PageTable::new()
        }
    }

    pub fn map_page(&mut self, physical_addr: u64, virtual_addr: u64) {
        let indexer = PageTableIndexer::new(virtual_addr);

        const SET_PT_TO_NULL: fn(&mut PageTable) = |mut pt| unsafe {
            write_bytes(addr_of_mut!(pt), 0, 1);
        };

        let pde: &mut PageTable;
        if !self.pml4[indexer.page_dir].get_flag(PageDirectoryFlag::Present) {
            let pd: u64 = GLOBAL_PAGE_FRAME_ALLOCATOR.0.get_mut().unwrap().get_free_page().unwrap() as u64;
            pde = unsafe {NonNull::new_unchecked(pd as *mut PageTable).as_mut()};
            SET_PT_TO_NULL(pde);
            self.pml4[indexer.page_dir].set_addr(pd >> 12);
            self.pml4[indexer.page_dir].set_flag(PageDirectoryFlag::Present, true);
            self.pml4[indexer.page_dir].set_flag(PageDirectoryFlag::ReadWrite, true);
        }
        else {
            pde = unsafe {NonNull::new_unchecked( self.pml4[indexer.page_dir].get_addr() as *mut PageTable).as_mut()};
        }


        let pte: &mut PageTable;
        if !pde[indexer.page_table].get_flag(PageDirectoryFlag::Present) {
            let pt: u64 = GLOBAL_PAGE_FRAME_ALLOCATOR.0.get_mut().unwrap().get_free_page().unwrap() as u64;
            pte = unsafe {NonNull::new_unchecked(pt as *mut PageTable).as_mut()};
            SET_PT_TO_NULL(pte);
            pde[indexer.page_table].set_addr(pt >> 12);
            pde[indexer.page_table].set_flag(PageDirectoryFlag::Present, true);
            pde[indexer.page_table].set_flag(PageDirectoryFlag::ReadWrite, true);
        }
        else {
            pte = unsafe {NonNull::new_unchecked( pde[indexer.page_table].get_addr() as *mut PageTable).as_mut()};
        }

        let pe: &mut PageTable;
        if !pte[indexer.page].get_flag(PageDirectoryFlag::Present) {
            let p: u64 = GLOBAL_PAGE_FRAME_ALLOCATOR.0.get_mut().unwrap().get_free_page().unwrap() as u64;
            pe = unsafe {NonNull::new_unchecked(p as *mut PageTable).as_mut()};
            SET_PT_TO_NULL(pe);
            pte[indexer.page].set_addr(p >> 12);
            pte[indexer.page].set_flag(PageDirectoryFlag::Present, true);
            pte[indexer.page].set_flag(PageDirectoryFlag::ReadWrite, true);
        }
        else {
            pe = unsafe {NonNull::new_unchecked( pte[indexer.page].get_addr() as *mut PageTable).as_mut()};
        }

        pe[indexer.offset].set_addr(physical_addr >> 12);
        pe[indexer.offset].set_flag(PageDirectoryFlag::Present, true);
        pe[indexer.offset].set_flag(PageDirectoryFlag::ReadWrite, true);
    }
}
