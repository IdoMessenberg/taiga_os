
use crate::{__k_start_addr__, __k_end_addr__, mem::page_frame_alloc::bitmap::Bitmap};
use boot::efi::data_types::{MemoryDescriptor, MemoryType};

pub struct PageFrameAllocator {
    pub pages: usize,
    pub free_pages: usize,
    pub used_pages: usize,
    pub reserved_pages: usize,
    pub bitmap: Bitmap 
}
pub enum PageFameAllocStatus {
    Success,
    AllocErr,
    //DeAllocErr,
    ReserveErr,
    UnREserveErr,
}
impl PageFrameAllocator {
    const PAGE_SIZE: usize = 4096;
    pub fn new(boot_info: &boot::_Info_) -> Self{
        let mut largest_page_addr = 0;
        let mut largest_page_size = 0;
        for index in 0..boot_info.memory_map.size / boot_info.memory_map.descriptor_size {
            let descriptor: &boot::efi::data_types::MemoryDescriptor = boot_info.memory_map.get_memory_descriptor(index as u64);
            if descriptor.r#type == boot::efi::data_types::MemoryType::ConventionalMemory &&
            descriptor.number_of_pages > largest_page_size {
                largest_page_size = descriptor.number_of_pages;
                largest_page_addr = descriptor.physical_start;
            }
        };

        let pages: usize = boot_info.memory_map.get_pages();

        let mut res = Self { 
            pages, 
            free_pages: pages, 
            used_pages: 0, 
            reserved_pages: 0, 
            bitmap: Bitmap::new( pages, largest_page_addr) 
        };
        let _ = res.iter_action( Self::reserve_page, 0, res.pages);
        
        for index in 0..boot_info.memory_map.size/boot_info.memory_map.descriptor_size - 1 {
            let descriptor: &MemoryDescriptor =  boot_info.memory_map.get_memory_descriptor(index as u64);
            if descriptor.r#type == MemoryType::ConventionalMemory {
                let _ = res.iter_action(Self::unreserve_page, descriptor.physical_start as usize, descriptor.number_of_pages as usize);
            }
        };
        let k_start: usize = &__k_start_addr__ as *const u8 as usize;
        let k_end: usize = &__k_end_addr__ as *const u8 as usize;
        let _ = res.iter_action(Self::alloc_page, k_start as usize, (k_end - k_start) as usize / Self::PAGE_SIZE + 1);
        let _ = res.iter_action(Self::alloc_page, res.bitmap.addr as usize, res.bitmap.capacity /8 / Self::PAGE_SIZE);
        let _ = res.iter_action(Self::alloc_page, boot_info.font.glyph_buffer_base_address as usize,  1);
        res
    }

    pub fn iter_action<T>(&mut self, action: T, start_addr: usize, pages: usize) -> Result<PageFameAllocStatus, (PageFameAllocStatus, usize)>
    where T : Fn(&mut Self, usize) -> PageFameAllocStatus,
    {
        for i in 0..pages {
            match action(self, start_addr + i * Self::PAGE_SIZE) {
                PageFameAllocStatus::Success => (),
                err => return Err((err, i))
            } 
        }
        Ok(PageFameAllocStatus::Success)
    }

    pub fn get_free_page(&mut self) -> Option<usize> {
        for index in 0..self.bitmap.capacity{
            if self.bitmap[index].unwrap_or(false) {
                continue;
            }
            self.alloc_page(index * Self::PAGE_SIZE);
            return Some(index * Self::PAGE_SIZE)
        }
        None
    }

    fn alloc_page(&mut self, addr: usize) -> PageFameAllocStatus {
        match self.bitmap[addr/Self::PAGE_SIZE] {
            None | Some(true) => {return PageFameAllocStatus::AllocErr;},
            Some(false) => ()
        }
        self.free_pages -= 1;
        self.used_pages += 1;
        self.bitmap.set(addr/Self::PAGE_SIZE, true);
        PageFameAllocStatus::Success
    }
    /*fn dealloc_page(&mut self, addr: usize) -> PageFameAllocStatus {
        match self.bitmap[addr/Self::PAGE_SIZE] {
            None | Some(false) => {return PageFameAllocStatus::DeAllocErr;},
            Some(_) => ()
        }
        self.free_pages += 1;
        self.used_pages -= 1;
        self.bitmap.set(addr/Self::PAGE_SIZE, false);
        PageFameAllocStatus::Success
    }*/

    fn reserve_page(&mut self, addr: usize) -> PageFameAllocStatus {
        match self.bitmap[addr/Self::PAGE_SIZE] {
            None | Some(true) => {return PageFameAllocStatus::ReserveErr;},
            Some(false) => ()
        }
        self.free_pages -= 1;
        self.reserved_pages += 1;
        self.bitmap.set(addr/Self::PAGE_SIZE, true);
        PageFameAllocStatus::Success
    }
    fn unreserve_page(&mut self, addr: usize) -> PageFameAllocStatus {
        match self.bitmap[addr/Self::PAGE_SIZE] {
            None | Some(false) => {return PageFameAllocStatus::UnREserveErr;},
            Some(true) => ()
        }
        self.free_pages += 1;
        self.reserved_pages -= 1;
        self.bitmap.set(addr/Self::PAGE_SIZE, false);
        PageFameAllocStatus::Success
    }
}