#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![allow(static_mut_refs)]

extern crate uefi as efi;
extern crate psf as psfont;
extern  crate memory_driver;

mod gdt;
mod idt;
mod terminal;

use gdt::*;
use graphics_deriver::Functions;
use terminal::GLOBAL_TERMINAL;

extern "C" {
    #[link_name = "__k_start_addr"]
    static mut _k_start: u8;
    #[link_name = "__k_end_addr"]
    static mut _k_end: u8;
}

#[no_mangle]
extern "efiapi" fn _start(boot_info: util::BootInfo) -> ! {main(boot_info)}


extern "C" fn main(boot_info: util::BootInfo) -> ! {
    let k_start: u64 = core::ptr::addr_of!(_k_start) as u64;
    let k_end: u64 = core::ptr::addr_of!(_k_end) as u64;

    unsafe {
        graphics_deriver::GLOBAL_FRAME_BUFFER = graphics_deriver::FrameBuffer::const_init(
            boot_info.graphics.frame_buffer_base_address,
            boot_info.graphics.pixels_per_scan_line,
            boot_info.graphics.horizontal_resolution,
            boot_info.graphics.vertical_resolution
        );
        load_gdt(Gdt::const_default());
        memory_driver::page_frame_allocator::GLOBAL_ALLOC.init(&boot_info, k_start, k_end);
         memory_driver::virtual_memory::init(&boot_info);
        idt::load_idt();
        terminal::GLOBAL_TERMINAL = terminal::Terminal::new(&boot_info, graphics_deriver::GLOBAL_FRAME_BUFFER);

        GLOBAL_TERMINAL.clear_screen();

    }

    unsafe {
        GLOBAL_TERMINAL.fg_colour = GLOBAL_TERMINAL.theme.red;
        GLOBAL_TERMINAL.put_num(&1);
        GLOBAL_TERMINAL.fg_colour = GLOBAL_TERMINAL.theme.green;
        GLOBAL_TERMINAL.put_num(&2);
        GLOBAL_TERMINAL.fg_colour = GLOBAL_TERMINAL.theme.blue;
        GLOBAL_TERMINAL.put_num(&3);
        GLOBAL_TERMINAL.fg_colour = GLOBAL_TERMINAL.theme.yellow;
        GLOBAL_TERMINAL.put_num(&4);
        GLOBAL_TERMINAL.fg_colour = GLOBAL_TERMINAL.theme.orange;
        GLOBAL_TERMINAL.put_num(&5);
        GLOBAL_TERMINAL.fg_colour = GLOBAL_TERMINAL.theme.purple;
        GLOBAL_TERMINAL.put_num(&6);
        GLOBAL_TERMINAL.fg_colour = GLOBAL_TERMINAL.theme.light_red;
        GLOBAL_TERMINAL.put_num(&7);
        GLOBAL_TERMINAL.fg_colour = GLOBAL_TERMINAL.theme.light_green;
        GLOBAL_TERMINAL.put_num(&8);
        GLOBAL_TERMINAL.fg_colour = GLOBAL_TERMINAL.theme.light_blue;
        GLOBAL_TERMINAL.put_num(&9);
        GLOBAL_TERMINAL.fg_colour = GLOBAL_TERMINAL.theme.light_yellow;
        GLOBAL_TERMINAL.put_num(&10);
        GLOBAL_TERMINAL.fg_colour = GLOBAL_TERMINAL.theme.light_orange;
        GLOBAL_TERMINAL.put_num(&11);
        GLOBAL_TERMINAL.fg_colour = GLOBAL_TERMINAL.theme.light_purple;
        GLOBAL_TERMINAL.put_num(&12);
        GLOBAL_TERMINAL.print("\r\n\n\t");
    }
    
    unsafe {
        
       // memory_driver::virtual_memory::PTM.map_memory(0x80000, 0x600000000);
    }
    let test :*mut usize = 0x600000000 as *mut usize;
    unsafe{
       // core::ptr::write_volatile(test, 4837589437589);
        //*test = 4837589437589;
       // GLOBAL_TERMINAL.put_num(&(*test));  
    };

    panic!()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        //safety: 
        // This should be safe cause if you reached here you already fucked up
        unsafe {
            // And this literally does nothing (tells the CPU to halt and do nothing)
            core::arch::asm!("hlt"); 
        }
    }
}