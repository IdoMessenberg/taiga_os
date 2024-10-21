#![no_std]

extern crate alloc as std_alloc;

pub mod protocols;
pub mod alloc;
mod console;
pub mod file;
pub mod graphics;

pub use console::{Colour, str_to_ucs2};
pub use protocols::data_types::{Guid, Status, InputKey, ResetType};
pub use protocols::system;
pub use protocols::system_services::boot_time;