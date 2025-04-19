
//pub mod paging; //todo: :( this isn't working... again cr3 error again
pub mod gdt;
pub mod idt;
pub mod io_bus;

pub use gdt::load_gdt;
pub use idt::load_idt;