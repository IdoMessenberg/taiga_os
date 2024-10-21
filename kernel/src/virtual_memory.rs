
#[repr(C)]
pub struct PageDiractoryEntry(u64);
impl PageDiractoryEntry {
    
}

#[repr(C, align(0x1000))]
pub struct PageTable([PageDiractoryEntry; 512]);