//*/-kernel/src/main.rs
#![no_std]
#![no_main]

mod terminal;

///-the info from the bootloader is allign in efi format so the _start function needs to be an efi function -> extern "efiapi"
#[export_name = "_start"]
extern "efiapi" fn get_boot_info(boot_info: BootInfo) -> ! { main(boot_info) }

pub extern "C" fn main(boot_info: BootInfo) -> ! {
    let mut con_out: terminal::Output = terminal::Output::new(&boot_info.graphics, boot_info.font);
    con_out.init();
    con_out.println("hello world!");
    con_out.println(core::env!("CARGO_PKG_NAME"));
    con_out.println("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()_+=-\\|/?.,><;:'\"`~");
    unsafe{
        con_out.put_usize(&7824937328947);
    };
    panic!();
}

#[repr(C)]
pub struct BootInfo {
    graphics: GraphicsInfo,
    font:     FontInfo,
}

#[repr(C)]
struct FontInfo {
    pub glyph_char_size:           u8,
    pub glyph_buffer_base_address: *const core::ffi::c_void,
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
