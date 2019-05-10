use nom::{map_res, space0 as space, IResult};
use std::str;

macro_rules! named_one_line_key_string {
    ( $key:ident ) => {
        named!($key<&str>, call!(one_line_key_string, stringify!($key)));
    };
}

macro_rules! named_one_line_key_quoted_or_unquoted_string {
    ( $key:ident ) => {
        named!(
            $key<&str>,
            alt!(
                call!(one_line_key_string, stringify!($key))
                    | call!(one_line_key_unquoted_string, stringify!($key))
            )
        );
    };
}

named_one_line_key_string!(catalogue);
named_one_line_key_string!(title);
named_one_line_key_string!(by);
named_one_line_key_string!(copyright);
named_one_line_key_string!(goal);
named_one_line_key_quoted_or_unquoted_string!(license);

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

named!(hex_primary<&str, u8>,
       map_res!(take_while_m_n!(2, 2, is_hex_digit), from_hex)
);

named!(hex_color<&str, Color>,
       do_parse!(
           tag!("#") >>
               red:   hex_primary >>
               green: hex_primary >>
               blue:  hex_primary >>
               (Color { red, green, blue })
       )
);

pub fn main() {
    assert_eq!(
        license(&b"license: \"A restrictive license\"\n"[..]),
        Ok((&[][..], "A restrictive license"))
    );

    assert_eq!(
        catalogue(&b"catalogue: \"mynonograms 1.my\"\n"[..]),
        Ok((&[][..], "mynonograms 1.my"))
    );

    println!("parser passed");
}

fn complete_byte_slice_to_str<'a>(
    s: &'a [u8],
) -> Result<&'a str, str::Utf8Error> {
    str::from_utf8(s)
}

fn one_line_key_string<'a>(
    i: &'a [u8],
    key: &str,
) -> IResult<&'a [u8], &'a str> {
    do_parse!(
        i,
        tag!(key)
            >> opt!(space)
            >> char!(':')
            >> opt!(space)
            >> char!('"')
            >> val: map_res!(take_until!("\"\n"), complete_byte_slice_to_str)
            >> tag!("\"\n")
            >> (val)
    )
}

fn one_line_key_unquoted_string<'a>(
    i: &'a [u8],
    key: &str,
) -> IResult<&'a [u8], &'a str> {
    do_parse!(
        i,
        tag!(key)
            >> opt!(space)
            >> char!(':')
            >> opt!(space)
            >> val: map_res!(take_until!("\n"), complete_byte_slice_to_str)
            >> char!('\n')
            >> (val)
    )
}

#[test]
fn parses_catalogue() {
    assert_eq!(
        catalogue(&b"catalogue: \"mynonograms 1.my\"\n"[..]),
        Ok((&[][..], "mynonograms 1.my"))
    );
}

#[test]
fn parses_title() {
    assert_eq!(
        title(&b"title: \"A really nice nonogram\"\n"[..]),
        Ok((&[][..], "A really nice nonogram"))
    );
}

#[test]
fn parses_by() {
    assert_eq!(
        by(&b"by: \"Cody Coder\"\n"[..]),
        Ok((&[][..], "Cody Coder"))
    );
}

#[test]
fn parses_copyright() {
    assert_eq!(
        copyright(
            &b"copyright: \"(c) 1500 Cody Coder <cody@gmail.com>\"\n"[..]
        ),
        Ok((&[][..], "(c) 1500 Cody Coder <cody@gmail.com>"))
    );
}

#[test]
fn parses_goal() {
    assert_eq!(
        goal(&b"goal: \"00110101\"\n"[..]),
        Ok((&[][..], "00110101"))
    );
}

#[test]
fn parses_quoted_license() {
    assert_eq!(
        license(&b"license: \"You can use this file\"\n"[..]),
        Ok((&[][..], "You can use this file"))
    );
}

#[test]
fn parses_unquoted_license() {
    assert_eq!(
        license(&b"license: MIT-8.0\n"[..]),
        Ok((&[][..], "MIT-8.0"))
    );
}

#[test]
fn parses_color() {
    assert_eq!(
        hex_color("#2F14DF"),
        Ok((
            "",
            Color {
                red: 47,
                green: 20,
                blue: 223,
            }
        ))
    );
}
