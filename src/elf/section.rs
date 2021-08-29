#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Section64 {
    pub name: String,
    pub header: SectionHeader64,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct SectionHeader64 {
    pub name_idx: u32,
    pub ty: SectionType,
    pub flags: u64,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub addr_align: u64,
    pub entry_size: u64,
}

#[derive(Debug, Clone, Hash, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum SectionType {
    Null = 0,
    ProgBits = 1,
    SymTab = 2,
    StrTab = 3,
    Rela = 4,
    Hash = 5,
    Dynamic = 6,
    Note = 7,
    NoBits = 8,
    Rel = 9,
    ShLib = 10,
    DynSym = 11,
    InitArray = 14,
    FiniArray = 15,
    PreInitArray = 16,
    Group = 17,
    SymTabShNdx = 18,
    Num = 19,
    Unknown,
}

impl From<u32> for SectionType {
    fn from(v: u32) -> SectionType {
        match v {
            0 => SectionType::Null,
            1 => SectionType::ProgBits,
            2 => SectionType::SymTab,
            3 => SectionType::StrTab,
            4 => SectionType::Rela,
            5 => SectionType::Hash,
            6 => SectionType::Dynamic,
            7 => SectionType::Note,
            8 => SectionType::NoBits,
            9 => SectionType::Rel,
            10 => SectionType::ShLib,
            11 => SectionType::DynSym,
            14 => SectionType::InitArray,
            15 => SectionType::FiniArray,
            16 => SectionType::PreInitArray,
            17 => SectionType::Group,
            18 => SectionType::SymTabShNdx,
            19 => SectionType::Num,
            _ => SectionType::Unknown,
        }
    }
}
