//*/-bootloader/lib/uefi/src/console.rs
use crate::protocols::{console_support::simple_text_output, data_types::Status};

pub fn init(con_out: &simple_text_output::Protocol) -> Status {
    if (con_out.reset)(con_out, true).is_err() {
        return con_out.println_status("Console - Boot Time Console Is Not Initialised! - Reset", Status::Aborted);
    }
    if con_out.clear_screen().is_err() {
        return con_out.println_status("Console - Boot Time Console Is Not Initialised! - Clear", Status::Aborted);
    }
    con_out.println_status("Console - Boot Time Console Is Initialised!", Status::Success)
}

fn put_char(con_out: &simple_text_output::Protocol, char: char) -> Status {
    if (con_out.test_string)(con_out, [char as u16, 0u16].as_ptr()).is_ok() {
        return (con_out.output_string)(con_out, [char as u16, 0u16].as_ptr());
    }
    Status::Aborted
}

fn put_usize(con_out: &simple_text_output::Protocol, num: usize) -> Status {
    let mut i = 1;
    for _ in 0..17 {
        i *= 10;
        if i >= num / 10 {
            break;
        }
    }
    let mut temp = num;
    for _ in 0..17 {
        if put_char(con_out, (b'0' + (temp / i) as u8) as char).is_err() {
            return Status::Aborted;
        }
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
        for char in string.chars() {
            if put_char(self, char).is_err() {
                return Status::Aborted;
            }
        }
        Status::Success
    }

    pub fn println(&self, string: &str) -> Status { return if self.print(string).is_err() || self.print("\r\n").is_err() { Status::Aborted } else { Status::Success } }

    pub fn print_usize(&self, string: &str, num: usize) -> Status {
        let mut last_char = '\0';
        for (_, char) in string.chars().enumerate() {
            if last_char == '{' && char == '}' {
                put_usize(self, num);
            } else if char != '{' {
                put_char(self, char);
            }
            if last_char == '{' && char != '}' {
                put_char(self, last_char);
                put_char(self, char);
            }
            last_char = char;
        }
        Status::Success
    }

    pub fn println_usize(&self, string: &str, num: usize) -> Status { return if self.print_usize(string, num).is_err() || self.print("\r\n").is_err() { Status::Aborted } else { Status::Success } }

    pub fn println_status(&self, string: &str, status: Status) -> Status {
        match status as u32 {
            0 => {
                self.set_forground_colour(Colour::Green);
                self.print("[OK!]");
                self.set_forground_colour(Colour::LightGray);
                self.print(" - ");
                self.println(string);
            }
            x if x > 0 && x < i32::MAX as u32 + 2u32 => {
                self.set_forground_colour(Colour::LightRed);
                self.print("[ERR!]");
                self.set_forground_colour(Colour::LightGray);
                self.print(" - ");
                self.println(string);
            }
            x if x >= i32::MAX as u32 + 2u32 => {
                self.set_forground_colour(Colour::Brown);
                self.print("[WARN!]");
                self.set_forground_colour(Colour::LightGray);
                self.print(" - ");
                self.println(string);
            }
            _ => {
                self.set_forground_colour(Colour::DarkGray);
                self.print("[UNKNOWN STATUS]");
                self.set_forground_colour(Colour::LightGray);
                self.print(" - ");
                self.println(string);
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
