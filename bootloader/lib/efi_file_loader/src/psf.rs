use efif::BootTimeAllocFunctions;

pub fn load_font(system_table: &efi::system::Table, file: &[u8]) -> Result<psf::FontInfo, efi::Status> {
    let header = unsafe { &*(file.as_ptr() as *const psf::Header) };
    if header.magic_bytes[0] != psf::PSF_1_MAG_0 || header.magic_bytes[1] != psf::PSF_1_MAG_1 {
        return Err(efi::Status::Aborted);
    }
    let glyph_buffer_size: usize = if header.font_mode == 1 { header.char_size as usize * 512 } else { header.char_size as usize * 256 };
    let glyph_buffer: *const core::ffi::c_void = core::ptr::null();
    system_table.boot_time_services.alloc_pool(glyph_buffer_size, core::ptr::addr_of!(glyph_buffer));
    system_table.boot_time_services.copy_mem(glyph_buffer, unsafe { file.as_ptr().add(core::mem::size_of::<psf::Header>()) } as *const core::ffi::c_void, glyph_buffer_size);
    Ok(psf::FontInfo { char_size: header.char_size, glyph_buffer_base_address: glyph_buffer as u64 })
}
