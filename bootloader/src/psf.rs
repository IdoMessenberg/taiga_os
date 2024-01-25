//*/-bootloader/src/psf.rs
const PSF_1_MAG_0: u8 = 0x36;
const PSF_1_MAG_1: u8 = 0x04;

///https://en.wikipedia.org/wiki/PC_Screen_Font
#[repr(C)]
pub struct Psf1Header {
    magic_bytes:   [u8; 2],
    font_mode:     u8,
    pub char_size: u8,
}

#[repr(C)]
pub struct FontInfo {
    pub char_size:                 u8,
    pub glyph_buffer_base_address: u64,
}

pub fn load_font(system_table: &efi::system::Table, file: &std_alloc::vec::Vec<u8>) -> Result<FontInfo, efi::Status> {
    let header = unsafe { &*(file.as_ptr() as *const Psf1Header) };
    if header.magic_bytes[0] != PSF_1_MAG_0 || header.magic_bytes[1] != PSF_1_MAG_1 {
        return Err(efi::Status::Aborted);
    }
    let glyph_buffer_size: usize = if header.font_mode == 1 { header.char_size as usize * 512 } else { header.char_size as usize * 256 };
    let glyph_buffer: *const core::ffi::c_void = core::ptr::null();
    system_table.boot_time_services.allocate_pool(glyph_buffer_size, core::ptr::addr_of!(glyph_buffer));
    system_table.boot_time_services.copy_mem(glyph_buffer, unsafe { file.as_ptr().add(core::mem::size_of::<Psf1Header>()) } as *const core::ffi::c_void, glyph_buffer_size);
    Ok(FontInfo { char_size: header.char_size, glyph_buffer_base_address: glyph_buffer as u64 })
}
