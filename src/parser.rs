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

macro_rules! one_line_key_string {
    ( $key:ident ) => {
        named!(
            $key<&[u8], &str>,
            do_parse!(
                tag!(stringify!($key))
                    >> opt!(space)
                    >> char!(':')
                    >> opt!(space)
                    >> val: map_res!(take_until!("\n"),
                                     complete_byte_slice_to_str)
                    >> newline
                    >> (val)
            )
        );
    };
}

one_line_key_string!(catalogue);

named!(
    catalogue2<&[u8], &str>,
    do_parse!(
        key: map_res!(alpha, complete_byte_slice_to_str)
            >> opt!(space)
            >> char!(':')
            >> opt!(space)
            >> val: map_res!(take_until!("\n"), complete_byte_slice_to_str)
            >> newline
            >> (val)
    )
);

named!(
    oneline_key_string_value<(&str, &str)>,
    do_parse!(
        key: map_res!(alpha, complete_byte_slice_to_str)
            >> opt!(space)
            >> char!(':')
            >> opt!(space)
            >> val: map_res!(take_until!("\n"), complete_byte_slice_to_str)
            >> newline
            >> (key, val)
    )
);

pub fn main() {
    assert_eq!(
        oneline_key_string_value(&b"key: value\n"[..]),
        Ok((&[][..], ("key", "value")))
    );

    assert_eq!(
        catalogue(&b"catalogue: this is my catalogue\n"[..]),
        Ok((&[][..], "this is my catalogue"))
    );

    println!("parser passed");
}
