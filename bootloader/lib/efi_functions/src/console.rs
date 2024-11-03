use uefi::{console_support::{simple_text_input, simple_text_output}, data_types::Status, system_services::boot_time};

pub trait ConsoleOutputFunctions {
    fn clear_screen(&self) -> Status;
    fn set_forground_colour(&self, colour: uefi::Colour) -> Status;
    fn put_char(&self, char: char) -> Status;
    fn print(&self, string: &str) -> Status;
    fn println(&self, string: &str) -> Status;
    fn log(&self, category: &str, colour: uefi::Colour, message: &str) -> Status;
}
pub trait ConsoleInputFunctions {
    fn wait_for_key_input(&self, boot_time_services : &boot_time::Services) -> Status;
}

impl ConsoleOutputFunctions for simple_text_output::Protocol<'_> {
    //simple text output abstaction as functions
    fn clear_screen(&self) -> Status { (self.clear_screen)(self) }

    fn set_forground_colour(&self, colour: uefi::Colour) -> Status { (self.set_attribute)(self, colour as usize) }

    //console output functions
    fn put_char(&self, char: char) -> Status {
        if (self.test_string)(self, [char as u16, 0].as_ptr()).is_ok() {
            return (self.output_string)(self, [char as u16, 0].as_ptr())
        }
        Status::Unsupported
    }

    fn print(&self, string: &str) -> Status {
        for char in string.chars() {
            if !self.put_char(char).is_ok(){
                return Status::Unsupported;
            }
        }
        Status::Success
    }
     
    fn println(&self, string: &str) -> Status {
        if self.print(string).is_ok() && self.print("\r\n").is_ok() {
            return Status::Success
        }
        Status::Unsupported
    }

    fn log(&self, category: &str, colour: uefi::Colour, message: &str) -> Status {
        if 
            self.set_forground_colour(colour).is_ok() &&
            self.put_char('[').is_ok() &&
            self.print(category).is_ok() && 
            self.print("] ").is_ok() &&
            self.set_forground_colour(uefi::Colour::White).is_ok() &&
            self.println(message).is_ok() {
            return Status::Success
        }
        Status::Unsupported
    }
}

impl ConsoleInputFunctions for simple_text_input::Protocol {
    fn wait_for_key_input(&self, boot_time_services : &boot_time::Services) -> Status {
        (boot_time_services.wait_for_event)(1, &self.wait_for_key, core::ptr::null())
    }
}

pub fn str_to_ucs2(string: &str) -> std_alloc::vec::Vec<u16>{
    let mut  str: std_alloc::vec::Vec<u16> = string.encode_utf16().collect();
    str.push(0);
    str
}