pub struct Point<T> {
    pub x: T,
    pub y: T,
}


pub struct Bitmap {
    pub size: usize,
    pub address: u64
}
impl Bitmap {
    pub fn new(size: usize, address: u64) -> Self{
        Bitmap { size, address }
    }
    pub fn set(&self, index: usize, value: bool) {
        if index > self.size-1 { return; }
        unsafe{
            core::ptr::write((self.address + (index / 8) as u64) as *mut u8, core::ptr::read_volatile((self.address + (index / 8) as u64) as *const u8) ^ (0b10000000 >> (index % 8)));
            if value {
                core::ptr::write((self.address + (index / 8) as u64) as *mut u8, core::ptr::read_volatile((self.address + (index / 8) as u64) as *const u8) | (0b10000000 >> (index % 8)));
            }
        }
    }
}
impl core::ops::Index<usize> for Bitmap {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        if unsafe { core::ptr::read_volatile((self.address + (index / 8) as u64) as *const u8) & (0b10000000 >> (index % 8)) > 0} {
            return &true;
        }
        &false
    }
}