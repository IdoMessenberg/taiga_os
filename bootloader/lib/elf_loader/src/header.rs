pub const EI_NIDENT: u8 = 16;

pub const ELF_MAG_0: u8 = 0x7f;
pub const ELF_MAG_1: u8 = b'E';
pub const ELF_MAG_2: u8 = b'L';
pub const ELF_MAG_3: u8 = b'F';

pub const EI_MAG_0: u8 = 0;
pub const EI_MAG_1: u8 = 1;
pub const EI_MAG_2: u8 = 2;
pub const EI_MAG_3: u8 = 3;

pub const ELF_CLASS64: u8 = 2;
pub const ELF_DATA2_LSB: u8 = 1;
pub const EV_CURRENT: u8 = 1;
pub const ET_EXEC: u8 = 2;

pub const EI_CLASS: u8 = 4;
pub const EI_DATA: u8 = 5;
pub const EI_VERSION: u8 = 6;

pub const EM_X86_64: u8 = 62;
pub const EM_AARCH64: u8 = 183;
pub const EM_RISCV: u8 = 243;

pub const PT_LOAD: u8 = 1;

///https://github.com/torvalds/linux/blob/master/include/uapi/linux/elf.h#L226
#[repr(C)]
pub struct Elf64Hdr {
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
pub struct Elf64PHdr {
    pub p_type:   u32,
    pub p_flags:  u32,
    pub p_offset: u64,
    pub p_vaddr:  u64,
    pub p_paddr:  u64,
    pub p_filesz: u64,
    pub p_memsz:  u64,
    pub p_align:  u64,
}
