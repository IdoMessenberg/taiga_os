
#[repr(C, packed)]
pub struct GdtDiscriptor {
    pub size: u16,
    pub offset: u64
}

#[repr(C, packed)]
pub struct GdtEntry {
    limit_0: u16,
    base_0: u16,
    base_1: u8,
    access_byte: u8,
    limit_1_flags: u8,
    base_2: u8
}

#[repr(C, align(0x1000))]
pub struct Gdt {
    null: GdtEntry,
    kernel_code: GdtEntry,
    kernel_data: GdtEntry,
    user_null: GdtEntry,
    user_code: GdtEntry,
    user_data: GdtEntry
}
impl Gdt {
    pub const fn const_default() -> Self {
        Gdt { 
            null: 
                GdtEntry { limit_0: 0, base_0: 0, base_1: 0, access_byte: 0x00, limit_1_flags: 0x00, base_2: 0 }, 
            kernel_code: 
                GdtEntry { limit_0: 0, base_0: 0, base_1: 0, access_byte: 0x9a, limit_1_flags: 0xa0, base_2: 0 }, 
            kernel_data: 
                GdtEntry { limit_0: 0, base_0: 0, base_1: 0, access_byte: 0x92, limit_1_flags: 0xa0, base_2: 0 }, 
            user_null: 
                GdtEntry { limit_0: 0, base_0: 0, base_1: 0, access_byte: 0x00, limit_1_flags: 0x00, base_2: 0 }, 
            user_code: 
                GdtEntry { limit_0: 0, base_0: 0, base_1: 0, access_byte: 0x9a, limit_1_flags: 0xa0, base_2: 0 }, 
            user_data: 
                GdtEntry { limit_0: 0, base_0: 0, base_1: 0, access_byte: 0x92, limit_1_flags: 0xa0, base_2: 0 }
        }
    } 
}

pub unsafe fn load_gdt(gdt_descriptor_ptr: *const GdtDiscriptor) {
    core::arch::asm!(
        "lgdt [{r}]",
        "mov ax, 0x10",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
        "pop {r}",
        "mov rax, 0x08",
        "push rax",
        "push {r}",
        r = in(reg) gdt_descriptor_ptr
    )
}