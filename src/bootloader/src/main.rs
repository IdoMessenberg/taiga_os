//*/-bootloader/src/main.rs
//-Enviroment setup
#![no_std]
#![no_main]

extern crate alloc as std_alloc;
extern crate uefi as efi;
extern crate file;


const KERNEL_FILE_NAME: *const u16 = [b'k' as u16, b'e' as u16, b'r' as u16, b'n' as u16, b'e' as u16, b'l' as u16, 0].as_ptr();
const KERNEL_FONT_FILE_NAME: *const u16 = [b'f' as u16, b'o' as u16, b'n' as u16, b't' as u16, b'.' as u16, b'p' as u16, b's' as u16, b'f' as u16, 0].as_ptr();

type KernelEntry = extern "efiapi" fn(BootInfo);

#[repr(C)]
struct BootInfo {
    graphics:             efi::graphics::Info,
    font:                 file::psf::FontInfo,
    memory_map_info:      efi::alloc::MemoryMapInfo,
    kernel_entry_address: u64,
    kernel_size:          usize,
}

#[export_name = "efi_main"]
extern "efiapi" fn main(handle: *const core::ffi::c_void, system_table: efi::system::Table) -> efi::Status {
    efi::init(&system_table, core::env!("CARGO_PKG_NAME"), core::env!("CARGO_PKG_AUTHORS"), core::env!("CARGO_PKG_VERSION"));
    let system_root: &efi::protocols::media_access::file::Protocol = if let Ok(root) = efi::file::get_root(handle, system_table.boot_time_services) { root } else { return efi::Status::Aborted };
    
    let kernel_size: usize;
    let kernel_entry_address: usize = {
        let kernel_file: std_alloc::vec::Vec<u8> = if let Ok(kernel_file_vec) = system_root.load_file(KERNEL_FILE_NAME) { kernel_file_vec } else { return efi::Status::Aborted };
        system_table.con_out.println_usize("[ KERNEL ] - File Size - {}KiB", &(kernel_file.len() / 1024));
        kernel_size = kernel_file.len();
        if let Ok(entry) =  file::elf::load_executable(&system_table, &kernel_file) {
            entry
        } else {
            return efi::Status::Aborted;
        }
    };
    system_table.con_out.println_usize("[ KERNEL ] - addrr - {}", &(kernel_entry_address));
    let font:  file::psf::FontInfo = {
        let font_file: std_alloc::vec::Vec<u8> = if let Ok(font_file_vec) = system_root.load_file(KERNEL_FONT_FILE_NAME) {
            font_file_vec
        } else {
            return efi::Status::Aborted;
        };
        if let Ok(font_info) =  file::psf::load_font(&system_table, &font_file) {
            font_info
        } else {
            return efi::Status::Aborted;
        }
    };
    let graphics_info: efi::graphics::Info = if let Ok(info) = system_table.boot_time_services.get_graphics_info() { info } else { return system_table.con_out.println_status("Graphics - Could Not Get Graphics Info!", efi::Status::Aborted) };
    let memory_map: efi::alloc::MemoryMapInfo = if let Ok(mem_map) = system_table.boot_time_services.get_memory_map() { mem_map } else { return system_table.con_out.println_status("Memory Map - Could Not Get The Memory Map!", efi::Status::Aborted) };
    if (system_table.boot_time_services.exit_boot_services)(handle, memory_map.key).is_err() {
        return system_table.con_out.println_status("Boot - Could Not Exit Boot Services!", efi::Status::Aborted);
    }
    //safety: yeah this shit is unsafe as fuck
    let kernel_entry_point: KernelEntry = unsafe { core::mem::transmute::<usize, KernelEntry>(kernel_entry_address) };
    kernel_entry_point(BootInfo { graphics: graphics_info, font, memory_map_info: memory_map, kernel_entry_address: kernel_entry_address as u64, kernel_size });
    panic!()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
