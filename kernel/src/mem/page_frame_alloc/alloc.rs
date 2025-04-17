
use crate::{__k_start_addr__, __k_end_addr__, mem::page_frame_alloc::bitmap::Bitmap};
use boot::efi::data_types::{MemoryDescriptor, MemoryType};


pub struct PageFrameAllocator {
    pub pages: usize,
    pub free_pages: usize,
    pub used_pages: usize,
    //pub reserved_pages: usize,
    pub bitmap: Bitmap 
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
            //reserved_pages: 0, 
            bitmap: Bitmap::new( pages, largest_page_addr) 
        };
        let _ = res.iter_action( Self::alloc_page, 0, res.pages);
        
        for index in 0..boot_info.memory_map.size/boot_info.memory_map.descriptor_size - 1 {
            let descriptor: &MemoryDescriptor =  boot_info.memory_map.get_memory_descriptor(index as u64);
            if descriptor.r#type == MemoryType::ConventionalMemory {
                let _ = res.iter_action(Self::dealloc_page, descriptor.physical_start as usize, descriptor.number_of_pages as usize);
            }
        };
        let k_start: usize = &__k_start_addr__ as *const u8 as usize;
        let k_end: usize = &__k_end_addr__ as *const u8 as usize;
        let _ = res.iter_action(Self::alloc_page, k_start as usize, (k_end - k_start) as usize / Self::PAGE_SIZE + 1);
        let _ = res.iter_action(Self::alloc_page, res.bitmap.addr as usize, res.bitmap.capacity /8 / Self::PAGE_SIZE);
        let _ = res.iter_action(Self::alloc_page, boot_info.font.glyph_buffer_base_address as usize,  1);
        res
    }
    pub fn iter_action<T>(&mut self, action: T, start_addr: usize, pages: usize) -> Result<(), ()>
    where T : Fn(&mut Self, usize) -> Result<(),()>,
    {
        for i in 0..pages {
            match action(self, start_addr + i * Self::PAGE_SIZE) {
                Ok(_) => (),
                Err(_) => return Err(())
            } 
        }
        Ok(())
    }
    fn alloc_page(&mut self, addr: usize) -> Result<(), ()> {
        match self.bitmap[addr/Self::PAGE_SIZE] {
            None | Some(true) => {return Err(());},
            Some(_) => ()
        }
        self.free_pages -= 1;
        self.used_pages += 1;
        self.bitmap.set(addr/Self::PAGE_SIZE, true);
        Ok(())
    }
    fn dealloc_page(&mut self, addr: usize) -> Result<(), ()> {
        match self.bitmap[addr/Self::PAGE_SIZE] {
            None | Some(false) => {return Err(());},
            Some(_) => ()
        }
        self.free_pages += 1;
        self.used_pages -= 1;
        self.bitmap.set(addr/Self::PAGE_SIZE, false);
        Ok(())
    }
}