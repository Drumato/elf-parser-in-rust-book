use nom::{
    combinator::map,
    multi::count,
    number::complete::{le_u32, le_u64},
    IResult,
};

use crate::elf::{SectionHeader64, SectionType};
pub fn parse_64bit_section_header_table<'a>(
    entries: usize,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<SectionHeader64>> {
    move |raw: &'a [u8]| count(parse_64bit_section_header, entries)(raw)
}

fn parse_section_type(raw: &[u8]) -> IResult<&[u8], SectionType> {
    map(le_u32, |v| SectionType::from(v))(raw)
}

fn parse_64bit_section_header(raw: &[u8]) -> IResult<&[u8], SectionHeader64> {
    let (r, name_idx) = le_u32(raw)?;
    let (r, ty) = parse_section_type(r)?;
    let (r, flags) = le_u64(r)?;
    let (r, addr) = le_u64(r)?;
    let (r, offset) = le_u64(r)?;
    let (r, size) = le_u64(r)?;
    let (r, link) = le_u32(r)?;
    let (r, info) = le_u32(r)?;
    let (r, addr_align) = le_u64(r)?;
    let (r, entry_size) = le_u64(r)?;

    Ok((
        r,
        SectionHeader64 {
            name_idx,
            ty,
            flags,
            addr,
            offset,
            size,
            link,
            info,
            addr_align,
            entry_size,
        },
    ))
}
