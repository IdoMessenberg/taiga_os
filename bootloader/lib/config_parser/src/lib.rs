#![no_std]

extern crate alloc as std_alloc;
use parser::{TomlTable, TomlValue};
use std_alloc::vec::Vec;

mod parser;

pub struct Info<'a>{
    pub kernel: KernelConfig<'a>,
    pub graphics: GraphicsConfig<'a>
}

#[repr(C)]
#[derive(Clone)]
pub struct KernelConfig<'a>{
    pub path: &'a str,
}

#[repr(C)]
#[derive(Clone)]
pub struct GraphicsConfig<'a> {
    pub dark_mode: bool,
    pub theme_path: &'a str,
    pub font_path: &'a str,
    pub resolution: Resolution
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct ColourTheme{
    pub black       : u32,
    pub white       : u32,
    pub red         : u32,
    pub green       : u32,
    pub blue        : u32,
    pub yellow      : u32,
    pub orange      : u32,
    pub purple      : u32,
    pub gray        : u32,
    pub dark_gray   : u32,
    pub light_red   : u32,
    pub light_green : u32,
    pub light_blue  : u32,
    pub light_yellow: u32,
    pub light_orange: u32,
    pub light_purple: u32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution{
    _Default,
    _Custom(u32,u32),
    _Hd,
    _2k,
    _4k,
}

pub fn read_config_file<'a>(file: &'a [u8]) -> Result<Info<'a>, &'a str> {
    let tables: Vec<TomlTable> = match parser::parse(file){
        Ok(v) => v,
        Err(_) => return Err("can not parse config file")
    };
    let mut config_info: Info = Info {
        kernel: KernelConfig {
            path: "", 
        },
        graphics: GraphicsConfig { 
            dark_mode: true, 
            theme_path: "", 
            font_path: "", 
            resolution: Resolution::_Default 
        }
    };
    for table in tables {
        match table.name {
            "kernel" => {
                config_info.kernel = match parse_kernel_table(table.values){
                    Some(v) => v,
                    None => return Err("can not read kernel table")
                }
            },
            "graphics" => {
                config_info.graphics = match parse_graphics_table(table.values){
                    Some(v) => v,
                    None => return Err("can not read graphics table")
                }
            },
            _ => continue
        }
    }
    Ok(config_info)
}
pub fn read_theme_file(file: &[u8]) -> Result<ColourTheme, &str>{
    let tables: Vec<TomlTable> = match parser::parse(file){
        Ok(v) => v,
        Err(_) => return Err("can not parse theme file")
    };
    let mut res:ColourTheme = ColourTheme { 
        black: 0, 
        white: 4586, 
        red: 0, 
        green: 0, 
        blue: 0, 
        yellow: 0, 
        orange: 0, 
        purple: 0, 
        gray: 0, 
        dark_gray: 0, 
        light_red: 0, 
        light_green: 0, 
        light_blue: 0, 
        light_yellow: 0, 
        light_orange: 0, 
        light_purple: 0 
    };
    for table in tables {
        match table.name {
            "colours" => {
                res = match parse_theme_table(table.values){
                    Some(v) => v,
                    None => return Err("can not read colours table")
                }
            }
            _ => continue
        }
    }
    Ok(res)
}

fn parse_kernel_table<'a>(items: Vec<(&'a str, TomlValue<'a>)>) -> Option<KernelConfig<'a>> {
    let mut res:KernelConfig = KernelConfig { path: ""};  
    for item in items {
        match item.0 {
            "path" => {
                res.path = match item.1.as_string(){
                    Some(v) => v,
                    None => return None
                } 
            }
            _ => ()
        }
   
    }
    Some(res)
}

