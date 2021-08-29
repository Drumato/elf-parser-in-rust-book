use nom::multi::count;
use nom::number::complete::{le_u16, le_u32, le_u64};
use nom::IResult;
use nom::{bytes::complete::tag, combinator::map, number::complete::u8 as read_u8};

use crate::elf::{
    ElfClass, ElfData, ElfHeader64, ElfIdentification, ElfOsAbi, ElfVersion, ELF_MAGIC_SIGNATURE,
};

pub fn parse_64bit_elf_header(raw: &[u8]) -> IResult<&[u8], ElfHeader64> {
    let (r, id) = parse_elf_identification(raw)?;
    let (r, ty) = le_u16(r)?;
    let (r, machine) = le_u16(r)?;
    let (r, version) = le_u32(r)?;
    let (r, entry) = le_u64(r)?;
    let (r, pht_offset) = le_u64(r)?;
    let (r, sht_offset) = le_u64(r)?;
    let (r, flags) = le_u32(r)?;
    let (r, size) = le_u16(r)?;
    let (r, pht_entry_size) = le_u16(r)?;
    let (r, pht_entries) = le_u16(r)?;
    let (r, sht_entry_size) = le_u16(r)?;
    let (r, sht_entries) = le_u16(r)?;
    let (r, sht_string_table_index) = le_u16(r)?;
    Ok((
        r,
        ElfHeader64 {
            id,
            ty,
            machine,
            version,
            entry,
            pht_offset,
            sht_offset,
            flags,
            size,
            pht_entry_size,
            pht_entries,
            sht_entry_size,
            sht_entries,
            sht_string_table_index,
        },
    ))
}

fn parse_elf_magic_number(raw: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(ELF_MAGIC_SIGNATURE)(raw)
}

fn parse_elf_class(raw: &[u8]) -> IResult<&[u8], ElfClass> {
    map(read_u8, |byte: u8| ElfClass::from(byte))(raw)
}

fn parse_elf_data(raw: &[u8]) -> IResult<&[u8], ElfData> {
    map(read_u8, |byte: u8| ElfData::from(byte))(raw)
}

fn parse_elf_version(raw: &[u8]) -> IResult<&[u8], ElfVersion> {
    map(read_u8, |byte: u8| ElfVersion::from(byte))(raw)
}

fn parse_elf_osabi(raw: &[u8]) -> IResult<&[u8], ElfOsAbi> {
    map(read_u8, |byte: u8| ElfOsAbi::from(byte))(raw)
}

fn parse_elf_abi_version(raw: &[u8]) -> IResult<&[u8], u8> {
    read_u8(raw)
}

