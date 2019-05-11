use nom::{anychar, digit1, map_res, space0 as space, IResult};
use std::str;

pub fn main() {
    assert_eq!(
        catalogue("catalogue: \"mynonograms 1.my\"\n"),
        Ok(("", "mynonograms 1.my"))
    );

    assert_eq!(
        license("license: \"A restrictive license\"\n"),
        Ok(("", "A restrictive license"))
    );

    assert_eq!(width("width: 48\n"), Ok(("", 48)));

    assert_eq!(
        rows("rows: 1a,2b,3\n"),
        Ok(("", vec![(1, Some('a')), (2, Some('b')), (3, None)]))
    );

    assert_eq!(
        goal("goal: \"00110101\"\n"),
        Ok(("", "00110101".chars().collect()))
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

// macro_rules! must come before use
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

macro_rules! named_key_and_int {
    ( $key:ident ) => {
        named!($key<&str, i64>, call!(key_and_int, stringify!($key)));
    };
}

macro_rules! named_key_and_sequence_of_lines {
    ( $key:ident ) => {
        named!(
            $key <&str, Vec<(i64, Option<char>)>>,
            call!(key_and_sequence_of_lines, stringify!($key))
        );
    };
}

// Keys
named_key_and_string!(catalogue);
named_key_and_string!(title);
named_key_and_string!(by);
named_key_and_string!(copyright);
named_key_and_possibly_unquoted_string!(license);
named_key_and_int!(width);
named_key_and_int!(height);
named_key_and_sequence_of_lines!(rows);
named_key_and_sequence_of_lines!(columns);
named!(
    goal<&str, Vec<char>>,
    do_parse!(
        tag!("goal")
            >> opt!(space)
            >> char!(':')
            >> opt!(space)
            >> value: delimited!(char!('"'), call!(goal_sequence), char!('"'))
            >> tag!("\n")
            >> (value)
    )
);

// Helper functions
fn key_and_string<'a>(input: &'a str, key: &str) -> IResult<&'a str, &'a str> {
    do_parse!(
        input,
        tag!(key)
            >> opt!(space)
            >> char!(':')
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
            >> char!(':')
            >> opt!(space)
            >> value: call!(unquoted_string)
            >> tag!("\n")
            >> (value)
    )
}

fn key_and_int<'a>(input: &'a str, key: &str) -> IResult<&'a str, i64> {
    do_parse!(
        input,
        tag!(key)
            >> opt!(space)
            >> char!(':')
            >> opt!(space)
            >> value: call!(int)
            >> tag!("\n")
            >> (value)
    )
}

fn key_and_sequence_of_lines<'a>(
    input: &'a str,
    key: &str,
) -> IResult<&'a str, Vec<(i64, Option<char>)>> {
    do_parse!(
        input,
        tag!(key)
            >> opt!(space)
            >> char!(':')
            >> opt!(space)
            >> value: call!(sequence_of_lines)
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

named!(int<&str, i64>,
       map_res!(digit1, |s: &str| s.parse::<i64>())
);

named!(sequence_of_lines<&str, Vec<(i64, Option<char>)>>,
       separated_list!(tag!(","), call!(hint))
);

named!(
    hint <&str, (i64, Option<char>)>,
    pair!(call!(int), opt!(one_letter))
);

named!(
    one_letter<&str, char>,
    verify!(anychar, |c: char| c.is_ascii_lowercase())
);

named!(
    goal_sequence<&str, Vec<char>>,
    many0!(call!(answer))
);

named!(answer<&str, char>, alt!(one_of!("01") | call!(one_letter)));

// Color
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
