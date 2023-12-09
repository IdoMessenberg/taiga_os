//-Enviroment setup
#![no_std]
#![no_main]

extern crate efi_elf_loader as elf;
extern crate uefi as efi;

const KERNEL_FILE_NAME: *const u16 = [b'k' as u16, b'e' as u16, b'r' as u16, b'n' as u16, b'e' as u16, b'l' as u16, 0].as_ptr();
type KernelEntry = fn() -> usize;

#[export_name = "efi_main"]
extern "C" fn main(handle: *const core::ffi::c_void, system_table: efi::system::Table) -> efi::Status {
    efi::init(&system_table);
    let system_root = if let Ok(root) = efi::file::get_root(handle, &system_table) {
        root
    } else {
        return efi::Status::Aborted;
    };
    let kernel_entry_addr = {
        let kernel_file = if let Ok(kernel_file_handle) = system_root.load_file(KERNEL_FILE_NAME) {
            kernel_file_handle
        } else {
            return efi::Status::Aborted;
        };
        system_table.con_out.println_usize("kernel file size: {} bytes", kernel_file.len());
        if let Ok(entry) = elf::load_executable(&system_table, &kernel_file) {
            entry
        } else {
            return efi::Status::Aborted;
        }
    };
    let kernel_entry_point = unsafe { core::mem::transmute::<usize, KernelEntry>(kernel_entry_addr) };
    system_table.con_out.println_usize("entry: {}", kernel_entry_point());
    system_table.con_out.println("hello!");

    loop {}
    efi::Status::Success
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
