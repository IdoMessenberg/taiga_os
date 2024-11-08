#![no_std]
#![feature(thread_local)]

extern crate page_frame_allocator as pfa;
pub use page_frame_allocator;
pub mod virtual_memory;