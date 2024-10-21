pub trait Functions {
    unsafe fn put_pixel(&self, x: u32, y: u32, color: u32)  -> Result<(), Status>;
    fn clear_screen(&self, color: u32);
}
pub enum Status{
    Overflow
}

impl Functions for boot::efi::graphics::Info {
    unsafe fn put_pixel(&self, x: u32, y: u32, color: u32) -> Result<(), Status> {
        if x > self.horizontal_resolution || y > self.vertical_resolution {
            return Err(Status::Overflow)
        }
        core::ptr::write_volatile((self.frame_buffer_base_address + 4 * self.pixels_per_scan_line as u64 * y as u64 +4 * x as u64) as *mut u32, color);
        Ok(())
    }

    fn clear_screen(&self, color: u32) {
        for x in 0..self.horizontal_resolution  {
            for y in 0..self.vertical_resolution  {
                if unsafe{self.put_pixel(x, y, color)}.is_err() {break;}
            }
        }
    }
}