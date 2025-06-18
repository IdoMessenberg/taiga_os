#![no_std]#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc as std_alloc;

mod arch;
mod drivers;
mod mem;
mod log;
mod temp_graphics;
mod temp_terminal;

use core::{arch::asm, fmt::Write, panic::PanicInfo, ptr::addr_of};
use arch::{load_gdt, load_idt, paging};
use drivers::ps2::keyboard::init_ps2;
use log::log_nl;
use mem::page_frame_alloc::alloc::{GlobalPageAlloc, PageFrameAllocator};
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

#[global_allocator]
static GLOBAL_PAGE_FRAME_ALLOCATOR: GlobalPageAlloc = GlobalPageAlloc(util::OnceLock::new());
static GLOBAL_TERMINAL: util::OnceLock<temp_terminal::Terminal> = util::OnceLock::new();

fn k_main(boot_info: boot::_Info_) -> ! {
    load_gdt();
    log_nl("....loaded gdt");
    load_idt();
    log_nl("....loaded idt");
    init_ps2();
    log_nl("....init ps2 k");
    GLOBAL_PAGE_FRAME_ALLOCATOR.0.init(|| PageFrameAllocator::new(&boot_info));
    GLOBAL_TERMINAL.init(|| Terminal::new(&boot_info));
    GLOBAL_TERMINAL.get().unwrap().clear_screen();
    GLOBAL_TERMINAL.get_mut().unwrap().put_string("HEY!\r\n");



    GLOBAL_TERMINAL.get_mut().unwrap().put_string("HELO!\r\n");
    let mut ptm: paging::PageTableManager = paging::PageTableManager::new();
    for page in 0..boot_info.memory_map.get_pages() as u64 {
        ptm.map_page(page * 4096, page * 4096);
    }

    for page in (boot_info.graphics.framebuffer_base_address..boot_info.graphics.framebuffer_base_address + boot_info.graphics.framebuffer_size as u64).step_by(0x1000) {
        ptm.map_page(page, page);
    }

    unsafe {
        asm!(
            "mov cr3, {0}",
            in(reg) addr_of!(ptm.pml4) as u64
        )
    }

    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let _ = GLOBAL_TERMINAL.get_mut().unwrap().write_fmt(format_args!("Panic!: {}", info.message()));
    hlt_loop()
}

fn hlt_loop() -> ! {
    unsafe {
        loop {
            asm!("hlt");
        }
    }  
}