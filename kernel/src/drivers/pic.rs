
use crate::arch::io_bus::{inb, outb, io_wait};

pub const PIC1_COMMAND: u16 = 0x20;
pub const PIC2_COMMAND: u16 = 0xa0;
pub const PIC1_DATA: u16 = 0x21;
pub const PIC2_DATA: u16 = 0xa1;
pub const PIC_EOI: u8 = 0x20;

const ICW1_INIT: u8 = 0x10;
const ICW1_ICW4: u8 = 0x01;
const ICW1_8086: u8 = 0x01;

pub fn remap_pic() {
    let a1: u8 = inb(PIC1_DATA);
    io_wait();
    let a2: u8 = inb(PIC2_DATA);
    io_wait();

    outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
    io_wait();
    outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
    io_wait();

    outb(PIC1_DATA, 0x20);
    io_wait();
    outb(PIC2_DATA, 0x28);
    io_wait();

    outb(PIC1_DATA, 4);
    io_wait();
    outb(PIC2_DATA, 2);
    io_wait();

    outb(PIC1_DATA, ICW1_8086);
    io_wait();
    outb(PIC2_DATA, ICW1_8086);
    io_wait();

    outb(PIC1_DATA, a1);
    io_wait();
    outb(PIC2_DATA, a2);
    io_wait();
}


pub fn pic_end_m(){
    outb(PIC1_COMMAND, PIC_EOI);
}

#[allow(unused)]
pub fn pic_end_s(){
    outb(PIC2_COMMAND, PIC_EOI);
    outb(PIC1_COMMAND, PIC_EOI);
}