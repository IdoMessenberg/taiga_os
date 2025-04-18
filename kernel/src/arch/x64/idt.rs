
use core::arch::asm;
use util::{OnceLocStatus, OnceLock};
use crate::drivers::ps2::keyboard::keyboard_interrupt_handler;

const IDT_TA_INTERRUPT_GATE:u8 = 0b10001110;

#[repr(C, packed)]
struct IdtDescriptor {
    size: u16,
    offset: u64
}
impl IdtDescriptor {
    fn new(idt_ptr: *const GateDescriptor) -> Self {
        Self { size: 0x0fff, offset: idt_ptr as u64 }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct GateDescriptor {
    offset_0: u16,
    segment_selector: u16,
    ist_and_reserved_0: u8,
    type_attr: u8,
    offset_1: u16,
    offset_2: u32,
    reserved_1: u32
}
impl GateDescriptor {
    const fn empty() -> Self {
        Self { 
            offset_0: 0, 
            segment_selector: 0, 
            ist_and_reserved_0: 0, 
            type_attr: 0, 
            offset_1: 0, 
            offset_2: 0, 
            reserved_1: 0
        }
    }

    fn new(offset: u64, segment_selector: u16, type_attr: u8) -> Self {
        let (offset_0, offset_1, offset_2): (u16,u16,u32) = Self::cut_offset(offset);
        Self { 
            offset_0, 
            segment_selector,
            ist_and_reserved_0: 0, 
            type_attr, 
            offset_1, 
            offset_2, 
            reserved_1: 0
        }
    }

    fn cut_offset(offset: u64) -> (u16,u16,u32) {
        ((offset & 0x000000000000ffff) as u16,
        ((offset & 0x00000000ffff0000) >> 16) as u16,
        ((offset & 0xffffffff00000000) >> 16) as u32)
    }
}

static IDT: OnceLock<[GateDescriptor; 256]> = OnceLock::new();

pub fn load_idt() {
    match IDT.init(|| init_idt()) {
        OnceLocStatus::Success => {
            let idt_desc: IdtDescriptor = IdtDescriptor::new(unsafe {IDT.get_value_unchecked().as_ptr()});
            unsafe {
                asm!(
                    "lidt[{idt_desc}]",
                    idt_desc = in(reg) core::ptr::addr_of!(idt_desc) as u64
                )
            }
        },
        OnceLocStatus::InitErr => return
    }
}

fn init_idt() -> [GateDescriptor; 256] {
    let mut ret: [GateDescriptor; 256] = [GateDescriptor::empty(); 256];
    ret[0x8] = GateDescriptor::new(double_fault_handler as u64, 0x08, IDT_TA_INTERRUPT_GATE);
    ret[0xd] = GateDescriptor::new(general_protection_fault_handler as u64, 0x08, IDT_TA_INTERRUPT_GATE);
    ret[0xe] = GateDescriptor::new(page_fault_handler as u64, 0x08, IDT_TA_INTERRUPT_GATE);
    ret[0x21] = GateDescriptor::new(keyboard_interrupt_handler as u64, 0x08, IDT_TA_INTERRUPT_GATE);

    ret
}

#[repr(C, packed)]
struct InterruptFame{}

extern "x86-interrupt" fn double_fault_handler(_fame: *const InterruptFame) -> ! {
    todo!()
}

extern "x86-interrupt" fn general_protection_fault_handler(_fame: *const InterruptFame) -> ! {
    todo!()
}

extern "x86-interrupt" fn page_fault_handler(_fame: *const InterruptFame) -> ! {
    todo!()
}