fn parse_elf_identification(raw: &[u8]) -> IResult<&[u8], ElfIdentification> {
    let (r, _) = parse_elf_magic_number(raw)?;
    let (r, class) = parse_elf_class(r)?;
    let (r, data) = parse_elf_data(r)?;
    let (r, version) = parse_elf_version(r)?;
    let (r, osabi) = parse_elf_osabi(r)?;
    let (r, abi_version) = parse_elf_abi_version(r)?;
    let (r, _padding) = count(read_u8, 7)(r)?;

    Ok((
        r,
        ElfIdentification {
            class,
            data,
            version,
            osabi,
            abi_version,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elf_magic_number_test() {
        helper(
            parse_elf_magic_number,
            ELF_MAGIC_SIGNATURE,
            ELF_MAGIC_SIGNATURE,
        );
        helper_fail(parse_elf_magic_number, b"\x7fFLF");
    }

    #[test]
    fn elf_class_test() {
        helper(parse_elf_class, &[0x00], ElfClass::None);
        helper(parse_elf_class, &[0x01], ElfClass::Bit32);
        helper(parse_elf_class, &[0x02], ElfClass::Bit64);
        helper(parse_elf_class, &[0x03], ElfClass::Num);
        helper(parse_elf_class, &[0xff], ElfClass::Unknown);
    }

    #[test]
    fn elf_data_test() {
        helper(parse_elf_data, &[0x00], ElfData::None);
        helper(parse_elf_data, &[0x01], ElfData::Lsb);
        helper(parse_elf_data, &[0x02], ElfData::Msb);
        helper(parse_elf_data, &[0x03], ElfData::Num);
        helper(parse_elf_data, &[0xff], ElfData::Unknown);
    }

    #[test]
    fn elf_version_test() {
        helper(parse_elf_version, &[0x01], ElfVersion::Current);
        helper(parse_elf_version, &[0xff], ElfVersion::Unknown);
    }

    #[test]
    fn elf_osabi_test() {
        helper(parse_elf_osabi, &[0x00], ElfOsAbi::SysV);
        helper(parse_elf_osabi, &[0x01], ElfOsAbi::HpUx);
        helper(parse_elf_osabi, &[0x02], ElfOsAbi::NetBsd);
        helper(parse_elf_osabi, &[0x03], ElfOsAbi::Gnu);
        helper(parse_elf_osabi, &[0x06], ElfOsAbi::Solaris);
        helper(parse_elf_osabi, &[0x07], ElfOsAbi::Aix);
        helper(parse_elf_osabi, &[0x08], ElfOsAbi::Irix);
        helper(parse_elf_osabi, &[0x09], ElfOsAbi::FreeBsd);
        helper(parse_elf_osabi, &[0x0a], ElfOsAbi::Tru64);
        helper(parse_elf_osabi, &[0x0b], ElfOsAbi::Modesto);
        helper(parse_elf_osabi, &[0x0c], ElfOsAbi::OpenBsd);
        helper(parse_elf_osabi, &[0x40], ElfOsAbi::ArmAeAbi);
        helper(parse_elf_osabi, &[0x61], ElfOsAbi::Arm);
        helper(parse_elf_osabi, &[0xfe], ElfOsAbi::Unknown);
    }

    #[test]
    fn elf_identification_test() {
        helper(
            parse_elf_identification,
            &[
                0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
            ElfIdentification {
                class: ElfClass::Bit64,
                data: ElfData::Lsb,
                version: ElfVersion::Current,
                osabi: ElfOsAbi::SysV,
                abi_version: 0x00,
            },
        );
    }

    #[test]
    fn elf_header64_test() {
        helper(
            parse_64bit_elf_header,
            &[
                // 0x00 ~ 0x10
                0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, // 0x10 ~ 0x20
                0x03, 0x00, 0x3e, 0x00, 0x01, 0x00, 0x00, 0x00, 0x60, 0x95, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, // 0x20 ~ 0x30
                0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x1c, 0x42, 0x00, 0x00, 0x00,
                0x00, 0x00, // 0x30 ~ 0x40
                0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x38, 0x00, 0x0c, 0x00, 0x40, 0x00, 0x2b, 0x00,
                0x2a, 0x00,
            ],
            ElfHeader64 {
                id: ElfIdentification {
                    class: ElfClass::Bit64,
                    data: ElfData::Lsb,
                    version: ElfVersion::Current,
                    osabi: ElfOsAbi::SysV,
                    abi_version: 0x00,
                },
                ty: 0x03,
                machine: 0x3e,
                version: 0x01,
                entry: 0x9560,
                pht_offset: 64,
                sht_offset: 4332680,
                flags: 0x0,
                size: 64,
                pht_entry_size: 56,
                pht_entries: 12,
                sht_entry_size: 64,
                sht_entries: 43,
                sht_string_table_index: 42,
            },
        );
    }

    fn helper<'a, T>(
        parser: impl Fn(&'a [u8]) -> IResult<&'a [u8], T>,
        input: &'a [u8],
        expected: T,
    ) where
        T: std::fmt::Debug + PartialEq + PartialOrd,
    {
        let parse_result = parser(input);
        assert!(parse_result.is_ok());

        assert_eq!(parse_result.unwrap().1, expected);
    }

    fn helper_fail<'a, T>(parser: impl Fn(&'a [u8]) -> IResult<&'a [u8], T>, input: &'a [u8])
    where
        T: std::fmt::Debug + PartialEq + PartialOrd,
    {
        let parse_result = parser(input);
        assert!(parse_result.is_err());
    }
}
