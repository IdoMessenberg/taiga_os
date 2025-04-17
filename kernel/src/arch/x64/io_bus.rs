
use core::arch::asm;

#[inline]
pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(preserves_flags, nomem, nostack)
        )
    }
}

#[inline]
pub fn inb(port: u16) -> u8 {
    let mut ret: u8;
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") ret
        )
    }
    ret
}

pub fn io_wait() {
    let _ = inb(0x80);
}