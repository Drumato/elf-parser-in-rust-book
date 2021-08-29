use nom::{
    combinator::map,
    multi::count,
    number::complete::{le_u32, le_u64},
    IResult,
};

use crate::elf::{ProgramHeader64, SegmentType};

pub fn parse_64bit_program_header_table<'a>(
    entries: usize,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<ProgramHeader64>> {
    move |raw: &'a [u8]| count(parse_64bit_program_header, entries)(raw)
}

fn parse_segment_type(raw: &[u8]) -> IResult<&[u8], SegmentType> {
    map(le_u32, |v| SegmentType::from(v))(raw)
}

fn parse_64bit_program_header(raw: &[u8]) -> IResult<&[u8], ProgramHeader64> {
    let (r, ty) = parse_segment_type(raw)?;
    let (r, flags) = le_u32(r)?;
    let (r, offset) = le_u64(r)?;
    let (r, virtual_addr) = le_u64(r)?;
    let (r, physical_addr) = le_u64(r)?;
    let (r, size_in_file) = le_u64(r)?;
    let (r, size_in_mem) = le_u64(r)?;
    let (r, align) = le_u64(r)?;

    Ok((
        r,
        ProgramHeader64 {
            ty,
            flags,
            offset,
            virtual_addr,
            physical_addr,
            size_in_file,
            size_in_mem,
            align,
        },
    ))
}
