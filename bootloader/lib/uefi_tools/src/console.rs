use uefi::{console_support::simple_text_output, data_types::Status};

trait _BaseConsoleFunctions {
    fn put_char(&self, char: char) -> Result<(),Status>;
    fn print_string(&self, str: &str) -> Result<(),Status>;
}
pub trait ConsoleOutputFunctions{
    fn print(&self, str: &str);
    fn println(&self, str: &str);
    fn set_colour(&self, colour: simple_text_output::Colour);
    fn reset(&self);
    fn clear_screen(&self);
}

impl<'a> _BaseConsoleFunctions for simple_text_output::Protocol<'a> {
    fn put_char(&self, char: char) -> Result<(),Status> {
        match (self.test_string)(self, [char as u16, 0u16].as_ptr()) {
            Status::Success => {(self.output_string)(self, [char as u16, 0u16].as_ptr());}
            err => {return Err(err)}
        }
        Ok(())
    }
    
    fn print_string(&self, str: &str) -> Result<(),Status> {
        for char in str.chars() {
            match self.put_char(char) {
                Ok(()) => (),
                Err(e) => {return Err(e)}
            }
        }
        Ok(())
    }
}

impl<'a> ConsoleOutputFunctions for simple_text_output::Protocol<'a> {
    fn print(&self, str: &str){
        let _ = self.print_string(str);
    }
    fn println(&self, str: &str){
        let _ = self.print_string(str);
        let _ = self.print_string("\n\r");
    }
    fn set_colour(&self, colour: simple_text_output::Colour) {
        let current_colour = self.mode.attribute;
        let mut colour = colour as i32;
        if colour.trailing_zeros() >= 4 {
            colour |= current_colour & 0x0f;
        }
        else{
            colour |= current_colour & 0xf0;
        }
        (self.set_attribute)(&self, colour as usize);
    }
    fn reset(&self) {
        (self.reset)(self, true);
    }
    fn clear_screen(&self) {
        (self.clear_screen)(self);
    }
}