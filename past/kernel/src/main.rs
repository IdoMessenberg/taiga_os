//*/-kernel/src/main.rs
#![no_std]
#![no_main]

///-the info from the bootloader is allign in efi format so the _start function needs to be an efi function -> extern "efiapi"
#[export_name = "_start"]
extern "efiapi" fn get_boot_info(boot_info: taiga::boot::Info) -> ! { main(boot_info) }
extern "C" {
    pub static _KernelStart: u8;
    pub static _KernelEnd: u8;
}
pub extern "C" fn main(boot_info: taiga::boot::Info) -> ! {

    let mut con_out: taiga::console::Output = taiga::console::Output::new(&boot_info);
    //let mut page_frame_alloc: taiga::memory_paging::PageFrameAllocator = taiga::memory_paging::PageFrameAllocator::new(&boot_info);
    con_out.clear_screen();
    unsafe { taiga::GLOBAL_ALLOC.initialise(&boot_info) };

    con_out.println("hello world!");
    con_out.println(core::env!("CARGO_PKG_NAME"));
    con_out.println("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()_+=-\\|/?.,><;:'\"`~");

    unsafe {
        con_out.print("total memory: ");
        con_out.put_usize(&(taiga::GLOBAL_ALLOC.total_mem as usize / 1024));
        con_out.println(" Kb");
        con_out.print("free memory: ");
        con_out.put_usize(&(taiga::GLOBAL_ALLOC.free_mem as usize / 1024));
        con_out.println(" kb");
        con_out.print("used memory: ");
        con_out.put_usize(&(taiga::GLOBAL_ALLOC.used_mem as usize / 1024));
        con_out.println(" kb");
        con_out.print("reserved memory: ");
        con_out.put_usize(&(taiga::GLOBAL_ALLOC.resv_mem as usize / 1024));
        con_out.println(" kb");
        //taiga::GLOBAL_ALLOC.print_bitmap(&mut con_out);
    }
    con_out.print("frame buffer addr: ");
        con_out.put_usize(&(boot_info.graphics.frame_buffer_base_address as usize));
        con_out.print("\r\n");


        /*
    for i in 0..20 {
        con_out.print("addrr: "); 
        con_out.put_usize(&i);
        con_out.print(" - ");
        con_out.put_usize(&unsafe {taiga::GLOBAL_ALLOC.get_free_page(&mut con_out).expect("no  free pages") as usize});
        con_out.print("\r\n");
    }
     
*/
    let pml4_address: u64 = unsafe { taiga::GLOBAL_ALLOC.get_free_page(&mut con_out).expect("no free pages") };
    taiga::memory_paging::set_mem(pml4_address, 0, 0x1000);
    let page_table_manager: taiga::page_map_indexer::PageTableManager = taiga::page_map_indexer::PageTableManager::new(pml4_address);
    for i in (0..boot_info.mem_map_info.get_memory_size()).step_by(4096) {
       //con_out.put_usize(&(i/ 0x1000));
        page_table_manager.map_memory(i as u64, i as u64, &mut con_out)
        //con_out.print(" ");
    
    } 
    let v = boot_info.graphics.frame_buffer_base_address;
    let s = boot_info.graphics.frame_buffer_size + 0x1000;
    //unsafe { taiga::GLOBAL_ALLOC.lock_pages(boot_info.graphics.frame_buffer_base_address, boot_info.graphics.frame_buffer_size as usize / 4096 + 1) };

    for i in (v..(s + v)).step_by(4096) {
        page_table_manager.map_memory(i, i, &mut con_out)
  
    }
    unsafe {
        core::arch::asm!("mov {}, cr3", in(reg)pml4_address);
    }

    con_out.println("hello from virtual mem");
    unsafe {
        con_out.print("total memory: ");
        con_out.put_usize(&(taiga::GLOBAL_ALLOC.total_mem as usize / 1024));
        con_out.println(" Kb");
        con_out.print("free memory: ");
        con_out.put_usize(&(taiga::GLOBAL_ALLOC.free_mem as usize / 1024));
        con_out.println(" kb");
        con_out.print("used memory: ");
        con_out.put_usize(&(taiga::GLOBAL_ALLOC.used_mem as usize / 1024));
        con_out.println(" kb");
        con_out.print("reserved memory: ");
        con_out.put_usize(&(taiga::GLOBAL_ALLOC.resv_mem as usize / 1024));
        con_out.println(" kb");
        //taiga::GLOBAL_ALLOC.print_bitmap(&mut con_out);
    }
    panic!()

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







    let page_diractory_entry: taiga::page_map_indexer::PageDiractoryEntry =
        taiga::page_map_indexer::PageDiractoryEntry::new(0b011111111111111111111111111111111111, taiga::page_map_indexer::PageDiractoryEntryFlags::PageSize | taiga::page_map_indexer::PageDiractoryEntryFlags::Present);

    for i in (0..64).rev() {
        con_out.put_usize(&(((page_diractory_entry.0 >> i) & 0x1) as usize))
    }
    con_out.print("\r\n");
    con_out.print(if page_diractory_entry.get_flag(taiga::page_map_indexer::PageDiractoryEntryFlags::ReadWrite) { "true" } else { "false" });
    con_out.print("\r\n");
    con_out.put_usize(&10);
    con_out.print("\r\n");

    let page_map_indexer: taiga::page_map_indexer::PageMapIndexer = taiga::page_map_indexer::PageMapIndexer::new(0x1000 * 52 + 0x50000 * 7);
    con_out.put_usize(&(page_map_indexer.page_index as usize));
    con_out.print(" - ");
    con_out.put_usize(&(page_map_indexer.page_table_index as usize));
    con_out.print(" - ");
    con_out.put_usize(&(page_map_indexer.page_directory_index as usize));
    con_out.print(" - ");
    con_out.put_usize(&(page_map_indexer.page_directory_pointer_index as usize));
 */
