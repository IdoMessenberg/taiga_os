use boot::efi::protocols::data_types::MemoryDescriptor;
pub trait GetPages {
    fn get_pages(&self) -> usize;
    fn get_available_memory_bytes(&self) -> usize;
}

impl GetPages for boot::efi::alloc::MemoryMapInfo {
    fn get_available_memory_bytes(&self) -> usize {
        self.get_pages() * boot::PAGE_SIZE
    }
    fn get_pages(&self) -> usize{
        let mut pages: usize = 0;
        for i in 0..self.size/self.descriptor_size {
            let desc: &MemoryDescriptor = unsafe {
                &*((self.address + self.descriptor_size as u64 * i as u64) as *const MemoryDescriptor)
            };
            pages += desc.number_of_pages as usize;
        };
        pages
    }
}