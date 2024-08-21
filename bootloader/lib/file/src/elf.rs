//*/-bootloader/src/elf.rs
//https://github.com/torvalds/linux/blob/master/include/uapi/linux/elf.h
const EI_NIDENT: u8 = 16;

const EI_MAG_0: u8 = 0;
const EI_MAG_1: u8 = 1;
const EI_MAG_2: u8 = 2;
const EI_MAG_3: u8 = 3;

const EI_CLASS: u8 = 4;
const EI_DATA: u8 = 5;
const EI_VERSION: u8 = 6;

const ELF_MAG_0: u8 = 0x7f;
const ELF_MAG_1: u8 = b'E';
const ELF_MAG_2: u8 = b'L';
const ELF_MAG_3: u8 = b'F';

const ELF_CLASS64: u8 = 2;
const ELF_DATA2_LSB: u8 = 1;
const EV_CURRENT: u8 = 1;
const ET_EXEC: u8 = 2;

const EM_X86_64: u8 = 62;

const PT_LOAD: u8 = 1;

///https://github.com/torvalds/linux/blob/master/include/uapi/linux/elf.h#L226
#[repr(C)]
struct Elf64Hdr {
    pub e_ident:     [core::ffi::c_uchar; EI_NIDENT as usize],
    pub e_type:      u16,
    pub e_machine:   u16,
    pub e_version:   u32,
    pub e_entry:     u64,
    pub e_phoff:     u64,
    pub e_shoff:     u64,
    pub e_flags:     u32,
    pub e_ehsize:    u16,
    pub e_phentsize: u16,
    pub e_phnum:     u16,
    pub e_shentsize: u16,
    pub e_shnum:     u16,
    pub e_shstrndx:  u16,
}

///https://github.com/torvalds/linux/blob/master/include/uapi/linux/elf.h#L260
#[repr(C)]
struct Elf64PHdr {
    pub p_type:   u32,
    pub p_flags:  u32,
    pub p_offset: u64,
    pub p_vaddr:  u64,
    pub p_paddr:  u64,
    pub p_filesz: u64,
    pub p_memsz:  u64,
    pub p_align:  u64,
}

pub fn load_executable(system_table: &efi::system::Table, file: &[u8]) -> Result<usize, efi::Status> {
    let header = unsafe { &*(file.as_ptr() as *const Elf64Hdr) };
    if header.e_ident[EI_MAG_0 as usize] != ELF_MAG_0 || header.e_ident[EI_MAG_1 as usize] != ELF_MAG_1 || header.e_ident[EI_MAG_2 as usize] != ELF_MAG_2 || header.e_ident[EI_MAG_3 as usize] != ELF_MAG_3 {
        return Err(efi::Status::Aborted);
    }
    if header.e_ident[EI_CLASS as usize] != ELF_CLASS64 {
        return Err(efi::Status::Aborted);
    }
    if header.e_ident[EI_DATA as usize] != ELF_DATA2_LSB {
        return Err(efi::Status::Aborted);
    }
    if header.e_ident[EI_VERSION as usize] != EV_CURRENT || header.e_version != EV_CURRENT as u32 {
        return Err(efi::Status::Aborted);
    }
    if header.e_type != ET_EXEC as u16 {
        return Err(efi::Status::Aborted);
    }
    if header.e_machine != EM_X86_64 as u16 {
        return Err(efi::Status::Aborted);
    }
    let mut pheader_pointer: *const u8 = unsafe { file.as_ptr().offset(header.e_phoff as isize) };
    let mut pheader: &Elf64PHdr = unsafe { &*(pheader_pointer as *const Elf64PHdr) };
    for mut i in 0..header.e_phnum as u64 {
        if pheader.p_type == PT_LOAD as u32 {
            system_table.boot_time_services.allocate_pages(pheader.p_memsz as usize, &pheader.p_paddr);

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
    system_table.con_out.println_status("kernel - Loaded Kernel ELF File Into Memory!", efi::Status::Success);
    Ok(header.e_entry as usize)
}
