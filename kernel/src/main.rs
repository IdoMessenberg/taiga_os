#![no_std] #![no_main]
use core::panic::PanicInfo;
extern crate boot;
use boot::_BootInfo_;

#[unsafe(no_mangle)]
extern "efiapi" fn _start(boot_info: _BootInfo_) -> ! {
    k_main(boot_info)
}

fn k_main(boot_info: _BootInfo_) -> ! {    
    for x in 0..boot_info.graphics.horizontal_resolution {
        for y in 0..boot_info.graphics.vertical_resolution {
            unsafe {
                core::ptr::write_volatile((boot_info.graphics.framebuffer_base_address + 4 * boot_info.graphics.pixels_per_scan_line as u64 * y as u64 + x as u64 * 4) as *mut u32, boot_info.theme.black);
            }
        }
    }
    panic!()
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    unsafe {
        loop{
            core::arch::asm!("hlt")
        }
    }
}