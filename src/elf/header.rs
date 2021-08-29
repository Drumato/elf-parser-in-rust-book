pub static ELF_MAGIC_SIGNATURE: &[u8] = b"\x7fELF";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ElfHeader64 {
    pub id: ElfIdentification,
    pub ty: u16,
    pub machine: u16,
    pub version: u32,
    pub entry: u64,
    pub pht_offset: u64,
    pub sht_offset: u64,
    pub flags: u32,
    pub size: u16,
    pub pht_entry_size: u16,
    pub pht_entries: u16,
    pub sht_entry_size: u16,
    pub sht_entries: u16,
    pub sht_string_table_index: u16,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ElfIdentification {
    pub class: ElfClass,
    pub data: ElfData,
    pub version: ElfVersion,
    pub osabi: ElfOsAbi,
    pub abi_version: u8,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum ElfClass {
    None = 0,
    Bit32 = 1,
    Bit64 = 2,
    Num = 3,
    Unknown,
}

impl ElfClass {}

impl From<u8> for ElfClass {
    fn from(b: u8) -> ElfClass {
        match b {
            0 => ElfClass::None,
            1 => ElfClass::Bit32,
            2 => ElfClass::Bit64,
            3 => ElfClass::Num,
            _ => ElfClass::Unknown,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum ElfData {
    None = 0,
    Lsb = 1,
    Msb = 2,
    Num = 3,
    Unknown,
}

impl ElfData {}

impl From<u8> for ElfData {
    fn from(b: u8) -> ElfData {
        match b {
            0 => ElfData::None,
            1 => ElfData::Lsb,
            2 => ElfData::Msb,
            3 => ElfData::Num,
            _ => ElfData::Unknown,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum ElfVersion {
    Current = 1,
    Unknown,
}

impl ElfVersion {}

impl From<u8> for ElfVersion {
    fn from(b: u8) -> ElfVersion {
        match b {
            1 => ElfVersion::Current,
            _ => ElfVersion::Unknown,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum ElfOsAbi {
    SysV = 0,
    HpUx = 1,
    NetBsd = 2,
    Gnu = 3,
    Solaris = 6,
    Aix = 7,
    Irix = 8,
    FreeBsd = 9,
    Tru64 = 10,
    Modesto = 11,
    OpenBsd = 12,
    ArmAeAbi = 64,
    Arm = 97,
    Unknown,
}

impl ElfOsAbi {}

impl From<u8> for ElfOsAbi {
    fn from(b: u8) -> ElfOsAbi {
        match b {
            0 => ElfOsAbi::SysV,
            1 => ElfOsAbi::HpUx,
            2 => ElfOsAbi::NetBsd,
            3 => ElfOsAbi::Gnu,
            6 => ElfOsAbi::Solaris,
            7 => ElfOsAbi::Aix,
            8 => ElfOsAbi::Irix,
            9 => ElfOsAbi::FreeBsd,
            10 => ElfOsAbi::Tru64,
            11 => ElfOsAbi::Modesto,
            12 => ElfOsAbi::OpenBsd,
            64 => ElfOsAbi::ArmAeAbi,
            97 => ElfOsAbi::Arm,
            _ => ElfOsAbi::Unknown,
        }
    }
}
