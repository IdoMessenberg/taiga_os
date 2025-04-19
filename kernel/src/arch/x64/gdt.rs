
use core::{arch::asm, ptr::addr_of};

#[repr(C, packed)]
struct GdtDescriptor {
    size: u16,
    offset: u64
}
impl GdtDescriptor {
    fn new(gdt_ptr: *const Gdt) -> Self {
        Self { size: core::mem::size_of::<Gdt>() as u16 - 1, offset: gdt_ptr as u64 }
    }
}

#[repr(C, packed)]
pub struct SegmentDescriptor{
    limit_0: u16,
    base_0: u16,
    base_1: u8,
    access_byte: u8,
    limit_and_flags: u8,
    base_2: u8
}
impl SegmentDescriptor {
    pub const NULL: Self = Self::new(0, 0);
    pub const KERNEL_CODE: Self = Self::new( 0x9a, 0xa0);
    pub const KERNEL_DATA: Self = Self::new(0x92, 0xa0);
    pub const USER_CODE: Self = Self::new(0x9a, 0xa0);
    pub const USER_DATA: Self = Self::new(0x92, 0xa0);

    const fn new(access_byte: u8, limit_and_flags: u8) -> Self {
        Self { 
            limit_0: 0, 
            base_0: 0, 
            base_1: 0, 
            access_byte, 
            limit_and_flags, 
            base_2: 0
        }
    }
}

#[repr(C, align(0x1000))]
pub struct Gdt{
    null: SegmentDescriptor,
    kernel_code: SegmentDescriptor,
    kernel_data: SegmentDescriptor,
    user_code: SegmentDescriptor,
    user_data: SegmentDescriptor,
}
impl Gdt {
    pub const fn const_default() -> Self {
        Self { 
            null: SegmentDescriptor::NULL,
            kernel_code: SegmentDescriptor::KERNEL_CODE,
            kernel_data: SegmentDescriptor::KERNEL_DATA,
            user_code: SegmentDescriptor::USER_CODE,
            user_data: SegmentDescriptor::USER_DATA,
        }
    } 
}

pub static GDT: Gdt = Gdt::const_default();

pub fn load_gdt() {
    let gdt_desc: GdtDescriptor = GdtDescriptor::new(addr_of!(GDT));
    unsafe {
        asm!(
            "lgdt[{gdt_desc}]",
            // moving kernel code segment index to the cs reg
            "mov cs, {kernel_code_index:x}",

            // moving kernel data segment index to the other regs
            "mov dx, {kernel_data_index:x}",
            "mov ds, {kernel_data_index:x}",
            "mov es, {kernel_data_index:x}",
            "mov fs, {kernel_data_index:x}",
            "mov gs, {kernel_data_index:x}",
            "mov ss, {kernel_data_index:x}",
            gdt_desc = in(reg) addr_of!(gdt_desc) as u64,
            kernel_code_index = in(reg) 0x08,
            kernel_data_index = in(reg) 0x10
        )
    }
}