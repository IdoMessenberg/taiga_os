#![no_std]

extern crate uefi as efi;
extern crate alloc as std_alloc;
extern crate efi_functions as efif;

pub mod elf;
pub mod psf;
pub mod config;