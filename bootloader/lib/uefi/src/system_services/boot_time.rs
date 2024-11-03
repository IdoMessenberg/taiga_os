use crate::{
    data_types::{AllocateType, Guid, MemoryDescriptor, MemoryType, Status}, system
};
use core::ffi::c_void;

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=171
#[repr(C)]
pub struct Services {
    pub hdr:                               system::TableHeader,
    raise_tpl:                             extern "efiapi" fn() -> Status,
    restore_tpl:                           extern "efiapi" fn() -> Status,
    pub allocate_pages:                    extern "efiapi" fn(r#type: AllocateType, memory_type: MemoryType, pages: usize, memory_adrr: *const u64) -> Status,
    free_pages:                            extern "efiapi" fn() -> Status,
    pub get_memory_map:                    extern "efiapi" fn(memory_map_size: *const usize, memory_map: *const MemoryDescriptor, map_key: *const usize, descriptor_size: *const usize, descriptor_version: *const u32) -> Status,
    pub allocate_pool:                     AllocatePool,
    pub free_pool:                         FreePool,
    create_event:                          extern "efiapi" fn() -> Status,
    set_timer:                             extern "efiapi" fn() -> Status,
    pub wait_for_event:                    extern "efiapi" fn(number_of_events: usize, event: &*const c_void, index: *const usize) -> Status,
    signal_event:                          extern "efiapi" fn() -> Status,
    close_event:                           extern "efiapi" fn() -> Status,
    check_event:                           extern "efiapi" fn() -> Status,
    install_protocol_interface:            extern "efiapi" fn() -> Status,
    reinstall_protocol_interface:          extern "efiapi" fn() -> Status,
    uninstall_protocol_interface:          extern "efiapi" fn() -> Status,
    pub handle_protocol:                   extern "efiapi" fn(handle: *const c_void, protocol: *const Guid, interface: *const *const c_void) -> Status,
    reserved:                              *const c_void,
    register_protocol_notify:              extern "efiapi" fn() -> Status,
    locate_handle:                         extern "efiapi" fn() -> Status,
    locate_device_path:                    extern "efiapi" fn() -> Status,
    install_configuration_table:           extern "efiapi" fn() -> Status,
    load_image:                            extern "efiapi" fn() -> Status,
    start_image:                           extern "efiapi" fn() -> Status,
    exit:                                  extern "efiapi" fn() -> Status,
    unload_image:                          extern "efiapi" fn() -> Status,
    pub exit_boot_services:                extern "efiapi" fn(image_handle: *const c_void, map_key: usize) -> Status,
    get_next_montonic_count:               extern "efiapi" fn() -> Status,
    stall:                                 extern "efiapi" fn() -> Status,
    set_watchdog_timer:                    extern "efiapi" fn() -> Status, //
    connect_controller:                    extern "efiapi" fn() -> Status,
    disconnect_controller:                 extern "efiapi" fn() -> Status,
    open_protocol:                         extern "efiapi" fn() -> Status,
    close_protocol:                        extern "efiapi" fn() -> Status,
    open_protocol_information:             extern "efiapi" fn() -> Status,
    protocols_per_handle:                  extern "efiapi" fn() -> Status,
    locate_handle_buffer:                  extern "efiapi" fn() -> Status,
    pub locate_protocol:                   extern "efiapi" fn(protocol: *const Guid, registration: *const c_void, interface: *const *const c_void) -> Status,
    install_multiple_protocol_interafes:   extern "efiapi" fn() -> Status,
    uninstall_multiple_protocol_interface: extern "efiapi" fn() -> Status,
    calculate_crc_32:                      extern "efiapi" fn() -> Status,
    pub copy_mem:                          extern "efiapi" fn(destination: *const c_void, source: *const c_void, length: usize) -> c_void,
}

pub type AllocatePool = extern "efiapi" fn(pool_type: MemoryType, size: usize, buffer: *const *const c_void) -> Status;
pub type FreePool = extern "efiapi" fn(buffer: *const c_void) -> Status;
