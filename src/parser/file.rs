use nom::IResult;

use crate::elf::{Elf64, Section64};

use super::{
    header::parse_64bit_elf_header, section::parse_64bit_section_header_table,
    string_table::parse_string_table_entry,
};

pub fn parse_64bit_elf(raw: &[u8]) -> IResult<&[u8], Elf64> {
    let (_, header) = parse_64bit_elf_header(raw)?;
    let (_, sht) = parse_64bit_section_header_table(header.sht_entries as usize)(
        &raw[header.sht_offset as usize..],
    )?;
    let raw_section_names_offset = sht[header.sht_string_table_index as usize].offset;
    let raw_section_names = &raw[raw_section_names_offset as usize..];
    let sections = sht
        .into_iter()
        .map(|header| Section64 {
            name: parse_string_table_entry(&raw_section_names[header.name_idx as usize..])
                .unwrap()
                .1,
            header,
        })
        .collect::<Vec<Section64>>();

    Ok((&raw[raw.len()..], Elf64 { header, sections }))
}
