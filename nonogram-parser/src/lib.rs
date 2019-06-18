pub mod parser;

// type ParseError = String;

// type Result<T> = std::result::Result<T, ParseError>;

// fn parse_key_string(key: &str) -> impl Fn(&str) -> Result<String, String> {
//     let saved_key: String = key.to_string();
//     |line| {
//         let key_and_rest = line.splitn(2, " ").collect::<Vec<_>>();
//         // let a: i32 = key_and_rest[0];
//         let line_key: &str = key_and_rest.get(0).ok_or("No key found".to_string())?;
//         let rest: &str = key_and_rest.get(0).ok_or("No value found".to_string())?;
//         if line_key != saved_key {
//             return Err(format!(
//                 "Wrong key: expected \"{}\" but got \"{}\"",
//                 key, line_key
//             ));
//         }
//         Ok(rest.to_string())
//     }
// }

#[macro_use]
extern crate nom;

use nom::{space1, is_not, named, tag, call, map_res, take_while};

named!(string<&str, &str>,
       delimited!(
           tag!("\""),
           is_not!("\""),
           tag!("\"")
       )
);

named!(integer<&str, u32>,
       map_res!(take_while!(|c: char| c.is_ascii_digit()), int_from_str)
);

fn int_from_str(input: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(input, 10)
}

macro_rules! parse_key_string {
    ( $name:ident ) => {
        named!(pub $name<&str, &str>,
               do_parse!(
                   tag!(stringify!($name)) >>
                       call!(space1) >>
                       value: call!(string) >>
                       (value)
               )
        );
    }
}

macro_rules! parse_key_int {
    ( $name:ident ) => {
        named!(pub $name<&str, u32>,
               do_parse!(
                   tag!(stringify!($name)) >>
                       call!(space1) >>
                       value: call!(integer) >>
                       (value)
               )
        );
    }
}

parse_key_string!(catalogue);
parse_key_string!(title);
parse_key_string!(by);
parse_key_string!(copyright);
parse_key_int!(width);
parse_key_int!(height);

pub fn main() {
    println!("{:?}", catalogue("catalogue \"This is a catalogue\""));
    println!("{:?}", width("width 12"));
}
