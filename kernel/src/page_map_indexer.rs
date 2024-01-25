#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PageDirectoryEntry(u64);
impl PageDirectoryEntry {
    pub fn set_flag(&mut self, flag: PageTableFlag, value: bool) {
        self.0 &= !(1 << flag as u8);
        if value {
            self.0 |= 1 << flag as u8;
        }
    }
    pub fn get_flag(&self, flag: PageTableFlag) -> bool { self.0 & (1 << flag as u8) != 0 }
    pub fn set_address(&mut self, address: u64) {
        let address = address & 0x000000FFFFFFFFFF;
        self.0 &= 0xFFF0000000000FFF;
        self.0 |= address << 12;
    }
    pub fn get_address(&self) -> u64 { (self.0 & 0x000FFFFFFFFFF000) >> 12 }
}
#[derive(Clone, Copy)]
pub enum PageTableFlag {
    Present       = 0,
    ReadWrite     = 1,
    UserSuper     = 2,
    WriteThrough  = 3,
    CacheDisabled = 4,
    Accessed      = 5,
    LargerPages   = 7,
    Custom0       = 9,
    Custom1       = 10,
    Custom2       = 11,
}

#[repr(C, align(0x1000))]
pub struct PageTable {
    pub entries: [PageDirectoryEntry; 512],
}

pub struct PageMapIndexer {
    pub page_directory_pointer_index: u64,
    pub page_directory_index:         u64,
    pub page_table_index:             u64,
    pub page_index:                   u64,
}
impl PageMapIndexer {
    pub fn new(virtual_address: u64) -> Self {
        Self {
            page_directory_pointer_index: (virtual_address >> 12) & 0x1FF,
            page_directory_index:         (virtual_address >> 21) & 0x1FF,
            page_table_index:             (virtual_address >> 30) & 0x1FF,
            page_index:                   (virtual_address >> 39) & 0x1FF,
        }
    }
}

pub struct PageTableManager {
    pub pml4: *mut PageTable,
}
impl PageTableManager {
    pub fn new(pml4_address: u64) -> Self { Self { pml4: pml4_address as  *mut PageTable } }

    pub fn map_memory(&self,virtual_address: u64, physical_address: u64) {
        let indexer = PageMapIndexer::new(virtual_address);

        let mut page_directory_entry: PageDirectoryEntry = unsafe {(*self.pml4).entries[indexer.page_directory_pointer_index as usize]};
        let page_directory_pointer: *mut PageTable = if page_directory_entry.get_flag(PageTableFlag::Present) {
            (page_directory_entry.get_address() << 12) as *mut PageTable
        }
        else{
            let address = unsafe {crate::GLOBAL_ALLOC.get_free_page().expect("no free pages")};
            crate::memory_paging::set_mem(address, 0, 0x1000);
            page_directory_entry.set_address(address >> 12);
            page_directory_entry.set_flag(PageTableFlag::Present, true);
            page_directory_entry.set_flag(PageTableFlag::ReadWrite, true);
            unsafe{
               (*self.pml4).entries[indexer.page_directory_pointer_index as usize] = page_directory_entry;
            }
            address as *mut PageTable
        };
        
        page_directory_entry = unsafe {(*page_directory_pointer).entries[indexer.page_directory_index as usize]};
        let page_directory: *mut PageTable = if page_directory_entry.get_flag(PageTableFlag::Present) {
            (page_directory_entry.get_address() << 12) as *mut PageTable
        }
        else{
            let address = unsafe {crate::GLOBAL_ALLOC.get_free_page().expect("no free pages")};
            crate::memory_paging::set_mem(address, 0, 0x1000);
            page_directory_entry.set_address(address >> 12);
            page_directory_entry.set_flag(PageTableFlag::Present, true);
            page_directory_entry.set_flag(PageTableFlag::ReadWrite, true);
            unsafe{
                (*page_directory_pointer).entries[indexer.page_directory_index as usize] = page_directory_entry;
            }
            address as *mut PageTable
        };
        /*
        page_directory_entry = unsafe {(*page_directory).entries[indexer.page_table_index as usize]};
        let page_table: *mut PageTable = if page_directory_entry.get_flag(PageTableFlag::Present) {
            (page_directory_entry.get_address() << 12) as *mut PageTable
        }
        else{
            let address = unsafe {crate::GLOBAL_ALLOC.get_free_page()};
            crate::memory_paging::set_mem(address, 0, 0x1000);
            page_directory_entry.set_address(address >> 12);
            page_directory_entry.set_flag(PageTableFlag::Present, true);
            page_directory_entry.set_flag(PageTableFlag::ReadWrite, true);
            unsafe{
                (*page_directory).entries[indexer.page_table_index as usize] = page_directory_entry;
            }
            address as *mut PageTable
        };
        
        page_directory_entry = unsafe {(*page_table).entries[indexer.page_index as usize]};
        page_directory_entry.set_address(physical_address >> 12);
        page_directory_entry.set_flag(PageTableFlag::Present, true);
        page_directory_entry.set_flag(PageTableFlag::ReadWrite, true);
        
        unsafe{
            (*page_table).entries[indexer.page_index as usize] = page_directory_entry;
        }
        */
    }
}

