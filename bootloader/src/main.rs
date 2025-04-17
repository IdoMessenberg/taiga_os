#![no_std] #![no_main]
 
extern crate uefi as efi;
extern crate alloc as std_alloc;
use core::{ffi::c_void, mem::transmute, panic::PanicInfo, str::from_utf8};
use boot::{GraphicsInfo, TerminalTheme};
use std_alloc::{format, vec::Vec};
use uefi_tools::{alloc::{self, BootTimeAllocFunctions}, console::ConsoleOutputFunctions, file::{FileFunctions, Thing}, graphics::{BootTimeGraphicsFunctions, GraphicsFunctions}};
const _CONFIG_FILE_PATH: *const u16 = [b'c' as u16, b'o' as u16, b'n' as u16, b'f' as u16, b'i' as u16, b'g' as u16, b'.' as u16, b't' as u16, b'o' as u16, b'm' as u16, b'l' as u16, 0].as_ptr();
static mut __PANIC_CON_OUT__: *const efi::console_support::simple_text_output::Protocol = core::ptr::null();

type _KernelEntry = extern "C" fn(boot::_Info_) -> !;

#[unsafe(export_name = "efi_main")]
extern "C" fn main(handle: *const c_void, system_table: efi::system::Table<'static>) -> efi::Status {
    let verbose_boot_mode: bool = core::env!("VERBOSE_BOOT_MODE").parse().unwrap_or(false);
    init_console(&system_table);
    init_global_alloc(verbose_boot_mode, &system_table);
    let root: &efi::media_access::file::Protocol = get_system_root(verbose_boot_mode, handle, &system_table);
    let config_file: Vec<u8> = get_config_file(verbose_boot_mode, root, &system_table);
    let config_info: config_parser::Info = parse_config_file(verbose_boot_mode, &config_file, &system_table);
    let graphics_output_protocol: &efi::console_support::graphics_output::Protocol = match system_table.boot_time_services.get_graphics_output_protocol(){
        Ok(v) => v,
        Err(_) => panic!(),
    };
    set_resolution(verbose_boot_mode, &config_info, &graphics_output_protocol, &system_table);
    let colour_theme: config_parser::ColourTheme = get_colour_theme(verbose_boot_mode, &config_info, root, &system_table);
    let font_info: psf_loader::FontInfo = get_font_info(verbose_boot_mode, &config_info, root, &system_table);
    let _kernel_address: usize = loader_kernel(verbose_boot_mode, &config_info, root, &system_table);
    let memory_map: efi::data_types::MemoryMapInfo = get_mem_map(verbose_boot_mode, &system_table);
    
    match (system_table.boot_time_services.exit_boot_services)(handle, memory_map.key) {
        efi::Status::Success => (),
        err if verbose_boot_mode => print_error(&format!("could not exit boot time services! : {:?}", err), &system_table),
        _ => panic!("exit err")
    }
    let boot_info: boot::_Info_ = boot::_Info_{
        memory_map: memory_map,
        font: font_info,
        theme: TerminalTheme::from_config_colour_theme(colour_theme, config_info.graphics.dark_mode),
        graphics: GraphicsInfo::from_graphics_output_protocol_mode(graphics_output_protocol.mode)
    };
    let k_entry: _KernelEntry = unsafe { transmute::<usize,_KernelEntry>(_kernel_address)};
    k_entry(boot_info);
    #[allow(unreachable_code)]
    system_table.con_out.println("how did you get here?");
}

