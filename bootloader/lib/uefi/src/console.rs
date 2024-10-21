use crate::protocols::{console_support::{simple_text_input, simple_text_output}, data_types::Status, system_services::boot_time};

impl simple_text_output::Protocol<'_> {
    //simple text output abstaction as functions
    pub fn clear_screen(&self) -> Status { (self.clear_screen)(self) }

    pub fn set_forground_colour(&self, colour: Colour) -> Status { (self.set_attribute)(self, colour as usize) }

    //console output functions
    fn put_char(&self, char: char) -> Status {
        if (self.test_string)(self, [char as u16, 0].as_ptr()).is_ok() {
            return (self.output_string)(self, [char as u16, 0].as_ptr())
        }
        Status::Unsupported
    }

    pub fn print(&self, string: &str) -> Status {
        for char in string.chars() {
            if !self.put_char(char).is_ok(){
                return Status::Unsupported;
            }
        }
        Status::Success
    }
     
    pub fn println(&self, string: &str) -> Status {
        if self.print(string).is_ok() && self.print("\r\n").is_ok() {
            return Status::Success
        }
        Status::Unsupported
    }

    pub fn log(&self, category: &str, colour: Colour, message: &str) -> Status {
        if 
            self.set_forground_colour(colour).is_ok() &&
            self.put_char('[').is_ok() &&
            self.print(category).is_ok() && 
            self.print("] ").is_ok() &&
            self.set_forground_colour(Colour::White).is_ok() &&
            self.println(message).is_ok() {
            return Status::Success
        }
        Status::Unsupported
    }
}

impl simple_text_input::Protocol {
    pub fn wait_for_key_input(&self, boot_time_services : &boot_time::Services) -> Status {
        (boot_time_services.wait_for_event)(1, &self.wait_for_key, core::ptr::null())
    }
}

pub fn str_to_ucs2(string: &str) -> std_alloc::vec::Vec<u16>{
    let mut  str: std_alloc::vec::Vec<u16> = string.encode_utf16().collect();
    str.push(0);
    str
}

#[repr(u8)]
pub enum Colour {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    White      = 15,
}