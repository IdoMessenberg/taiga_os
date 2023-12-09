#![no_std] #![no_main]

#[export_name = "_start"]
pub extern "C" fn main() -> usize {
    5300
}

#[panic_handler]
fn panic (_info: &core::panic::PanicInfo) -> ! {loop { unsafe{core::arch::asm!("hlt")}}}