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
    pub char_size:    u8,
    pub glyph_buffer: *const core::ffi::c_void,
}

pub fn load_font(system_table: &efi::system::Table, file_handle: &efi::protocols::media_access::file::Protocol)-> Result<FontInfo, efi::Status> {
    let header: *const Psf1Header = core::ptr::null();
    system_table.boot_time_services.allocate_pool(core::mem::size_of::<Psf1Header>(), core::ptr::addr_of!(header) as *const *const core::ffi::c_void);

    if (file_handle.read)(file_handle, &4, header as *const core::ffi::c_void).is_err() {
        return Err(efi::Status::Aborted);
    }
    if unsafe {(*header).magic_bytes[0] != PSF_1_MAG_0 || (*header).magic_bytes[1] != PSF_1_MAG_1} {
        return Err(efi::Status::Aborted);
    }
    let glyph_buffer_size: usize = unsafe {if (*header).font_mode == 1 { (*header).char_size as usize * 512} else{ (*header).char_size as usize * 256 }};
    let glyph_buffer: *const core::ffi::c_void = core::ptr::null();
    (file_handle.set_position)(file_handle, 4);
    system_table.boot_time_services.allocate_pool(glyph_buffer_size, core::ptr::addr_of!(glyph_buffer));
    (file_handle.read)(file_handle, &glyph_buffer_size, glyph_buffer);
    Ok(FontInfo { char_size: unsafe {(*header).char_size}, glyph_buffer })
}