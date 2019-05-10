use nom::{anychar, map_res, space0 as space, IResult};
use std::str;

macro_rules! named_key_and_string {
    ( $key:ident ) => {
        named!($key<&str, &str>, call!(key_and_string, stringify!($key)));
    };
}

macro_rules! named_key_and_possibly_unquoted_string {
    ( $key:ident ) => {
        named!($key<&str, &str>, alt!(
            call!(key_and_string, stringify!($key))
        | call!(key_and_unquoted_string, stringify!($key))));
    };
}

named_key_and_string!(catalogue);
named_key_and_string!(title);
named_key_and_string!(by);
named_key_and_string!(copyright);
named_key_and_string!(goal);
named_key_and_possibly_unquoted_string!(license);

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

fn key_and_string<'a>(input: &'a str, key: &str) -> IResult<&'a str, &'a str> {
    do_parse!(
        input,
        tag!(key)
            >> opt!(space)
            >> tag!(":")
            >> opt!(space)
            >> value: call!(quoted_string)
            >> tag!("\n")
            >> (value)
    )
}

fn key_and_unquoted_string<'a>(
    input: &'a str,
    key: &str,
) -> IResult<&'a str, &'a str> {
    do_parse!(
        input,
        tag!(key)
            >> opt!(space)
            >> tag!(":")
            >> opt!(space)
            >> value: call!(unquoted_string)
            >> tag!("\n")
            >> (value)
    )
}

named!(quoted_string<&str, &str>,
       delimited!(
           tag!("\""),
           take_till!(|c: char| c == '"'),
           tag!("\"")
       )
);

named!(unquoted_string<&str, &str>,
       take_till!(|c: char| c == '\n')
);

named!(
    one_letter<&str, char>,
    verify!(anychar, |c: char| c.is_ascii_lowercase())
);

named!(color<&str, (char, Color)>,
       do_parse!(
           tag!("color")
               >> opt!(space)
               >> char!(':')
               >> opt!(space)
               >> label: call!(one_letter)
               >> space
               >> color_code: call!(hex_color)
               >> char!('\n')
               >> (label, color_code)
       )
);

pub fn main() {
    assert_eq!(
        license("license: \"A restrictive license\"\n"),
        Ok(("", "A restrictive license"))
    );

    assert_eq!(
        catalogue("catalogue: \"mynonograms 1.my\"\n"),
        Ok(("", "mynonograms 1.my"))
    );

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

    println!("parser passed");
}

#[test]
fn parses_catalogue() {
    assert_eq!(
        catalogue("catalogue: \"mynonograms 1.my\"\n"),
        Ok(("", "mynonograms 1.my"))
    );
}

#[test]
fn parses_title() {
    assert_eq!(
        title("title: \"A really nice nonogram\"\n"),
        Ok(("", "A really nice nonogram"))
    );
}

#[test]
fn parses_by() {
    assert_eq!(by("by: \"Cody Coder\"\n"), Ok(("", "Cody Coder")));
}

#[test]
fn parses_copyright() {
    assert_eq!(
        copyright("copyright: \"(c) 1500 Cody Coder <cody@gmail.com>\"\n"),
        Ok(("", "(c) 1500 Cody Coder <cody@gmail.com>"))
    );
}

#[test]
fn parses_goal() {
    assert_eq!(goal("goal: \"00110101\"\n"), Ok(("", "00110101")));
}

#[test]
fn parses_quoted_license() {
    assert_eq!(
        license("license: \"You can use this file\"\n"),
        Ok(("", "You can use this file"))
    );
}

#[test]
fn parses_unquoted_license() {
    assert_eq!(license("license: MIT-8.0\n"), Ok(("", "MIT-8.0")));
}

#[test]
fn parses_hex_color() {
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

#[test]
fn parses_color() {
    assert_eq!(
        color("color: a #2F14DF\n"),
        Ok((
            "",
            (
                'a',
                Color {
                    red: 47,
                    green: 20,
                    blue: 223,
                }
            )
        ))
    );

    assert_eq!(
        color("color: f #010203\n"),
        Ok((
            "",
            (
                'f',
                Color {
                    red: 1,
                    green: 2,
                    blue: 3,
                }
            )
        ))
    );
}
