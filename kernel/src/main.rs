#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![allow(static_mut_refs)]

extern crate uefi as efi;
extern crate psf as psfont;
extern  crate memory_driver;
use gdt::*;
use graphics_deriver::{Functions, PsfFontFunctions, PutPixel};

mod gdt;
mod idt;

pub const PAGE_SIZE: usize = 4096;

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
    }
    unsafe {
        graphics_deriver::GLOBAL_FRAME_BUFFER.put_pixel(0,0,&graphics_deriver::Colour::from_hex(boot_info.graphics.theme.black));
        graphics_deriver::GLOBAL_FRAME_BUFFER.clear_screen(&graphics_deriver::Colour::from_hex(boot_info.graphics.theme.black));

    }
    
    let mut x_pos: u32 = 0;
    let mut y_pos: u32 = 300;
    put_usize(&boot_info.font, &mut x_pos, &mut y_pos, boot_info.graphics.theme.white, boot_info.graphics.theme.black, &(1));
    put_usize(&boot_info.font,  &mut x_pos, &mut y_pos, boot_info.graphics.theme.red, boot_info.graphics.theme.black, &(2));
    put_usize(&boot_info.font, &mut x_pos, &mut y_pos, boot_info.graphics.theme.green, boot_info.graphics.theme.black, &(3));
    put_usize(&boot_info.font, &mut x_pos, &mut y_pos, boot_info.graphics.theme.blue, boot_info.graphics.theme.black, &(4));
    put_usize(&boot_info.font, &mut x_pos, &mut y_pos, boot_info.graphics.theme.yellow, boot_info.graphics.theme.black, &(5));
    put_usize(&boot_info.font, &mut x_pos, &mut y_pos, boot_info.graphics.theme.orange, boot_info.graphics.theme.black, &(6));
    put_usize(&boot_info.font, &mut x_pos, &mut y_pos, boot_info.graphics.theme.purple, boot_info.graphics.theme.black, &(7));
    put_usize(&boot_info.font,  &mut x_pos, &mut y_pos, boot_info.graphics.theme.gray, boot_info.graphics.theme.black, &(8));
    put_usize(&boot_info.font,  &mut x_pos, &mut y_pos, boot_info.graphics.theme.dark_gray, boot_info.graphics.theme.black, &(9));
    put_usize(&boot_info.font, &mut x_pos, &mut y_pos, boot_info.graphics.theme.light_red, boot_info.graphics.theme.black, &(10));
    put_usize(&boot_info.font,  &mut x_pos, &mut y_pos, boot_info.graphics.theme.light_green, boot_info.graphics.theme.black, &(11));
    put_usize(&boot_info.font,  &mut x_pos, &mut y_pos, boot_info.graphics.theme.light_blue, boot_info.graphics.theme.black, &(12));
    put_usize(&boot_info.font, &mut x_pos, &mut y_pos, boot_info.graphics.theme.light_yellow, boot_info.graphics.theme.black, &(13));
    put_usize(&boot_info.font, &mut x_pos, &mut y_pos, boot_info.graphics.theme.light_orange, boot_info.graphics.theme.black, &(14));
    put_usize(&boot_info.font, &mut x_pos, &mut y_pos, boot_info.graphics.theme.light_purple, boot_info.graphics.theme.black, &(15));
    let mut  c : graphics_deriver::Colour = graphics_deriver::Colour::from_hex(boot_info.graphics.theme.orange);
    c.alpha =50;
    let mut c2 = graphics_deriver::Colour::from_hex(boot_info.graphics.theme.blue);
    c2.alpha = 80;

    y_pos = 316;
    x_pos = 0;
    unsafe {
        //core::arch::asm!("int 0xe");
        graphics_deriver::GLOBAL_FRAME_BUFFER.put_char(&boot_info.font, 100, 80, 'A', &graphics_deriver::Colour::from_hex(boot_info.graphics.theme.light_blue), &graphics_deriver::Colour::from_hex(boot_info.graphics.theme.black));
        graphics_deriver::GLOBAL_FRAME_BUFFER.clear_screen_with_alpha_correction(&c2);
        graphics_deriver::GLOBAL_FRAME_BUFFER.clear_screen_with_alpha_correction(&c);

    }
    unsafe {
        
        memory_driver::virtual_memory::PTM.map_memory(0x80000, 0x60000000);
    }
        let test :*mut usize = 0x60000000 as *mut usize;
    unsafe{
        *test = 4837589437589;
        put_usize(&boot_info.font, &mut x_pos, &mut y_pos, boot_info.graphics.theme.light_purple, boot_info.graphics.theme.black, &(*test));
    };
    panic!()
}

pub fn put_usize(font: &psf::FontInfo, x: &mut u32, y: &mut u32,  forground_color: u32, background_color: u32, num: &usize) {
    let mut i: usize = 1;
    if i <= num / 10 {
        for _ in 0..17 {
            i *= 10;
            if i > num / 10 {
                break;
            }
        }
    }

    let mut temp: usize = *num;
    for _ in 0..17 {
        unsafe {
            graphics_deriver::GLOBAL_FRAME_BUFFER.put_char(font , *x, *y, (b'0' + (temp / i) as u8) as char, &graphics_deriver::Colour::from_hex(forground_color), &graphics_deriver::Colour::from_hex(background_color))
        };
        *x += 8;
        if *x > unsafe {
            graphics_deriver::GLOBAL_FRAME_BUFFER.horizontal_resolution - 8  
        }  {
            *x = 0;
            *y += 18
        }
        temp %= i;
        i /= 10;
        if i == 0 {
            break;
        }
    }
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