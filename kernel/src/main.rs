#![no_std]#![no_main]
#![feature(abi_x86_interrupt)]

mod arch;
mod drivers;
mod mem;
mod log;
mod temp_graphics;
mod temp_terminal;

use core::{arch::asm, panic::PanicInfo, ptr::{write_bytes, NonNull}};
use arch::{load_gdt, load_idt};
use drivers::ps2::keyboard::init_ps2;
use log::log_nl;
use mem::page_frame_alloc::alloc::PageFrameAllocator;
use temp_terminal::Terminal;

unsafe extern "C" {
    #[link_name = "__k_start_addr__"]
    pub safe static __k_start_addr__: u8;
    #[link_name = "__k_end_addr__"]
    pub safe static __k_end_addr__: u8;
}

// for some reason the in boot info from the bootloader is passed
// with the "efiapi" abi so the starting function needs to be formatted with this abi
#[unsafe(no_mangle)]
extern "efiapi" fn _start(boot_info: boot::_Info_) -> ! {
    log_nl("....moving to k main");
    k_main(boot_info)
}

static GLOBAL_PAGE_FRAME_ALLOCATOR: util::OnceLock<PageFrameAllocator> = util::OnceLock::new();
static GLOBAL_TERMINAL: util::OnceLock<temp_terminal::Terminal> = util::OnceLock::new();

fn k_main(_boot_info: boot::_Info_) -> ! {
    load_gdt();
    log_nl("....loaded gdt");
    load_idt();
    log_nl("....loaded idt");
    init_ps2();
    log_nl("....init ps2 k");
    GLOBAL_PAGE_FRAME_ALLOCATOR.init(|| PageFrameAllocator::new(&_boot_info));
    GLOBAL_TERMINAL.init(|| Terminal::new(&_boot_info));
    GLOBAL_TERMINAL.get().unwrap().clear_screen();


    GLOBAL_TERMINAL.get_mut().unwrap().put_string("in virtual mem");
    loop {}
}   



#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        loop {
            asm!("hlt");
        }
    }
}