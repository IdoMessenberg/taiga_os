#![no_std]

extern crate uefi as efi;
extern crate alloc as std_alloc;

pub mod elf;
pub mod psf;
pub mod config;