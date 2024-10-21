#![no_std]
#![no_main]

use graphics::Functions;
use psf::RenderChar;

mod psf;
mod graphics;
mod memory;
mod bitmap;
mod page_frame_allocator;
mod virtual_memory;

static mut GLOBAL_ALLOC: page_frame_allocator::PageFrameAllocator = page_frame_allocator::PageFrameAllocator::default();

extern "C" {
    #[link_name = "__k_start_addr"]
    static mut _k_start: u8;
    #[link_name = "__k_end_addr"]
    static mut _k_end: u8;
}

#[no_mangle]
extern "efiapi" fn _start(boot_info: boot::Info) -> ! {main(boot_info)}

extern "C" fn main(boot_info: boot::Info) -> ! {
    let mut x_pos: u32 = 0;
    let mut y_pos: u32 = 0;

    let k_start: u64 = unsafe {core::ptr::addr_of!(_k_start)} as u64;
    let k_end: u64 = unsafe {core::ptr::addr_of!(_k_end)} as u64;
    unsafe { GLOBAL_ALLOC.init(&boot_info, k_start, k_end) };


    

    boot_info.graphics.clear_screen(boot_info.graphics.theme.black);
    for i in 0..unsafe {&GLOBAL_ALLOC.page_bitmap}.size {
        put_usize(&boot_info.font, &boot_info.graphics, &mut x_pos, &mut y_pos, boot_info.graphics.theme.white, boot_info.graphics.theme.black, &(unsafe {&GLOBAL_ALLOC.page_bitmap}[i] as usize));
    }

    panic!()
}
pub fn put_usize<T: RenderChar>(font: &T, graphics: &boot::efi::graphics::Info, x: &mut u32, y: &mut u32,  forground_color: u32, background_color: u32, num: &usize) {
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
        let _ = font.put_char(graphics , x, y, (b'0' + (temp / i) as u8) as char, forground_color, background_color);
        *x += 8;
        if *x > graphics.horizontal_resolution - 8 {
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