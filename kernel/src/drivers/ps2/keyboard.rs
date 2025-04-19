
use core::{arch::asm, sync::atomic::{AtomicBool, Ordering}};
use crate::{arch::io_bus::{inb, io_wait, outb}, drivers::pic::{pic_end_m, remap_pic, PIC1_DATA, PIC2_DATA}, GLOBAL_TERMINAL};

const INPUT_PORT: u16 = 0x60;

const QWERTY_SCAN_CODE_TABLE: [char; 58] = 
['\0' ,  '\0' , '1', '2',
'3', '4', '5', '6',
'7', '8', '9', '0',
'-', '=', '\0', '\0',
'q', 'w', 'e', 'r',
't', 'y', 'u', 'i',
'o', 'p', '[', ']',
 '\0',  '\0', 'a', 's',
'd', 'f', 'g', 'h',
'j', 'k', 'l', ';',
'\'','`', '\0', '\\',
'z', 'x', 'c', 'v',
'b', 'n', 'm', ',',
'.', '/',  '\0', '*',
 '\0', ' '];

pub fn init_ps2(){
    remap_pic();
    outb(PIC1_DATA, 0b11111101);
    outb(PIC2_DATA, 0b11111111);
    unsafe {
        asm!("sti")
    }
}

static LEFT_SHIT: AtomicBool = AtomicBool::new(false);
static RIGHT_SHIT: AtomicBool = AtomicBool::new(false);
//static mut CAPS: bool = false;


pub extern "x86-interrupt" fn keyboard_interrupt_handler(){
    let scan_code = inb(INPUT_PORT);
    io_wait();
    pic_end_m();

    match scan_code {
        0x2A => LEFT_SHIT.store(true, Ordering::SeqCst),// left shift press
        0xAA => LEFT_SHIT.store(false, Ordering::SeqCst),// left shift release

        0x36 => RIGHT_SHIT.store(true, Ordering::SeqCst),// right shift press
        0xB6 => RIGHT_SHIT.store(false, Ordering::SeqCst),// right shift release

        0x0E => GLOBAL_TERMINAL.get_mut().unwrap().clear_char(),

        0x4D => GLOBAL_TERMINAL.get_mut().unwrap().fix_curs_pos(1),//right arrow
        0x4B => GLOBAL_TERMINAL.get_mut().unwrap().fix_curs_pos(-1),//left arrow

        0xE0 => (),
        _ => {
            let right_shift: bool = RIGHT_SHIT.load(Ordering::SeqCst);
            let left_shift: bool = LEFT_SHIT.load(Ordering::SeqCst);
            let is_upper_case: bool = left_shift | right_shift;
            let key = parse_key_input(scan_code, is_upper_case);
            if  key != '\0' {
                GLOBAL_TERMINAL.get_mut().unwrap().put_char(key);
            }
        }
    }
}

fn parse_key_input(scan_code: u8, is_upper_case: bool) -> char {
    if scan_code > 58{
        return '\0';
    }
    let scan_code_char: char = QWERTY_SCAN_CODE_TABLE[scan_code as usize];
    let abc: core::ops::RangeInclusive<char>= 'a'..='z';
    if is_upper_case && abc.contains(&scan_code_char){
        return (scan_code_char as u8 - 32) as char
    }
    else if is_upper_case{
        match scan_code_char {
            '1' => return '!',
            '2' => return '@',
            '3' => return '#',
            '4' => return '$',
            '5' => return '%',
            '6' => return '^',
            '7' => return '&',
            '8' => return '*',
            '9' => return '(',
            '0' => return ')',
            '-' => return '_',
            '=' => return '+',
            '[' => return '{',
            ']' => return '}',
            '\\' => return '|',
            '/' => return '?',
            '\'' => return '\"',
            ';' => return ':',
            '.' => return '>',
            ',' => return '<',
            _ => ()
        }
    }
    scan_code_char
}