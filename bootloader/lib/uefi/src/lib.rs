#![no_std]

pub mod data_types;
pub mod loaded_image;
pub mod system;

pub mod console_support {
    pub mod graphics_output;
    pub mod simple_text_input;
    pub mod simple_text_output;
}

pub mod media_access {
    pub mod file;
    pub mod simple_file_system;
}
pub mod system_services {
    pub mod boot_time;
    pub mod run_time;
}

pub use data_types::{Guid, Status, InputKey, ResetType};
pub use system_services::boot_time;
pub use console_support::simple_text_output::Colour;