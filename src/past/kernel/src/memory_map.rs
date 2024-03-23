#[repr(C)]
pub struct Info {
    pub address:                u64,
    pub size:                   usize,
    pub key:                    usize,
    pub descriptor_size:        usize,
    pub map_descriptor_version: u32,
}

#[repr(C)]
pub struct Descriptor {
    pub r#type:          u32,
    pub physical_start:  u64,
    pub virtual_start:   u64,
    pub number_of_pages: u64,
    pub attribute:       u64,
}

impl Info {
    pub fn get_memory_size(&self) -> usize {
        let mut size: usize = 0;
        for i in 0..(self.size / self.descriptor_size-2) {
            unsafe { size += (*((self.address + i as u64 * self.descriptor_size as u64) as *const Descriptor)).number_of_pages as usize * 4096 };
        }
        size
    }
}
