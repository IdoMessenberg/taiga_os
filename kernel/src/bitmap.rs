
#[derive(Clone, Copy)]
pub struct Bitmap {
    pub size: usize,
    pub addr: u64,
}

impl Bitmap {
    pub fn new(size: usize, addr: u64) -> Self {
        unsafe { core::ptr::write_bytes(addr as *mut u8, 0x0, size/8 + size % 8) ;}
        Bitmap { size, addr }
    }

    pub fn set(&self, index: usize, value: bool) -> bool {
        if index > self.size {
            return false;
        }
        if value {
            unsafe { core::ptr::write_volatile((self.addr + index as u64 / 8) as *mut u8, *((self.addr + index as u64 / 8) as *mut u8) | (0b10000000 >> (index % 8))) }
        }
        else {
            unsafe { core::ptr::write_volatile((self.addr + index as u64 / 8) as *mut u8, *((self.addr + index as u64 / 8) as *mut u8) ^ (0b10000000 >> (index % 8))) }
        }
        true
    }
}

impl core::ops::Index<usize> for Bitmap {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        let ptr = (self.addr + index as u64 / 8) as *const u8;
        match unsafe{*ptr} & 0b10000000 >> (index % 8) {
            0 => &false,
            _ => &true
        }
    }
}