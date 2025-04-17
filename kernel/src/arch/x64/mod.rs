
pub mod paging;
pub mod gdt;
pub mod idt;
pub mod io_bus;

pub use gdt::load_gdt;
pub use idt::load_idt;