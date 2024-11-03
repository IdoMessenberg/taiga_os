#![no_std]

extern crate alloc as std_alloc;

pub mod alloc;
mod console;
pub mod file;
mod graphics;

pub use alloc::BootTimeAllocFunctions;
pub use console::{ConsoleInputFunctions, ConsoleOutputFunctions, str_to_ucs2};
pub use graphics::{BootTimeGraphicsFunctions, GraphicsFunctions};
pub use file::FileFunctions;