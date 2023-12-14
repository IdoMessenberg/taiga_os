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
    let map_e = boot_info.mem_map_info.size / boot_info.mem_map_info.descriptor_size;
    unsafe{
        con_out.put_usize(&map_e);
        con_out.print("\r\n")
    }
    for i in 0..map_e {
        unsafe {
            let t = (*boot_info.mem_map_info.address.add(i * boot_info.mem_map_info.descriptor_size)).r#type as usize;
            con_out.put_usize(&t);
            con_out.print("-");
            if t < 17 {
                con_out.print(MEM_TYPES[t]);
            }
            con_out.print(" ");

            con_out.put_usize(&((*boot_info.mem_map_info.address.add(i * boot_info.mem_map_info.descriptor_size)).number_of_pages as usize * 4096 / 1024));
            con_out.print("\r\n");
        };
    }

    con_out.println("end");

    con_out.set_cursor_position(50, 30);
    con_out.print("<------this is a bug - the memory map should not look like that");
    panic!();
}

#[repr(C)]
pub struct BootInfo {
    graphics:     GraphicsInfo,
    font:         FontInfo,
    mem_map_info: MemMapInfo,
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

#[repr(C)]
struct MemMapInfo {
    pub size:                   usize,
    pub key:                    usize,
    pub descriptor_size:        usize,
    pub map_descriptor_version: u32,
    pub address:                *const MemoryDescriptor,
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