fn init_console(system_table: &efi::system::Table<'static>){
    unsafe {
        __PANIC_CON_OUT__ = system_table.con_out as *const efi::console_support::simple_text_output::Protocol;
    }
    system_table.con_out.reset();
    system_table.con_out.println("hello world!");
}
fn init_global_alloc(verbose_boot_mode: bool, system_table: &efi::system::Table) {
    match alloc::init(&system_table) {
        efi::Status::Success if verbose_boot_mode => {
            print_success("global allocator is initialized!..", system_table);
            system_table.con_out.println(&format!("global alloc: {:?}", alloc::BOOT_TIME_GLOBAL_ALLOCATOR));
        },
        efi::Status::Success => (),
        err if  verbose_boot_mode => print_error(&format!("global allocator is initialized! err message: {:?}", err), system_table),
        _ => panic!("alloc err")
    }
}
fn get_system_root<'a>(verbose_boot_mode: bool,handle: *const c_void,  system_table: &'a efi::system::Table) -> &'a efi::media_access::file::Protocol {
    match system_table.boot_time_services.get_root(handle){
        Ok(v) if verbose_boot_mode => {
            print_success("system root is found!..", system_table);
            v
        },
        Ok(v) => v,
        Err(e) if verbose_boot_mode => print_error(&format!("system root is not found! err message: {:?}", e), system_table),
        Err(_) => panic!("root err")
    }
}
fn get_config_file(verbose_boot_mode: bool, root: &efi::media_access::file::Protocol, system_table: &efi::system::Table) -> Vec<u8> {
    match root.get_file(_CONFIG_FILE_PATH,&system_table) {
        Ok(v) if verbose_boot_mode => {
            print_success("config file is found and loaded!..", system_table);
            system_table.con_out.println(&format!("config file: \r\n{}", &(from_utf8(&v).unwrap().trim())));
            v
        },
        Ok(v) => v,
        Err(e) if verbose_boot_mode => print_error(&format!("config file is not found! err message: {:?}", e), system_table),
        Err(_) => panic!("config err")
    }
}
fn parse_config_file<'a>(verbose_boot_mode: bool, config_file: &'a[u8], system_table: &efi::system::Table) -> config_parser::Info<'a> {
    match config_parser::read_config_file(&config_file){
        Ok(v) if verbose_boot_mode => {
            print_success("parsed config file!..", system_table);
            v
        },
        Ok(v) => v,
        Err(e) if verbose_boot_mode => print_error(&format!("config file parse err! err message: {}",e), &system_table),
        Err(_) => panic!()
    }
}
fn set_resolution(verbose_boot_mode: bool, config_info: &config_parser::Info, graphics_output_protocol: &efi::console_support::graphics_output::Protocol, system_table: &efi::system::Table) {
    let status: efi::Status = match config_info.graphics.resolution {
        config_parser::Resolution::_Hd => graphics_output_protocol.set_mode_to_resolution(1920, 1080),
        config_parser::Resolution::_2k => graphics_output_protocol.set_mode_to_resolution(2560, 1440),
        //config_parser::Resolution::_4k => {graphics_protocol.set_mode_to_resolution(3840, 2160 );} /currently not supported
        config_parser::Resolution::_Custom(width, hight) => {
            system_table.con_out.println(&format!("width:{}, hight:{}", width, hight));
            graphics_output_protocol.set_mode_to_resolution(width, hight)
        },
        _ => efi::Status::Success
    };
    match status {
        //todo:
        efi::Status::Success if verbose_boot_mode => (),
        efi::Status::Success => (),
        _err if verbose_boot_mode => (),
        _ => panic!("resolution not supported")
    }
}
fn get_colour_theme(verbose_boot_mode: bool, config_info: &config_parser::Info, root: &efi::media_access::file::Protocol, system_table: &efi::system::Table) -> config_parser::ColourTheme {
    let theme_file_path: Vec<u16> = format!("themes\\{}.toml\0", config_info.graphics.theme_path).encode_utf16().collect();
    let theme_file: Vec<u8> = match root.get_file(theme_file_path.as_ptr(), &system_table) {
        Ok(v) if verbose_boot_mode => {
            print_success("theme file is found and loaded!..", system_table);
            v
        },
        Ok(v) => v,
        Err(e) if verbose_boot_mode => print_error(&format!("theme file is not found! err message: {:?}", e), system_table),
        Err(_) => panic!()
    };
    match config_parser::read_theme_file(&theme_file){
        Ok(v) if verbose_boot_mode => {
            print_success("parsed theme file!..", system_table);
            v
        },
        Ok(v) => v,
        Err(e) if verbose_boot_mode => print_error(&format!("theme file parse err! err message: {}",e), system_table),
        Err(_) => panic!()
    }
}
fn get_font_info(verbose_boot_mode: bool, config_info: &config_parser::Info, root: &efi::media_access::file::Protocol, system_table: &efi::system::Table) -> psf_loader::FontInfo {
    let font_file_path: Vec<u16> = format!("fonts\\{}.psf\0", config_info.graphics.font_path).encode_utf16().collect();
    let font_file: Vec<u8> = match root.get_file(font_file_path.as_ptr(), system_table) {
        Ok(v) if verbose_boot_mode => {
            print_success("font file is found and loaded!..", system_table);
            v
        },
        Ok(v) => v,
        Err(e) if verbose_boot_mode => print_error(&format!("font file is not found! err message: {:?}", e), system_table),
        Err(_) => panic!()
    };

    match psf_loader::load_font(&system_table, &font_file){
        Ok(v) if verbose_boot_mode => {
            print_success("read font file!..", system_table);
            v
        },
        Ok(v) => v,
        Err(e) if verbose_boot_mode => print_error(&format!("font file read err! err message: {:?}", e), system_table),
        Err(_) => panic!()
    }
}
fn get_mem_map(verbose_boot_mode: bool, system_table: &efi::system::Table) -> efi::data_types::MemoryMapInfo {
    match system_table.boot_time_services.get_memory_map(){
        // can not have verbose success as calling print will change the map key
        Ok(v) => v,
        Err(e) if verbose_boot_mode => print_error(&format!("memory map is not found!! err message: {:?}",e), system_table),
        Err(_) => panic!("mem map err")
    }
}
fn loader_kernel(verbose_boot_mode: bool, config_info: &config_parser::Info, root: &efi::media_access::file::Protocol, system_table: &efi::system::Table) -> usize {
    let path: Vec<u16> = format!("{}\0",config_info.kernel.path).encode_utf16().collect();
    let kernel_file: Vec<u8> = match root.get_file(path.as_ptr(), &system_table) {
        Ok(v) if verbose_boot_mode => {
            print_success("kernel file is found!..", &system_table);
            v
        },
        Ok(v) => v,
        Err(e) if verbose_boot_mode => print_error(&format!("kernel file is not found! err message: {:?}", e), &system_table),
        Err(_) => panic!()
    };

    match elf_loader::load(&system_table, &kernel_file) {
        Ok(v) if verbose_boot_mode => {
            print_success("kernel file is loaded!..", &system_table);
            v
        },
        Ok(v) => v,
        Err(e) if verbose_boot_mode => print_error(&format!("kernel file is not loaded! err message: {:?}", e), &system_table),
        Err(_) => panic!("")
    }
}


fn print_error(message: &str, system_table: &efi::system::Table) -> ! {
    system_table.con_out.set_colour(efi::Colour::LightRed);
    system_table.con_out.print("[Err] ");
    system_table.con_out.set_colour(efi::Colour::White);
    system_table.con_out.println(message);
    panic!()
}
fn print_success(message: &str, system_table: &efi::system::Table) {
    system_table.con_out.set_colour(efi::Colour::LightGreen);
    system_table.con_out.print("[Ok] ");
    system_table.con_out.set_colour(efi::Colour::White);
    system_table.con_out.println(message);
}


#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    unsafe {
        (*__PANIC_CON_OUT__).println(&format!("{}",info.message()));
        loop{
            core::arch::asm!("hlt")
        }
    }
}