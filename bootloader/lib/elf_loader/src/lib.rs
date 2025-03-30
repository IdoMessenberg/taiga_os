#![no_std]

use uefi as efi;
pub mod header;
pub use header::*;

#[cfg(feature = "loader")]
use uefi_tools::alloc::BootTimeAllocFunctions;

#[cfg(all(feature = "loader", target_arch = "x86_64"))]
pub fn load(system_table: &efi::system::Table, file: &[u8]) -> Result<usize, efi::Status> {
    let header = unsafe { &*(file.as_ptr() as *const Elf64Hdr) };
    if header.e_ident[EI_MAG_0 as usize] != ELF_MAG_0 || header.e_ident[EI_MAG_1 as usize] != ELF_MAG_1 || header.e_ident[EI_MAG_2 as usize] != ELF_MAG_2 || header.e_ident[EI_MAG_3 as usize] != ELF_MAG_3 {
        return Err(efi::Status::AccessDenied);
    }
    if header.e_ident[EI_CLASS as usize] != ELF_CLASS64 {
        return Err(efi::Status::DeviceError);
    }
    if header.e_ident[EI_DATA as usize] != ELF_DATA2_LSB {
        return Err(efi::Status::LoadError);
    }
    if header.e_ident[EI_VERSION as usize] != EV_CURRENT || header.e_version != EV_CURRENT as u32 {
        return Err(efi::Status::IncompatibleVersion);
    }
    if header.e_type != ET_EXEC as u16 {
        return Err(efi::Status::InvalidParameter);
    }
    if header.e_machine != EM_X86_64 as u16 {
        return Err(efi::Status::Unsupported);
    }
    let mut pheader_pointer: *const u8 = unsafe { file.as_ptr().offset(header.e_phoff as isize) };
    let mut pheader: &Elf64PHdr = unsafe { &*(pheader_pointer as *const Elf64PHdr) };
    for mut i in 0..header.e_phnum as u64 {
        if pheader.p_type == PT_LOAD as u32 {
            system_table.boot_time_services.alloc_pages(pheader.p_memsz as usize, &pheader.p_paddr);

            if pheader.p_filesz > 0 {
                system_table.boot_time_services.copy_mem(pheader.p_paddr as *const core::ffi::c_void, unsafe { file.as_ptr().offset(pheader.p_offset as isize) } as *const core::ffi::c_void, pheader.p_filesz as usize);
            }
            let diff: u64 = pheader.p_memsz - pheader.p_filesz;
            let start = (pheader.p_paddr + pheader.p_filesz) as *mut u8;
            let x = i;
            for _ in x..diff {
                unsafe { *(start.offset(i as isize)) = 0 }
                i += 1;
            }
        }

        pheader_pointer = unsafe { pheader_pointer.offset(header.e_phentsize as isize) };
        pheader = unsafe { &*(pheader_pointer as *const Elf64PHdr) }
    }
    Ok(header.e_entry as usize)
}

