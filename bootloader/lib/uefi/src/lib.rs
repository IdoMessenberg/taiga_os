//*/-bootloader/lib/uefi/src/lib.rs
//-Enviroment setup
#![no_std]

extern crate alloc as std_alloc;

pub mod protocols {
    pub mod data_types;
    pub mod loaded_image;
    pub mod system;

    pub mod console_support {
        pub mod graphics_output;
        pub mod simple_text_output;
    }

    pub mod media_access {
        pub mod file;
        pub mod simple_file_system;
    }
    pub mod system_services {
        pub mod boot_time;
    }
}

pub mod alloc;
mod console;
pub mod file;
pub mod graphics;

pub use console::Colour;
pub use protocols::data_types::{Guid, Status};
pub use protocols::system;
pub use protocols::system_services::boot_time;

pub fn init(system_table: &system::Table) {
    if graphics::init(system_table).is_err() {
        return;
    }
    if console::init(system_table).is_err() {
        return;
    }
    if alloc::init(system_table).is_err() {
        return;
    }
}
