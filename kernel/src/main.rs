//*/-kernel/src/main.rs
#![no_std]
#![no_main]

///-the info from the bootloader is allign in efi format so the _start function needs to be an efi function -> extern "efiapi"
#[export_name = "_start"]
extern "efiapi" fn get_boot_info(boot_info: BootInfo) -> ! { main(boot_info) }

pub extern "C" fn main(boot_info: BootInfo) -> ! {
    for x in 0..boot_info.graphics.horizontal_resolution {
        for y in boot_info.graphics.vertical_resolution / 2..boot_info.graphics.vertical_resolution {
            unsafe { core::ptr::write((boot_info.graphics.frame_buffer_base_address + 4 * boot_info.graphics.pixels_per_scan_line as u64 * y as u64 + 4 * x as u64) as *mut u64, 0x1e2120) };
        }
    }
    put_char(boot_info.graphics.frame_buffer_base_address, boot_info.graphics.pixels_per_scan_line, boot_info.font.glyph_buffer, boot_info.font.char_size, 'A');
    panic!();
}

fn put_char(frame_buffer_base_address: u64, pixels_per_scan_line: u32, glyph_buffer: *const core::ffi::c_void, char_size: u8, char: char) {
    unsafe {
        let mut char_ptr = glyph_buffer.add(char_size as usize * char as usize).cast::<u64>();

        for y in 50..68 {
            for x in 50..58 {
                if (*char_ptr & (0b10000000 >> (x - 50))) > 0 {
                    core::ptr::write_unaligned((frame_buffer_base_address + 4 * pixels_per_scan_line as u64 * y as u64 + 4 * x as u64) as *mut u32, 0xFFFFFFFF);
                }
                else {
                    core::ptr::write_unaligned((frame_buffer_base_address + 4 * pixels_per_scan_line as u64 * y as u64 + 4 * x as u64) as *mut u32, 0xFFF000FF);
  
                }
            }
            char_ptr = char_ptr.add(1);
        }
    };
}

#[repr(C)]
pub struct BootInfo {
    graphics: GraphicsInfo,
    font:     FontInfo,
}

#[repr(C)]
struct FontInfo {
    pub char_size:    u8,
    pub glyph_buffer: *const core::ffi::c_void,
}

#[repr(C)]
struct GraphicsInfo {
    pub frame_buffer_base_address: u64,
    pub frame_buffer_size:         usize,
    pub horizontal_resolution:     u32,
    pub vertical_resolution:       u32,
    pub pixels_per_scan_line:      u32,
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}
