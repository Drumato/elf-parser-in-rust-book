use nom::{
    bytes::complete::{tag, take_while},
    combinator::map,
    sequence::terminated,
    IResult,
};

pub fn parse_string_table_entry(raw: &[u8]) -> IResult<&[u8], String> {
    terminated(parse_ascii_string, tag(b"\x00"))(raw)
}

fn parse_ascii_string(raw: &[u8]) -> IResult<&[u8], String> {
    map(
        take_while(|c: u8| c.is_ascii_graphic()),
        |raw_name: &[u8]| String::from_utf8(raw_name.to_vec()).unwrap().to_string(),
    )(raw)
}
