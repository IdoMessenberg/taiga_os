use crate::{boot, console, data_types::Bitmap, memory_map};
pub struct PageFrameAllocator {
    pub total_mem:   u64,
    pub used_mem:    u64,
    pub free_mem:    u64,
    pub resv_mem:    u64,
    pub initialised: bool,
    pub page_bitmap: Bitmap,
    i : usize
}
impl PageFrameAllocator {
    pub const fn empty() -> Self { PageFrameAllocator { total_mem: 0, used_mem: 0, free_mem: 0, resv_mem: 0, initialised: false, page_bitmap: Bitmap { size: 0, address: 0 }, i: 0 } }
    pub fn initialise(&mut self, boot_info: &boot::Info) {
        if self.initialised {
            return;
        }
        self.initialised = true;
        self.i = 0;
        let mut largest_free_map_segment_addrss: u64 = 0;
        let mut largest_free_map_segment_size: usize = 0;

        for i in 0..(boot_info.mem_map_info.size / boot_info.mem_map_info.descriptor_size - 1) {
            let memory_map_segment_descriptor: &memory_map::Descriptor = unsafe { &*((boot_info.mem_map_info.address + i as u64 * boot_info.mem_map_info.descriptor_size as u64) as *const memory_map::Descriptor) };
            if memory_map_segment_descriptor.r#type != 7 {
                continue;
            }
            if memory_map_segment_descriptor.number_of_pages * 4096 > largest_free_map_segment_size as u64 {
                largest_free_map_segment_addrss = memory_map_segment_descriptor.physical_start;
                largest_free_map_segment_size = memory_map_segment_descriptor.number_of_pages as usize * 4096;
            }
        }

        self.total_mem = boot_info.mem_map_info.get_memory_size() as u64;
        self.free_mem = self.total_mem;
        self.set_page_bitmap(largest_free_map_segment_addrss);

        self.lock_pages(self.page_bitmap.address as *const Bitmap as u64, self.page_bitmap.size / 4096 + 1);
        self.lock_pages(boot_info.kernel_entry_address, boot_info.kernel_file_size / 4096 + 1);
        self.lock_pages(boot_info.font.glyph_buffer_base_address, 1);
        self.lock_pages(0x10000,  800);

        for i in 0..(boot_info.mem_map_info.size / boot_info.mem_map_info.descriptor_size ) {
            let memory_map_segment_descriptor: &memory_map::Descriptor = unsafe { &*((boot_info.mem_map_info.address + i as u64 * boot_info.mem_map_info.descriptor_size as u64) as *const memory_map::Descriptor) };
            if memory_map_segment_descriptor.r#type == 8 || memory_map_segment_descriptor.r#type == 9 || memory_map_segment_descriptor.r#type == 11 {
                self.reserve_pages(memory_map_segment_descriptor.physical_start, memory_map_segment_descriptor.number_of_pages as usize);
            }
        }
    }

    fn set_page_bitmap(&mut self, buffer_address: u64) {
        self.page_bitmap = Bitmap::new(self.total_mem as usize / 4096 / 8 + 1, buffer_address);
        for i in 0..(self.total_mem as usize / 4096 / 8 + 1) as u64 {
            unsafe { core::ptr::write_volatile((self.page_bitmap.address + i) as *mut u8, 0) }
        }
    }

    pub fn get_free_page(&mut self, con_out: &mut console::Output) -> Option<u64> {
        for i in 0..self.page_bitmap.size * 8 {
            if !self.page_bitmap[i] {
                /* 
                con_out.put_usize(&(i *4096));
                con_out.print("\t");
                if self.i % 14 ==  0{
                    con_out.print("\r\n");
                }
                */
                self.i+=1;
                self.lock_page(i as u64 * 4096);
                return Some(i as u64 * 4096);
            }
        }
        None
    }

    pub fn free_pages(&mut self, address: u64, pages: usize) {
        for i in 0..pages as u64 {
            self.free_page(address + i * 4096);
        }
    }

    pub fn lock_pages(&mut self, address: u64, pages: usize) {
        for i in 0..pages as u64 {
            self.lock_page(address + i * 4096);
        }
    }

    pub fn unreserve_pages(&mut self, address: u64, pages: usize) {
        for i in 0..pages as u64 {
            self.unreserve_page(address + i * 4096);
        }
    }

    pub fn reserve_pages(&mut self, address: u64, pages: usize) {
        for i in 0..pages as u64 {
            self.reserve_page(address + i * 4096);
        }
    }

    pub fn free_page(&mut self, address: u64) {
        if !self.page_bitmap[address as usize / 4096] {
            return;
        }
        if self.page_bitmap.set(address as usize / 4096, false){
            self.free_mem += 4096;
            self.used_mem -= 4096;
        }
    }

    pub fn lock_page(&mut self, address: u64) {
        if self.page_bitmap[address as usize / 4096] {
            return;
        }
        if self.page_bitmap.set(address as usize / 4096, true){
            self.free_mem -= 4096;
            self.used_mem += 4096;
        }
    }

    pub fn unreserve_page(&mut self, address: u64) {
        if !self.page_bitmap[address as usize / 4096] {
            return;
        }
        if self.page_bitmap.set(address as usize / 4096, false){
            self.free_mem += 4096;
            self.resv_mem -= 4096;
        }
    }

    pub fn reserve_page(&mut self, address: u64) {
        if self.page_bitmap[address as usize / 4096] {
            return;
        }
        if self.page_bitmap.set(address as usize / 4096, true){
            self.free_mem -= 4096;
            self.resv_mem += 4096;
        }
    }
}

pub fn set_mem(address: u64, value: u8, size: u64) {
    for i in 0..size {
        unsafe {
            core::ptr::write_volatile((address + i) as *mut u8, value);
        }
    }
}
