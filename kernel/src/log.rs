
use crate::arch::io_bus::outb;

const QEMU_SERIAL_OUT: u16 = 0x3f8;

pub fn log(str: &str) {
    for char in str.chars() {
        outb(QEMU_SERIAL_OUT, char as u8);
    }
}

pub fn log_nl(str: &str) {
    log(str);
    log("\r\n");
}