//*/-kernel/src/main.rs
#![no_std]
#![no_main]
///-the info from the bootloader is allign in efi format so the _start function needs to be an efi function -> extern "efiapi"
#[export_name = "_start"]
extern "efiapi" fn get_boot_info(boot_info: taiga::boot::Info) -> ! { main(boot_info) }

pub extern "C" fn main(boot_info: taiga::boot::Info) -> ! {

    let mut con_out: taiga::console::Output = taiga::console::Output::new(&boot_info);
    con_out.clear_screen();
    con_out.println("hello world!");
    con_out.println(core::env!("CARGO_PKG_NAME"));
    con_out.println("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()_+=-\\|/?.,><;:'\"`~");
    let map_e: usize = boot_info.mem_map_info.size / boot_info.mem_map_info.descriptor_size;
    let mut mem_size = 0;
    for i in 0..map_e -1{

        unsafe {
            mem_size += core::ptr::read_volatile((boot_info.mem_map_info.address + i as u64 * boot_info.mem_map_info.descriptor_size as u64) as *const taiga::memory::map::Descriptor).number_of_pages as usize * 4096;
        }
    }
    con_out.print("\r\n");
    con_out.put_usize(&(mem_size/1024/1024));

    con_out.println("end");
    panic!();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}
