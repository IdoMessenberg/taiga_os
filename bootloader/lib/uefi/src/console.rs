//*/-bootloader/lib/uefi/src/console.rs
use crate::protocols::{console_support::simple_text_output, data_types::Status};

pub fn init(con_out: &simple_text_output::Protocol) -> Status {
    if (con_out.reset)(con_out, true).is_err() || con_out.clear_screen().is_err() {
        return con_out.println_status("Console - Boot Time Console Is Not Initialised!", Status::Aborted);
    }
    con_out.println_status("Console - Boot Time Console Is Initialised!", Status::Success)
}

fn put_char(con_out: &simple_text_output::Protocol, char: char) -> Status {
    //-testing if simple text output protocl support that char
    // only if it supports it then will this function output it
    // for example uefi simple text output protocol supports this char - 'A' but not '🌲'
    // so this function will only attempt to output the 'A' char

    //-simple text output protocol work with the UCS-2 text format while Rust strings and chars are UTF-8
    // this is why the char is converted to u16 as UCS-2 is a 16-bit format while UTF-8 is an 8-bit format
    // the 0u16 is added to the array as it is a null terminating end point to make sure only the char will be outputed
    if (con_out.test_string)(con_out, [char as u16, 0u16].as_ptr()).is_ok() {
        // returning the status of output string so the function could make sure this function work correctly
        return (con_out.output_string)(con_out, [char as u16, 0u16].as_ptr());
    }
    Status::Aborted
}

fn put_usize(con_out: &simple_text_output::Protocol, num: usize) -> Status {
    //-geting the numer of digits in the number
    let mut i: usize = 1;
    //-this is a for loop and not a while to prevent an infinate loop and is set to 17
    // as it is the max number fo digits in a usize
    for _ in 0..17 {
        i *= 10;
        if i >= num / 10 {
            break;
        }
    }
    //-outputing the number to screen
    let mut temp: usize = num;
    //-iterating over the number for it's length an outputing the digits one by one
    // by deviding the temporary number by i which is
    for _ in 0..17 {
        if put_char(con_out, (b'0' + (temp / i) as u8) as char).is_err() {
            return Status::Aborted;
        }
        //-removing the first digit of the number
        // temp = 1234  -> 234
        // i    = 1000
        temp %= i;

        i /= 10;
        if i == 0 {
            break;
        }
    }
    Status::Success
}

impl simple_text_output::Protocol {
    pub fn clear_screen(&self) -> Status { (self.clear_screen)(self) }

    pub fn set_forground_colour(&self, colour: Colour) -> Status { (self.set_attribute)(self, colour as usize) }

    pub fn print(&self, string: &str) -> Status {
        //-iterating over the string slice chars and outputing them one by one
        // this let's you output a string of any size with out ant limits
        for char in string.chars() {
            //-if the char couldn't be outputed either because it's not aviable to output or a diffrent output problem
            // the function will stop and will not continue to output the chars of the string
            if put_char(self, char).is_err() {
                return Status::Aborted;
            }
        }
        //- if nothing fails the function will exit on a success status if nothing goes wrong in the for loop
        Status::Success
    }

    pub fn println(&self, string: &str) -> Status {
        //-printing both the string and "\r\n" -> \r which returns to the begining of the line and \n which makes a new line
        if self.print(string).is_err() || self.print("\r\n").is_err() {
            Status::Aborted
        } else {
            Status::Success
        }
    }

    pub fn print_usize(&self, string: &str, num: usize) -> Status {
        let mut last_char: char = '\0';
        for (_, char) in string.chars().enumerate() {
            //-printing the number if both chars are "{}" which is like the standard rust formating
            if last_char == '{' && char == '}' {
                if put_usize(self, num).is_err() {
                    return Status::Aborted;
                }
            }
            //-printing to screen the char if it's not '{' which could be where the number should be placed
            else if char != '{' {
                if put_char(self, char).is_err() {
                    return Status::Aborted;
                }
            }
            //-printing to screen the last char if it was '{' which is skiped and printing the current char
            else if last_char == '{' && char != '}' {
                if put_char(self, last_char).is_err() {
                    return Status::Aborted;
                }
                if put_char(self, char).is_err() {
                    return Status::Aborted;
                }
            }
            last_char = char;
        }
        Status::Success
    }

    pub fn println_usize(&self, string: &str, num: usize) -> Status {
        //-printing both the string and "\r\n" -> \r which returns to the begining of the line and \n which makes a new line
        if self.print_usize(string, num).is_err() || self.print("\r\n").is_err() {
            Status::Aborted
        } else {
            Status::Success
        }
    }

    pub fn println_status(&self, string: &str, status: Status) -> Status {
        match status as usize {
            0 => {
                //-∇-setting the colour to green                        ∇-printing out ok                       ∇-setting the colour to light gray the default colour   ∇-printing the spacer                 ∇-printing the string
                if self.set_forground_colour(Colour::Green).is_err() || self.print("[OK!]").is_err() || self.set_forground_colour(Colour::LightGray).is_err() || self.print(" - ").is_err() || self.println(string).is_err() {
                    return Status::Aborted;
                }
            }
            x if x > 0 && x < i32::MAX as usize + 2 => {
                //-∇-setting the colour to red                             ∇-printing out err                      ∇-setting the colour to light gray the default colour     ∇-printing the spacer                ∇-printing the string
                if self.set_forground_colour(Colour::LightRed).is_err() || self.print("[ERR!]").is_err() || self.set_forground_colour(Colour::LightGray).is_err() || self.print(" - ").is_err() || self.println(string).is_err() {
                    return Status::Aborted;
                }
            }
            x if x >= i32::MAX as usize + 2 => {
                //-∇-setting the colour to brown (which is more yellow) ∇-printing out warn                      ∇-setting the colour to light gray the default colour     ∇-printing the spacer                ∇-printing the string
                if self.set_forground_colour(Colour::Brown).is_err() || self.print("[WARN!]").is_err() || self.set_forground_colour(Colour::LightGray).is_err() || self.print(" - ").is_err() || self.println(string).is_err() {
                    return Status::Aborted;
                }
            }
            _ => {
                //-∇-setting the colour to dark gray                        ∇-printing out unknown status                     ∇-setting the colour to light gray the default colour    ∇-printing the spacer                 ∇-printing the string
                if self.set_forground_colour(Colour::DarkGray).is_err() || self.print("[UNKNOWN STATUS]").is_err() || self.set_forground_colour(Colour::LightGray).is_err() || self.print(" - ").is_err() || self.println(string).is_err() {
                    return Status::Aborted;
                }
            }
        }
        status
    }
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