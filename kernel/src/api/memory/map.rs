
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