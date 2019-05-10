use nom::{alpha, map_res, newline, space0 as space, IResult};
use std::str;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn complete_byte_slice_to_str<'a>(
    s: &'a [u8],
) -> Result<&'a str, str::Utf8Error> {
    str::from_utf8(s)
}

// use nom::be_u8;
// named!(
//     tag_length_value<u8>,
//     do_parse!(
//         tag!( &[ 42u8 ][..] ) >>
//                length: be_u8         >>
//            // bytes:  take!(length) >>
//            // (length, bytes)
//                (length)
//     )
// );

named!(
    oneline_key_string_value<(&str, &str)>,
    do_parse!(
        key: map_res!(alpha, complete_byte_slice_to_str)
               >> opt!(space)
               >> char!(':')
               >> opt!(space)
               >> val: map_res!(alpha, complete_byte_slice_to_str)
               >> newline
               >> (key, val)
               // >> (key, key)
    )
);

pub fn main() {
    assert_eq!(
        oneline_key_string_value(&b"key: value\n"[..]),
        Ok((&[][..], ("key", "value")))
    );

    println!("parser passed");
}
