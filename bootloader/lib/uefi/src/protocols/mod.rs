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