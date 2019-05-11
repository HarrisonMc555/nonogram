

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
    assert_eq!(
        goal("goal: \"00110101\"\n"),
        Ok(("", "00110101".chars().collect()))
    );
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

#[test]
fn parses_width() {
    assert_eq!(width("width: 48\n"), Ok(("", 48)));
}

#[test]
fn parses_height() {
    assert_eq!(height("height: 62\n"), Ok(("", 62)));
}

#[test]
fn parses_rows() {
    assert_eq!(
        rows("rows: 1a,2b,3\n"),
        Ok(("", vec![(1, Some('a')), (2, Some('b')), (3, None)]))
    );
}

#[test]
fn parses_columns() {
    assert_eq!(
        columns("columns: 17f,323,4z\n"),
        Ok(("", vec![(17, Some('f')), (323, None), (4, Some('z'))]))
    );
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
