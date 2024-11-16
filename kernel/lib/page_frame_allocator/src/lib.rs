#![no_std]

mod bitmap;
extern crate uefi as efi;

use bitmap::Bitmap;
use efi::data_types::{MemoryDescriptor, MemoryType};

pub const PAGE_SIZE: usize = 0x1000;// 4096b -> 4kb
pub static mut GLOBAL_ALLOC: PageFrameAllocator = PageFrameAllocator::const_default();

pub struct PageFrameAllocator {
    pub total_mem: usize,
    pub free_mem: usize,
    pub used_mem: usize,
    pub reserved_mem: usize,
    pub page_bitmap: Bitmap,
    pub first_free_page_index: usize
}

//todo: impl err handeling
impl PageFrameAllocator {
    pub const fn const_default() -> Self {
        Self {
            total_mem: 0,
            free_mem: 0, 
            used_mem: 0, 
            reserved_mem: 0, 
            page_bitmap: Bitmap { size: 0, addr: 0 }, 
            first_free_page_index: 0
        }
    }
    
    pub fn init<'a>(&'a mut self, boot_info: &util::BootInfo, k_start: u64, k_end: u64) {
        let mut largest_free_map_segment_addr = 0;
        let mut largest_free_map_segment_size = 0;
        for i in 0..boot_info.memory_map_info.size/boot_info.memory_map_info.descriptor_size {
            let desc: &MemoryDescriptor = unsafe {&*((boot_info.memory_map_info.address + boot_info.memory_map_info.descriptor_size as u64 * i as u64) as *const MemoryDescriptor)};
            if desc.r#type != MemoryType::ConventionalMemory {
                continue;
            }
            if desc.number_of_pages > largest_free_map_segment_size {
                largest_free_map_segment_addr = desc.physical_start;
                largest_free_map_segment_size = desc.number_of_pages;
            }
        };

        self.total_mem = boot_info.memory_map_info.get_available_memory_bytes();
        self.free_mem = self.total_mem;
        self.used_mem = 0;
        self.reserved_mem = 0;
        self.page_bitmap = Bitmap::new(boot_info.memory_map_info.get_pages()/ 8 + 1, largest_free_map_segment_addr);
        self.first_free_page_index = 0;

        self.iter_action(Self::lock_page,self.page_bitmap.addr as usize, self.page_bitmap.size / crate::PAGE_SIZE + 1); 
        self.iter_action(Self::lock_page, k_start as usize, (k_end - k_start) as usize / crate::PAGE_SIZE + 1);
        self.iter_action(Self::lock_page, boot_info.graphics.frame_buffer_base_address as usize, boot_info.graphics.frame_buffer_size/ crate::PAGE_SIZE +1);
        self.lock_page(boot_info.font.glyph_buffer_base_address as  usize);

        for i in 0..boot_info.memory_map_info.size/boot_info.memory_map_info.descriptor_size {
            let desc: &MemoryDescriptor = unsafe {&*((boot_info.memory_map_info.address + boot_info.memory_map_info.descriptor_size as u64 * i as u64) as *const MemoryDescriptor)};
            if desc.r#type != MemoryType::ConventionalMemory && desc.r#type != MemoryType::LoaderData {
                let _ = self.iter_action(Self::reserve_page, desc.physical_start as usize, desc.number_of_pages as usize);
            }
        };
    }

    pub fn iter_action<T>(&mut self, action: T, start_addr: usize, pages: usize) -> bool
    where T : Fn(&mut Self, usize) -> bool,
    {
        for i in 0..pages {
            action(self, start_addr + i * crate::PAGE_SIZE);
        }
        true
    }
    
    pub fn get_free_page<'a>(&'a mut self) -> Option<u64> {
        for i in 0..self.page_bitmap.size {
            if !self.page_bitmap[i] {
               // self.first_free_page_index = i;
                self.lock_page(i * crate::PAGE_SIZE);
                unsafe {
                    core::ptr::write_bytes((i * crate::PAGE_SIZE) as *mut u8, 0, crate::PAGE_SIZE);
                }
                return Some((i* crate::PAGE_SIZE) as u64)
            }
        }
        None
    }

    pub fn lock_page(&mut self, page_addr: usize) -> bool {
        if self.page_bitmap[page_addr / crate::PAGE_SIZE] || !self.page_bitmap.set(page_addr / crate::PAGE_SIZE, true) {
            return false;
        }
        self.used_mem += crate::PAGE_SIZE;
        self.free_mem -= crate::PAGE_SIZE;
        true
    }
    
    /*
    pub fn unlock_page(&mut self, page_addr: usize) -> bool {
        if !self.page_bitmap[page_addr / boot::PAGE_SIZE] || !self.page_bitmap.set(page_addr / boot::PAGE_SIZE, false) {
            return false;
        }
        if page_addr / boot::PAGE_SIZE < self.first_free_page_index {
            self.first_free_page_index = page_addr / boot::PAGE_SIZE;
        }
        self.used_mem -= boot::PAGE_SIZE;
        self.free_mem += boot::PAGE_SIZE;
        true
    }
    */
    pub fn reserve_page (&mut self, page_addr: usize) -> bool {
        if self.page_bitmap[page_addr / crate::PAGE_SIZE] || !self.page_bitmap.set(page_addr / crate::PAGE_SIZE, true) {
            return false;
        }
        self.reserved_mem += crate::PAGE_SIZE;
        self.free_mem -= crate::PAGE_SIZE;
        true
    }
    /* 
    pub fn unreserve_page (&mut self, page_addr: usize) -> bool {
        if !self.page_bitmap[page_addr / boot::PAGE_SIZE] || !self.page_bitmap.set(page_addr / boot::PAGE_SIZE, false) {
            return false;
        }
        if page_addr / boot::PAGE_SIZE < self.first_free_page_index {
            self.first_free_page_index = page_addr / boot::PAGE_SIZE;
        }
        self.reserved_mem -= boot::PAGE_SIZE;
        self.free_mem += boot::PAGE_SIZE;
        true
    }
    */
}