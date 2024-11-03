use graphics_deriver::{Colour, Functions, GLOBAL_FRAME_BUFFER};


pub const IDT_TA_INTERRUPT_GATE:u8 = 0b10001110;
//pub const IDT_TA_CALL_GATE:u8 = 0b10001100;
//pub const IDT_TA_TRAP_GATE:u8 = 0b10001111;


#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtDescEntry {
    pub offset_0 : u16,
    pub selector: u16,
    pub ist: u8,
    pub type_attr: u8,
    pub offset_1: u16,
    pub offset_2: u32,
    pub ignore: u32,
}
impl IdtDescEntry {
    pub fn set_offset(&mut self, offset: u64) {
        self.offset_0 = (offset &  0x000000000000ffff) as u16;
        self.offset_1 = ((offset & 0x00000000ffff0000) >> 16) as u16;
        self.offset_2 = ((offset & 0xffffffff00000000) >> 32) as u32;
    }
    /* 
    pub fn get_offset(&self) -> u64 { 
        let mut offset: u64 = 0;
        offset |= self.offset_0 as u64;
        offset |= (self.offset_1 as u64) << 16;
        offset |= (self.offset_2 as u64) << 32;
        offset
    }
    */
}

#[repr(C, packed)]
pub struct Idtr {
    pub limit: u16,
    pub offset: u64
}

static mut IDTR: Idtr = Idtr {limit: 0x0fff, offset: 0};
static mut IDT: [IdtDescEntry; 256] = [
    IdtDescEntry {
        offset_0: 0,
        selector : 0,
        ist: 0,
        type_attr: 0,
        offset_1: 0,
        offset_2: 0,
        ignore: 0
    }; 
    256
];

pub unsafe fn load_idt() {
    IDTR.offset =  core::ptr::addr_of!(IDT) as u64;

    IDT[0xe] = IdtDescEntry{
        offset_0: 0,
        selector:0x08,
        ist:0,
        type_attr: IDT_TA_INTERRUPT_GATE,
        offset_1: 0,
        offset_2: 0,
        ignore: 0
    };
    IDT[0xe].set_offset(page_fault_handler as usize as u64);
    core::arch::asm!(
        "lidt [{r}]",
        r = in(reg) &IDTR,
        options(readonly, nostack, preserves_flags)
    )
}


#[repr(C, packed)]
pub struct InterruptFrame{}

pub unsafe extern "x86-interrupt" fn page_fault_handler(_frame: *const InterruptFrame) -> ! {
    GLOBAL_FRAME_BUFFER.clear_screen(&Colour::from_hex(0xffffffff));
    panic!()
}