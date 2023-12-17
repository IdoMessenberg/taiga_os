//*/-kernel/src/main.rs
#![no_std]
#![no_main]

extern crate efi_graphics_output;
extern crate utility;
extern crate fonts;

mod terminal;

///-the info from the bootloader is allign in efi format so the _start function needs to be an efi function -> extern "efiapi"
#[export_name = "_start"]
extern "efiapi" fn get_boot_info(boot_info: BootInfo) -> ! { main(boot_info) }

pub extern "C" fn main(boot_info: BootInfo) -> ! {

    let mut con_out: terminal::Output = terminal::Output::new(&boot_info);
    con_out.clear_screen();
    con_out.println("hello world!");
    con_out.println(core::env!("CARGO_PKG_NAME"));
    con_out.println("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()_+=-\\|/?.,><;:'\"`~");
    let map_e = boot_info.mem_map_info.size / boot_info.mem_map_info.descriptor_size;
    con_out.put_usize(&map_e);
    con_out.print("\r\n");

    for i in 0..map_e {
        unsafe {
            let t = (core::ptr::read_volatile((boot_info.mem_map_info.address + i as u64 * boot_info.mem_map_info.descriptor_size as u64) as *const MemoryDescriptor).r#type) as usize;
            con_out.put_usize(&t);
            con_out.print("-");
            if t < 17 {
                con_out.print(MEM_TYPES[t]);
            }
            con_out.print(" ");

            con_out.put_usize(&(core::ptr::read_volatile((boot_info.mem_map_info.address + i as u64 * boot_info.mem_map_info.descriptor_size as u64) as *const MemoryDescriptor).number_of_pages as usize * 4096 / 1024));
            con_out.print("KiB\r\n");
        };
    }

    con_out.println("end");
    panic!();
}

#[repr(C)]
pub struct BootInfo {
    graphics:     efi_graphics_output::Info,
    font:         fonts::psf::Info,
    mem_map_info: MemMapInfo,
}

#[repr(C)]
struct MemMapInfo {
    pub address:                u64,
    pub size:                   usize,
    pub key:                    usize,
    pub descriptor_size:        usize,
    pub map_descriptor_version: u32,
}

#[repr(C)]
pub struct MemoryDescriptor {
    pub r#type:          u32,
    pub physical_start:  u64,
    pub virtual_start:   u64,
    pub number_of_pages: u64,
    pub attribute:       u64,
}

const MEM_TYPES: [&str; 17] = [
    "ReservedMemoryType",
    "LoaderCode",
    "LoaderData",
    "BootServicesCode",
    "BootServicesData",
    "RuntimeServicesCode",
    "RuntimeServicesData",
    "ConventionalMemory",
    "UnusableMemory",
    "ACPIReclaimMemory",
    "ACPIMemoryNVS",
    "MemoryMappedIO",
    "MemoryMappedIOPortSpace",
    "PalCode",
    "PersistentMemory",
    "UnacceptedMemoryType",
    "MaxMemoryType",
];

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}
