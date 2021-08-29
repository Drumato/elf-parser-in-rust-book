#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProgramHeader64 {
    pub ty: SegmentType,
    pub flags: u32,
    pub offset: u64,
    pub virtual_addr: u64,
    pub physical_addr: u64,
    pub size_in_file: u64,
    pub size_in_mem: u64,
    pub align: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SegmentType {
    Null = 0,
    Load = 1,
    Dynamic = 2,
    Interp = 3,
    Note = 4,
    ShLib = 5,
    Phdr = 6,
    TLS = 7,
    Num = 8,
    GNUEHFrame = 0x6474e550,
    GNUStack = 0x6474e551,
    GNURelRO = 0x6474e552,
    Unknown,
}

impl From<u32> for SegmentType {
    fn from(v: u32) -> SegmentType {
        match v {
            0 => SegmentType::Null,
            1 => SegmentType::Load,
            2 => SegmentType::Dynamic,
            3 => SegmentType::Interp,
            4 => SegmentType::Note,
            5 => SegmentType::ShLib,
            6 => SegmentType::Phdr,
            7 => SegmentType::TLS,
            8 => SegmentType::Num,
            0x6474e550 => SegmentType::GNUEHFrame,
            0x6474e551 => SegmentType::GNUStack,
            0x6474e552 => SegmentType::GNURelRO,
            _ => SegmentType::Unknown,
        }
    }
}
