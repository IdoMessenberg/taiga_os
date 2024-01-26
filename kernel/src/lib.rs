#![no_std]

pub mod boot;
pub mod console;
pub mod efi_graphics_output;
pub mod memory_map;
pub mod memory_paging;
pub mod page_map_indexer;
pub mod psf;
pub mod utility {
    pub mod data_types;
}
pub use utility::data_types;

pub static mut GLOBAL_ALLOC: memory_paging::PageFrameAllocator = memory_paging::PageFrameAllocator::empty();
