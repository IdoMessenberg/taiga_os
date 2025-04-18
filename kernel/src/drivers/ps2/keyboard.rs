use core::arch::asm;
use crate::{arch::io_bus::{inb, io_wait, outb}, drivers::pic::{pic_end_m, remap_pic, PIC1_DATA, PIC2_DATA}, GLOBAL_TERMINAL};

const INPUT_PORT: u16 = 0x60;

pub fn init_ps2(){
    remap_pic();
    outb(PIC1_DATA, 0b11111101);
    outb(PIC2_DATA, 0b11111111);
    unsafe {
        asm!("sti")
    }
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(){
    let _ = inb(INPUT_PORT);
    io_wait();
    pic_end_m();
    GLOBAL_TERMINAL.get_mut().unwrap().put_string("hello");
}