fn parse_graphics_table<'a>(items: Vec<(&'a str, TomlValue<'a>)>) -> Option<GraphicsConfig<'a>> {
    let mut res:GraphicsConfig = GraphicsConfig { dark_mode: true, theme_path: "", font_path: "", resolution: Resolution::_Default };
    for item in items {
        match item.0 {
            "mode" => {
                res.dark_mode = match item.1.as_string(){
                    Some(v) => {
                        match v {
                            "light" => false,
                            _ => true,
                        }
                    }
                    None => return None
                } 
            }
            "theme" => {
                res.theme_path = match item.1.as_string(){
                    Some(v) => v,
                    None => return None
                } 
            }
            "font" => {
                res.font_path = match item.1.as_string(){
                    Some(v) => v,
                    None => return None
                } 
            }
            "resolution" => {
                res.resolution = match item.1.as_string(){
                    Some(v) => {
                        match v.to_lowercase().as_str() {
                            "default" => Resolution::_Default,
                            "hd" => Resolution::_Hd,
                            "2k" => Resolution::_2k,
                            "4k" => Resolution::_4k,
                            _ =>  Resolution::_Default,
                        }
                    },
                    None => {
                        match item.1.as_array() {
                            Some(v) => {
                                if v.len() != 2{
                                    return None
                                };
                                let width = match v[0].as_integer() {
                                    Some(width) => width,
                                    None => return None
                                };
                                let hight = match v[1].as_integer() {
                                    Some(hight) => hight,
                                    None => return None
                                };
                                Resolution::_Custom(width as u32, hight as u32)
                            }
                            None => return None
                        }
                    }
                } 
            }
            _ => ()
        }
   
    }
    Some(res)
}

fn parse_theme_table<'a>(items: Vec<(&'a str, TomlValue)>) -> Option<ColourTheme> {
    let mut res:ColourTheme = ColourTheme { 
        black: 0, 
        white: 0, 
        red: 0, 
        green: 0, 
        blue: 0, 
        yellow: 0, 
        orange: 0, 
        purple: 0, 
        gray: 0, 
        dark_gray: 0, 
        light_red: 0, 
        light_green: 0, 
        light_blue: 0, 
        light_yellow: 0, 
        light_orange: 0, 
        light_purple: 0 
    };
    for item in items {
        match item.0 {
            "black" => {
                res.black = match item.1.as_integer(){
                    Some(v) => v as u32,
                    None => return None
                } 
            }
            "white" => {
                res.white = match item.1.as_integer(){
                    Some(v) => v as u32,
                    None => return None
                } 
            }
            "red" => {
                res.red = match item.1.as_integer(){
                    Some(v) => v as u32,
                    None => return None
                } 
            }
            "green" => {
                res.green = match item.1.as_integer(){
                    Some(v) => v as u32,
                    None => return None
                } 
            }
            "blue" => {
                res.blue = match item.1.as_integer(){
                    Some(v) => v as u32,
                    None => return None
                } 
            }
            "yellow" => {
                res.yellow = match item.1.as_integer(){
                    Some(v) => v as u32,
                    None => return None
                } 
            }
            "orange" => {
                res.orange = match item.1.as_integer(){
                    Some(v) => v as u32,
                    None => return None
                } 
            }
            "purple" => {
                res.purple = match item.1.as_integer(){
                    Some(v) => v as u32,
                    None => return None
                } 
            }
            "gray" => {
                res.gray = match item.1.as_integer(){
                    Some(v) => v as u32,
                    None => return None
                } 
            }
            "dark-gray" => {
                res.dark_gray = match item.1.as_integer(){
                    Some(v) => v as u32,
                    None => return None
                } 
            }
            "light-red" => {
                res.light_red = match item.1.as_integer(){
                    Some(v) => v as u32,
                    None => return None
                } 
            }
            "light-green" => {
                res.light_green = match item.1.as_integer(){
                    Some(v) => v as u32,
                    None => return None
                } 
            }
            "light-blue" => {
                res.light_blue = match item.1.as_integer(){
                    Some(v) => v as u32,
                    None => return None
                } 
            }
            "light-yellow" => {
                res.light_yellow = match item.1.as_integer(){
                    Some(v) => v as u32,
                    None => return None
                } 
            }
            "light-orange" => {
                res.light_orange = match item.1.as_integer(){
                    Some(v) => v as u32,
                    None => return None
                } 
            }
            "light-purple" => {
                res.light_purple = match item.1.as_integer(){
                    Some(v) => v as u32,
                    None => return None
                } 
            }
            
            _ => ()
        }
   
    }
    Some(res)
}