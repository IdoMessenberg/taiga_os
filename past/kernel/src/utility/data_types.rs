pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub struct Bitmap {
    pub size:    usize,
    pub address: u64,
}
impl Bitmap {
    pub fn new(size: usize, address: u64) -> Self { Bitmap { size, address } }
    pub fn set(&self, index: usize, value: bool) -> bool{
        if index > self.size * 8 {
            return false;
        }
        unsafe {
            core::ptr::write_volatile((self.address + (index / 8) as u64) as *mut u8, *((self.address + (index / 8) as u64) as *const u8) ^ (0b10000000 >> (index % 8)));
            if value {
                core::ptr::write_volatile((self.address + (index / 8) as u64) as *mut u8, *((self.address + (index / 8) as u64) as *const u8) | (0b10000000 >> (index % 8)));
            }
        }
        true
    }
}
impl core::ops::Index<usize> for Bitmap {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        /*
        if unsafe { core::ptr::read_volatile((self.address + (index / 8) as u64) as *const u8) & (0b10000000 >> (index % 8)) > 0 } {
            return &true;
        }
        &false
        */
        if index > self.size * 8 {
            return &false;
        }
        if unsafe {core::ptr::read_volatile((self.address + (index / 8) as u64) as *const u8) & (0b10000000 >> (index%8))} > 0
        {
            return &true;
        }
        &false
    }
}
