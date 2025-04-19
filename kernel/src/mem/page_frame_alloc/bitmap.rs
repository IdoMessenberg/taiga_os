
use core::{ops::Index, ptr::write_bytes};

pub struct Bitmap{
    pub capacity: usize, 
    pub addr: *mut u8
}
impl Bitmap {
    pub fn new(capacity: usize, addr: u64) -> Self {
        unsafe {
            write_bytes(addr as *mut u8, 0, capacity / 8 + 1);
        }
        Self { capacity, addr: addr as *mut u8 }
    }

    fn get(&self, index: usize) -> Option<bool> {
        if index >= self.capacity {
            return None;
        }
        let value_byte: u8 = unsafe { *(self.addr.add(index/8))};
        let value_mask: u8 = 0b10000000 >> (index % 8);
        Some(value_byte & value_mask != 0)
    }
    
    pub fn set(&mut self, index: usize, value: bool) -> bool {
        if index >= self.capacity {
            return false
        }
        let value_mask = 0b10000000 >> (index % 8);
        unsafe {
            if value {
                *(self.addr.add(index / 8)) |= value_mask;
            }
            else {
                *(self.addr.add(index / 8)) &= !value_mask;
            }
        }
        true
    }
}
impl Index<usize> for Bitmap {
    type Output = Option<bool>;

    fn index(&self, index: usize) -> &Self::Output {
        match self.get(index) {
            Some(true) => &Some(true),
            Some(false) => &Some(false),
            None => &None
        }
    }
}
impl core::fmt::Display for Bitmap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for index in 0..self.capacity {
            let _ = f.write_str(match self[index].unwrap() {
                true => "1",
                false => "0"
            });
        }
        Ok(())
    }
}