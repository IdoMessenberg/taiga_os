//*/-bootloader/lib/uefi/src/protocols/data_types.rs
#[repr(C)]
pub struct Guid(pub u32, pub u16, pub u16, pub [u8; 8]);

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
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
    TftpErorr           = 23,
    ProtocolError       = 24,
    IncompatibleVersion = 25,
    SecurityViolation   = 26,
    CrcError            = 27,
    EndOfMedia          = 28,
    EndOfFile           = 31, //-This is not an error there's supposed to be a jump in the values here
    InvalidLanguage     = 32,
    CompromisedData     = 33,
    IpAddressConflict   = 34,
    HttpError           = 35,
    UnknownGlyph        = i32::MAX as u32 + 2,
    DeleteFailure       = i32::MAX as u32 + 3,
    WriteFailure        = i32::MAX as u32 + 4,
    WarnBufferTooSmall  = i32::MAX as u32 + 5,
    StaleData           = i32::MAX as u32 + 6,
    FileSystem          = i32::MAX as u32 + 7,
    ResetRequired       = i32::MAX as u32 + 8,
}
impl Status {
    pub fn is_ok(&self) -> bool { self == &Status::Success }
    pub fn is_err(&self) -> bool { self != &Status::Success }
}

#[repr(C)]
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

#[repr(C)]
pub enum AllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

#[repr(C)]
pub struct MemoryDescriptor {
    pub r#type:          u32,
    pub physical_start:  u64,
    pub virtual_start:   u64,
    pub number_of_pages: u64,
    pub attribute:       u64,
}

#[repr(C)]
pub struct Time {
    year:        u16,
    month:       u8,
    day:         u8,
    hour:        u8,
    minute:      u8,
    second:      u8,
    pad1:        u8,
    nano_second: u32,
    time_zone:   i16,
    daylight:    u8,
    pad2:        u8,
}
