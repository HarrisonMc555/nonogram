use nom::{alpha, map_res, newline, space0 as space};
use std::str;

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
                    >> char!('"')
                    >> val: map_res!(take_until!("\"\n"),
                                     complete_byte_slice_to_str)
                    >> char!('"')
                    >> newline
                    >> (val)
            )
        );
    };
}

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

one_line_key_string!(catalogue);
one_line_key_string!(title);
one_line_key_string!(by);
one_line_key_string!(copyright);
one_line_key_string!(goal);

pub fn main() {
    // assert_eq!(
    //     oneline_key_string_value(&b"key: value\n"[..]),
    //     Ok((&[][..], ("key", "value")))
    // );

    assert_eq!(
        catalogue(&b"catalogue: \"mynonograms 1.my\"\n"[..]),
        Ok((&[][..], "mynonograms 1.my"))
    );

    println!("parser passed");
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
