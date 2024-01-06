//*/-kernel/src/main.rs
#![no_std]
#![no_main]
///-the info from the bootloader is allign in efi format so the _start function needs to be an efi function -> extern "efiapi"
#[export_name = "_start"]
extern "efiapi" fn get_boot_info(boot_info: taiga::boot::Info) -> ! { main(boot_info) }

pub extern "C" fn main(boot_info: taiga::boot::Info) -> ! {
    let mut con_out: taiga::console::Output = taiga::console::Output::new(&boot_info);
    let page_frame_alloc : taiga::memory_paging::PageFrameAllocator = taiga::memory_paging::PageFrameAllocator::new(&boot_info);
    con_out.clear_screen();
    con_out.println("hello world!");
    con_out.println(core::env!("CARGO_PKG_NAME"));
    con_out.println("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()_+=-\\|/?.,><;:'\"`~");

    con_out.print("\r\n");

    con_out.print("total memory: ");
    con_out.put_usize(&(page_frame_alloc.total_mem as usize / 1024));
    con_out.println(" Kb");
    con_out.print("free memory: ");
    con_out.put_usize(&(page_frame_alloc.free_mem as usize / 1024));
    con_out.println(" kb");
    con_out.print("used memory: ");
    con_out.put_usize(&(page_frame_alloc.used_mem as usize / 1024));
    con_out.println(" kb");
    con_out.print("reserved memory: ");
    con_out.put_usize(&(page_frame_alloc.resv_mem as usize / 1024));
    con_out.println(" kb");

    con_out.println("end");
    panic!();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}

/*

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
 */