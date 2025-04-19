
use core::ptr::write_volatile;
use boot::GraphicsInfo;

pub trait FrameBufferTrait {
    unsafe fn put_pixel(&self, pos_x: u32, pos_y: u32, colour: u32);
    fn clear_screen(&self, colour: u32);
}
impl FrameBufferTrait for GraphicsInfo {
    unsafe fn put_pixel(&self, pos_x: u32, pos_y: u32, colour: u32) {
        if pos_x > self.horizontal_resolution || pos_y > self.vertical_resolution{
            return;
        }
        unsafe {
            write_volatile((self.framebuffer_base_address + 4 * self.pixels_per_scan_line as u64 * pos_y as u64 + pos_x as u64 * 4) as *mut u32, colour);
        }
    }
    fn clear_screen(&self, colour: u32) {
        for y in 0..self.vertical_resolution {
            for x in 0..self.horizontal_resolution {
                unsafe {
                    self.put_pixel(x,  y, colour);
                }
            }
        }
    }
}