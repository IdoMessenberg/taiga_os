
///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=256
#[repr(C)]
pub struct Guid(pub u32, pub u16, pub u16, pub [u8; 8]);

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=2269
#[repr(usize)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Status {
    Success             = 0,
    LoadError           = 1,
    InvalidParameter    = 2,
    Unsupported         = 3,
    BadBufferSize       = 4,
    BufferTooSmall      = 5,
    NotReady            = 6,
    DeviceError         = 7,
    WriteProtected      = 8,
    OutOfResources      = 9,
    VolumeCorrupted     = 10,
    VolumeFull          = 11,
    NoMedia             = 12,
    MediaChanged        = 13,
    NotFound            = 14,
    AccessDenied        = 15,
    NoResponse          = 16,
    NoMapping           = 17,
    Timeout             = 18,
    NotStarted          = 19,
    AlreadyStarted      = 20,
    Aborted             = 21,
    IcmpError           = 22,
    TftpError           = 23,
    ProtocolError       = 24,
    IncompatibleVersion = 25,
    SecurityViolation   = 26,
    CrcError            = 27,
    EndOfMedia          = 28,
    EndOfFile           = 31, //This is not an error there's supposed to be a jump in the values here
    InvalidLanguage     = 32,
    CompromisedData     = 33,
    IpAddressConflict   = 34,
    HttpError           = 35,
    UnknownGlyph        = i32::MAX as usize + 2,
    DeleteFailure       = i32::MAX as usize + 3,
    WriteFailure        = i32::MAX as usize + 4,
    WarnBufferTooSmall  = i32::MAX as usize + 5,
    StaleData           = i32::MAX as usize + 6,
    FileSystem          = i32::MAX as usize + 7,
    ResetRequired       = i32::MAX as usize + 8,
}
impl Status {
    pub fn is_ok(&self) -> bool { self == &Status::Success }
}


///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=241
#[repr(C)]
pub enum AllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=242
///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=239
#[repr(u32)]
#[derive(PartialEq, Eq)]
pub enum MemoryType {
    ReservedMemoryType,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
    UnacceptedMemoryType,
    MaxMemoryType,
}

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=245
#[repr(C)]
pub struct MemoryDescriptor {
    pub r#type:          MemoryType,
    pub physical_start:  u64,
    pub virtual_start:   u64,
    pub number_of_pages: u64,
    pub attribute:       u64,
}

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=560
#[repr(C)]
pub struct PixelBitmask {
    pub red_mask:      u32,
    pub green_mask:    u32,
    pub blue_mask:     u32,
    pub reserved_mask: u32,
}

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=561
#[repr(C)]
pub enum PixelFormat {
    PixelRedGreenBlueReserved8BitPerColour,
    PixelBlueGreenRedReserved8BitPerColour,
    PixelBitMask,
    PixelBltOnly,
    PixelFormatMax,
}

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=566
#[repr(C)]
pub struct BltPixel {
    blue:     u8,
    green:    u8,
    red:      u8,
    reserved: u8,
}

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=566
#[repr(C)]
pub enum BltOperation {
    VideoFill,
    VideoToBltBuffer,
    BufferToVideo,
    VideoToVideo,
    Max,
}

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=523
#[repr(C)]
pub struct InputKey {
    pub scan_code: u16,
    pub unicode_char: u16
}

///https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf#page=344
#[repr(C)]
pub enum ResetType {
    Cold,
    Warm,
    Shutdown,
    PlatformSpecific
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct MemoryMapInfo {
    pub address:            u64,
    pub size:               usize,
    pub key:                usize,
    pub descriptor_size:    usize,
    pub descriptor_version: u32,
}
impl MemoryMapInfo {
    pub fn get_pages(&self) -> usize{
        let mut pages: usize = 0;
        for index in 0..self.size/self.descriptor_size - 1 {
            let descriptor: &MemoryDescriptor =  self.get_memory_descriptor(index as u64);
            pages += descriptor.number_of_pages as usize;
        };
        pages
    }
    pub fn get_memory_descriptor(&self, index: u64) -> &MemoryDescriptor {
        unsafe {
            &*((self.address + self.descriptor_size as u64 * index) as *const MemoryDescriptor)
        }
    }
}