use core::{ops::{Index, IndexMut}, ptr::{addr_of, write_bytes}};

use crate::GLOBAL_PAGE_FRAME_ALLOCATOR;

#[repr(u8)]
pub enum PageTableEntryFlag {
    Present,
    ReadWrite,
    //UserSupervisor,
    //WriteThrough,
    //CacheDisable,
    //Accessed,
    //LargePages = 7,
    //Custom0 = 9,
    //Custom1,
    //Custom2,
    //NX = 63
}

#[repr(transparent)]
pub struct PageTableEntry(u64);
impl PageTableEntry {

    pub fn clear(&mut self) {
        self.0 = 0
    }

    pub fn set_addr(&mut self, addr: u64) {
        let address: u64 = addr & 0x000_00ff_ffff_ffff;
        self.0 &= 0xfff0_0000_0000_0fff;
        self.0 |= address << 12;
    }

    pub fn set_flag(&mut self, flag: PageTableEntryFlag, value: bool) {
        let flag_selector: u64 = 1 << flag as u8;
        self.0 &= !flag_selector;
        if value {
            self.0 |= flag_selector;
        }
    }

    pub fn addr(&self) -> u64 {
        (self.0 & 0x000f_ffff_ffff_f000) >> 12
    }
    
    pub fn flag(&self, flag: PageTableEntryFlag) -> bool {
        let flag_selector: u64 = 1 << flag as u8;
        self.0 & flag_selector != 0
    }

}

#[repr(C, align(0x1000))]
pub struct PageTable([PageTableEntry; 512]);
impl Index<usize> for PageTable {
    type Output = PageTableEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

pub struct PageMapIndexer{
    pub pdp: usize,
    pub pd: usize,
    pub pt: usize,
    pub p: usize
}
impl PageMapIndexer {
    pub fn new(addr: usize) -> Self {
        Self { 
            pdp: (addr >> 39) & 0x1ff, 
            pd: (addr >> 30) & 0x1ff, 
            pt: (addr >> 21) & 0x1ff, 
            p: (addr >> 12) & 0x1ff 
        }
    }
}

pub struct PageTableManager<'a> {
    pub pml4: &'a mut PageTable
}
impl<'a> PageTableManager<'a> {
    pub fn map_mem(&mut self, physical_mem: u64, virtual_mem: u64) {

        let indexer = PageMapIndexer::new(virtual_mem as usize);

        let pdp = if !self.pml4[indexer.pdp].flag(PageTableEntryFlag::Present) {
            let temp = unsafe {
                &mut *(GLOBAL_PAGE_FRAME_ALLOCATOR.0.get_mut().unwrap().get_free_page().unwrap() as *mut PageTable)   
            };
            unsafe {
                write_bytes(temp as *mut PageTable, 0, 1);
            }
            self.pml4[indexer.pdp].set_addr(temp as *const PageTable as u64 >> 12);
            self.pml4[indexer.pdp].set_flag(PageTableEntryFlag::Present, true);
            self.pml4[indexer.pdp].set_flag(PageTableEntryFlag::ReadWrite, true);
            temp
        }else {
            unsafe {
                &mut *((self.pml4[indexer.pdp].addr() << 12) as *mut PageTable)   
            }
        };


        let pd = if !pdp[indexer.pd].flag(PageTableEntryFlag::Present) {
            let temp = unsafe {
                &mut *(GLOBAL_PAGE_FRAME_ALLOCATOR.0.get_mut().unwrap().get_free_page().unwrap() as *mut PageTable)   
            };
            unsafe {
                write_bytes(temp as *mut PageTable, 0, 1);
            }
            pdp[indexer.pd].set_addr(&temp.0 as &[PageTableEntry; 512]as *const [PageTableEntry; 512] as u64 >> 12);
            pdp[indexer.pd].set_flag(PageTableEntryFlag::Present, true);
            pdp[indexer.pd].set_flag(PageTableEntryFlag::ReadWrite, true);
            temp
        }else {
            unsafe {
                &mut *((pdp[indexer.pd].addr() << 12)  as *mut PageTable)   
            }
        };

        let pt = if !pd[indexer.pt].flag(PageTableEntryFlag::Present) {
            let temp = unsafe {
                &mut *(GLOBAL_PAGE_FRAME_ALLOCATOR.0.get_mut().unwrap().get_free_page().unwrap() as *mut PageTable)   
            };
            unsafe {
                write_bytes(temp as *mut PageTable, 0, 1);
            }
            pd[indexer.pt].set_addr(&temp.0 as &[PageTableEntry; 512]as *const [PageTableEntry; 512] as u64 >> 12);
            pd[indexer.pt].set_flag(PageTableEntryFlag::Present, true);
            pd[indexer.pt].set_flag(PageTableEntryFlag::ReadWrite, true);
            temp
        }else {
            unsafe {
                &mut *((pd[indexer.pt].addr() << 12) as *mut PageTable)   
            }
        };

        //pt[indexer.p].clear();
        pt[indexer.p].set_addr(physical_mem >> 12);
        pt[indexer.p].set_flag(PageTableEntryFlag::Present, true);
        pt[indexer.p].set_flag(PageTableEntryFlag::ReadWrite, true);

    }
}