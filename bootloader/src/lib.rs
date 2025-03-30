#![no_std]

pub extern crate uefi as efi;
pub extern crate psf_loader as psf;
#[repr(C)]
#[derive(Debug)]
pub struct _BootInfo_ {
    pub memory_map: efi::data_types::MemoryMapInfo,
    pub font: psf::FontInfo,
    pub theme: TerminalTheme,
    pub graphics: GraphicsInfo
}


#[repr(C)]
#[derive(Debug)]
pub struct GraphicsInfo{
    pub framebuffer_base_address:  u64,
    pub framebuffer_size:          usize,
    pub horizontal_resolution:     u32,
    pub vertical_resolution:       u32,
    pub pixels_per_scan_line:      u32,
}
impl GraphicsInfo {
    pub fn from_graphics_output_protocol_mode(mode: &efi::console_support::graphics_output::Mode) -> Self {
        Self { 
            framebuffer_base_address: mode.frame_buffer_base, 
            framebuffer_size: mode.frame_buffer_size, 
            horizontal_resolution: mode.info.horizontal_resolution, 
            vertical_resolution: mode.info.vertical_resolution, 
            pixels_per_scan_line: mode.info.pixels_per_scan_line
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TerminalTheme{
    pub dark_mode   : bool,
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
#[cfg(feature = "default")]
impl TerminalTheme {
    pub fn from_config_colour_theme(colour_theme: config_parser::ColourTheme, dark_mode: bool) -> Self{
        Self { 
            dark_mode, 
            black: colour_theme.black, 
            white: colour_theme.white, 
            red: colour_theme.red, 
            green: colour_theme.green, 
            blue: colour_theme.blue, 
            yellow: colour_theme.yellow, 
            orange: colour_theme.orange, 
            purple: colour_theme.purple, 
            gray: colour_theme.gray, 
            dark_gray: colour_theme.dark_gray, 
            light_red: colour_theme.light_red, 
            light_green: colour_theme.light_green, 
            light_blue: colour_theme.light_blue, 
            light_yellow: colour_theme.light_yellow, 
            light_orange: colour_theme.light_orange, 
            light_purple: colour_theme.light_purple
        }
    }
}