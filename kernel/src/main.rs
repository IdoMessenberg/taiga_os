#![no_std]
#![no_main]

use graphics::Functions;
use memory::GetPages;
use psf::RenderChar;
use virtual_memory::*;

mod psf;
mod graphics;
mod memory;
mod bitmap;
mod page_frame_allocator;
mod virtual_memory;

static mut GLOBAL_ALLOC: page_frame_allocator::PageFrameAllocator = page_frame_allocator::PageFrameAllocator::const_default();

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
        
    let pml4: *mut PageTable = unsafe { GLOBAL_ALLOC.get_free_page().unwrap()} as *mut PageTable; 
    unsafe {
        core::ptr::write_bytes(pml4, 0, boot::PAGE_SIZE);
    }
    
    let ptm = PageTableManager{pml4};
    
    for i in (0..boot_info.memory_map_info.get_available_memory_bytes() as u64).step_by(boot::PAGE_SIZE) {
        ptm.map_memory(i, i);
    }
    for i in (boot_info.graphics.frame_buffer_base_address..(boot_info.graphics.frame_buffer_base_address + boot_info.graphics.frame_buffer_size as u64)).step_by(boot::PAGE_SIZE) {
        ptm.map_memory(i, i);
    }
    unsafe{
        core::arch::
        asm!("mov {x} , cr3", 
              x  = in(reg) pml4);
    }

    boot_info.graphics.clear_screen(boot_info.graphics.theme.black);
    //put_usize(&boot_info.font, &boot_info.graphics, &mut x_pos, &mut y_pos, boot_info.graphics.theme.white, boot_info.graphics.theme.black, &(1000));

    ptm.map_memory(0x80000, 0x60000000);
    let test :*mut usize = 0x60000000 as *mut usize;
    unsafe{*test = 4837589437589};
    put_usize(&boot_info.font, &boot_info.graphics, &mut x_pos, &mut y_pos, boot_info.graphics.theme.white, boot_info.graphics.theme.black, &(unsafe{*test}));

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