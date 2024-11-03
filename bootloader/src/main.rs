#![no_std]
#![no_main]

extern crate uefi as efi;
extern crate alloc as std_alloc;
use efi_functions as efif;
extern crate efi_file_loader as efl;

use efif::*;
use std_alloc::vec::Vec;


const CONFIG_FILE_PATH: *const u16 = [b'c' as u16, b'o' as u16, b'n' as u16, b'f' as u16, b'.' as u16, b't' as u16, b'o' as u16, b'm' as u16, b'l' as u16, 0].as_ptr();

#[repr(C)]
pub struct BootInfo {
    pub graphics:             efi::GraphicsInfo,
    pub memory_map_info:      efi::MemoryMapInfo,
    pub font:                 psf::FontInfo,
}

type KernelEntry = extern "efiapi" fn(BootInfo) -> !;

//todo: docs

#[export_name = "efi_main"]
extern "C" fn main(handle: *const core::ffi::c_void, system_table: efi::system::Table) -> efi::Status {
    init(&system_table);
    let system_root: &efi::media_access::file::Protocol = get_system_root(handle, &system_table);
    let gop: &efi::console_support::graphics_output::Protocol<'_> = if  let Ok(g) = system_table.boot_time_services.get_graphics_output_protocol(){g} 
        else {error(&system_table, "Err - graphics", "graphical output protocol is not found!")};
    //gop.set_mode_to_resulotion(1920, 1080);

    let config_file: Vec<u8> = match system_root.load_file(system_table.boot_time_services, CONFIG_FILE_PATH) {
        Ok(toml) => toml,
        Err(_) => error(&system_table, "Err - fs", "config file (conf.toml) is not found! - wrong path / not there!")
    };
    
    let config: efl::config::File = match efl::config::parse_config_toml(&config_file) {
        Ok(conf) => conf,
        Err(_) => error(&system_table, "Err - config", "incorect config file fromat!")
    };

    let kernel_entry_addr: usize = get_kernel_entry_point_addr(&system_table, system_root, &config);
    let font: psf::FontInfo = get_psf_font_info(&system_table, system_root, &config);
    let mut graphics_info: efi::GraphicsInfo = gop.get_graphics_info();
    graphics_info.theme = config.graphics_theme;

    let memory_map: efi::MemoryMapInfo = match system_table.boot_time_services.get_memory_map() {
        Ok(mem_map) => {
            mem_map
        },
        Err(_) => error(&system_table, "Err - mem", "memory map is not found!")
    };

    if !(system_table.boot_time_services.exit_boot_services)(handle, memory_map.key).is_ok() {
        error(&system_table, "Err - efi", "can not leave efi boot time!")
    }

    //safety: 
    // Yeah this shit is unsafe as fuck
    let kernel_entry_point: KernelEntry = unsafe { core::mem::transmute::<usize, KernelEntry>(kernel_entry_addr) };
    kernel_entry_point(BootInfo { graphics: graphics_info, font, memory_map_info: memory_map});

    #[allow(unreachable_code)]
    system_table.con_out.println("how did you get here?")
}


fn init(system_table: &efi::system::Table) {
    system_table.con_out.clear_screen();
    match efif::alloc::init(system_table) {
        efi::Status::Success => system_table.con_out.log("alloc",  efi::Colour::LightGreen, "bootloader allocator initialized successfully!"),
        efi::Status::NotFound => error(system_table,"Err - alloc",  "bootloader allocator was not initialized successfully! - Variables Were Not Found!"),
        _ => error(system_table,"Err - alloc",  "bootloader allocator was not initialized successfully! - Unknwon Err!"),
    };
}

fn get_system_root<'a>(handle: *const core::ffi::c_void, system_table: &'a efi::system::Table) -> &'a efi::media_access::file::Protocol{
    match efif::file::get_root(handle, system_table.boot_time_services) {
        Ok(root) => {
            system_table.con_out.log("fs", efi::Colour::LightGreen, "system root accessed!");
            root
        },
        Err(_) => error(system_table,"Err - fs", "efi file system is not found!"),
    }
}

fn get_kernel_entry_point_addr(system_table: &efi::system::Table, system_root: &efi::media_access::file::Protocol, config: &efl::config::File) -> usize {
    let kernel_file: Vec<u8> = match system_root.load_file(system_table.boot_time_services, efif::str_to_ucs2(config.loader_paths.kernel_path).as_ptr()) {
        Ok(elf) =>  elf,
        Err(_) => error(system_table, "Err - fs", "kernel file is not found! - wrong path / not there")
    };
    match efl::elf::load_executable(system_table, &kernel_file) {
        Ok(adrr) => adrr,
        Err(_) => error(system_table, "Err - elf", "kernel file could not be loaded")
    }
}

fn get_psf_font_info(system_table: &efi::system::Table, system_root: &efi::media_access::file::Protocol, config: &efl::config::File) -> psf::FontInfo {
    let font_file = match system_root.load_file(system_table.boot_time_services, efif::str_to_ucs2(config.loader_paths.font_path).as_ptr()) {
        Ok(psf) =>  psf,
        Err(_) => error(system_table, "Err - fs", "font file is not found! - wrong path / not there")
    };
    match efl::psf::load_font(system_table, &font_file) {
        Ok(info) => info,
        Err(_) => error(system_table, "Err - psf", "font file could not be loaded")
    }
}

fn error(system_table: &efi::system::Table, category: &str, message: &str) -> ! {
    system_table.con_out.log(category, efi::Colour::LightRed, message);
    system_table.con_out.println("press 'e' or 'esc' to exit or press 'r' to restart");
    let key: efi::InputKey = efi::InputKey{
        scan_code: 0,
        unicode_char:0
    };
    loop {
        system_table.con_in.wait_for_key_input(system_table.boot_time_services);

        if (system_table.con_in.read_key_stroke)(system_table.con_in, core::ptr::addr_of!(key)).is_ok() {
            if key.scan_code == 0x17 // 'esc' scan code
            || key.unicode_char == b'e' as u16 {
    
                (system_table.run_time_services.reset_system)(efi::ResetType::Shutdown, efi::Status::Aborted, 0); 
            }
            else if key.unicode_char ==  b'r' as u16 {
                
                (system_table.run_time_services.reset_system)(efi::ResetType::Cold, efi::Status::Aborted, 0);
            }
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