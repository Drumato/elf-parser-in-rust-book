use nom::{combinator::map, IResult};

use crate::elf::{Elf64, Section64};

use super::{header::parse_64bit_elf_header, section::parse_64bit_section_header_table};

pub fn parse_64bit_elf(raw: &[u8]) -> IResult<&[u8], Elf64> {
    let (_, header) = parse_64bit_elf_header(raw)?;
    let (_, sections) = map(
        parse_64bit_section_header_table(header.sht_entries as usize),
        |sht| {
            sht.into_iter()
                .map(|header| Section64 {
                    name: "".to_string(),
                    header,
                })
                .collect::<Vec<Section64>>()
        },
    )(&raw[header.sht_offset as usize..])?;

    Ok((&raw[raw.len()..], Elf64 { header, sections }))
}