/*

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PageDiractoryEntry(pub u64);
impl PageDiractoryEntry {
    pub fn new(address: u64, flags: u64) -> Self { PageDiractoryEntry(address  | (flags << 12)) }
    pub fn get_flag(&self, flag: PageDiractoryEntryFlags) -> bool { (self.0 & ((flag as u64)  << 12)) >> ((flag as u64).trailing_zeros() + 12)== 1 }
    pub fn set_flag(&mut self, flag: PageDiractoryEntryFlags, val: bool) { self.0 |= (val as u64) << ((flag as u64 ).trailing_zeros() + 12); }
    pub fn get_address(&self) -> u64 { self.0 << 12 }
    pub fn set_address(&mut self, address: u64) { self.0 |= address << 12 }
}
#[repr(u64)]
#[derive(Clone, Copy)]
pub enum PageDiractoryEntryFlags {
    Present        = 1,
    ReadWrite      = 1 << 1,
    UserSupervisor = 1 << 2,
    WriteThrough   = 1 << 3,
    CacheDisable   = 1 << 4,
    Accessed       = 1 << 5,
    Available1     = 1 << 6,
    PageSize       = 1 << 7,
}
impl core::ops::BitOr for PageDiractoryEntryFlags {
    type Output = u64;
    fn bitor(self, rhs: PageDiractoryEntryFlags) -> Self::Output { self as Self::Output | rhs as Self::Output }
}

#[repr(C, align(0x1000))]
#[derive(Clone, Copy)]
pub struct PageTable {
    pub entries: [PageDiractoryEntry; 512],
}

pub struct PageMapIndexer {
    pub page_directory_pointer_index: u64,
    pub page_directory_index:         u64,
    pub page_table_index:             u64,
    pub page_index:                   u64,
}
impl PageMapIndexer {
    pub fn new(mut virtual_address: u64) -> Self {
        virtual_address >>= 12;
        let p_i = virtual_address & 0x1ff;
        virtual_address >>= 9;
        let pt_i = virtual_address & 0x1ff;
        virtual_address >>= 9;
        let pd_i = virtual_address & 0x1ff;
        virtual_address >>= 9;
        let pdp_i = virtual_address & 0x1ff;
        PageMapIndexer { page_directory_pointer_index: pdp_i, page_directory_index: pd_i, page_table_index: pt_i, page_index: p_i }
    }
}

pub struct PageTableManager {
    pub pml4_address: *mut PageTable,
}
impl PageTableManager {
    pub fn new(plm4: *mut PageTable) -> Self { PageTableManager { pml4_address: plm4 } }

    pub fn map_memory(&self, virtual_mem: u64, phys_mem: u64, con_out: &mut crate::console::Output, boot_info: &crate::boot::Info) -> bool {
        let page_map_indexer = PageMapIndexer::new(virtual_mem);
        let pde = unsafe{(*self.pml4_address).entries[page_map_indexer.page_directory_pointer_index as usize]};
        let pdp = if pde.get_flag(PageDiractoryEntryFlags::Present)  {
            pde.get_address()
        }
        /*
        pde = unsafe { *pdp }.entries[page_map_indexer.page_directory_index as usize];
        let pd: *mut PageTable = if pde.get_flag(PageDiractoryEntryFlags::Present) {
            pde.get_address() as *mut PageTable
        } else {
            unsafe {
                (*pdp).entries[page_map_indexer.page_directory_index as usize] = pde;
                let address = GLOBAL_ALLOC.get_free_page() as *mut PageTable;
                set_mem(address as u64, 0, 4096);
                address
            }
        };

        pde = unsafe { *pd }.entries[page_map_indexer.page_table_index as usize];
        let pt: *mut PageTable = if pde.get_flag(PageDiractoryEntryFlags::Present) {
            pde.get_address() as *mut PageTable
        } else {
            unsafe {
                (*pd).entries[page_map_indexer.page_table_index as usize] = pde;
                let address = GLOBAL_ALLOC.get_free_page() as *mut PageTable;
                //set_mem(address as u64, 0, 4096);
                address
            }
        };

        pde = unsafe { (*pt).entries[page_map_indexer.page_index as usize] };
        pde.set_address(phys_mem >> 12);
        pde.set_flag(PageDiractoryEntryFlags::Present, true);
        pde.set_flag(PageDiractoryEntryFlags::ReadWrite, true);
        unsafe { (*pt).entries[page_map_indexer.page_index as usize] = pde };

        */
        false
    }
}



*/
