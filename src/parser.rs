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

pub fn main() {
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
        goal(&b"license: \"You can use this file\"\n"[..]),
        Ok((&[][..], "You can use this file"))
    );
}

#[test]
fn parses_unquoted_license() {
    assert_eq!(goal(&b"license: MIT-8.0\n"[..]), Ok((&[][..], "MIT-8.0")));
}
