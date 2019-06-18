// use nonogram;

// Structs
pub struct NonogramFile {
    pub width: u64,
    pub height: u64,
    pub rows: Vec<Hint>,
    pub columns: Vec<Hint>,

    pub goal: Option<Cell>,

    pub catalogue: Option<String>,
    pub title: Option<String>,
    pub by: Option<String>,
    pub copyright: Option<String>,
    pub license: Option<String>,
    pub colors: Option<Vec<ColorDefinition>>,
}

pub enum Cell {
    Empty,
    Filled(Option<ColorCode>),
}

pub struct Hint {
    pub length: u64,
    pub color: Option<ColorCode>,
}

pub struct ColorCode(pub char);

pub struct ColorDefinition {
    pub code: ColorCode,
    pub color: Color,
}

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

// Builder
pub struct NonogramFileBuilder {
    pub width: Option<u64>,
    pub height: Option<u64>,
    pub rows: Option<Vec<Hint>>,
    pub columns: Option<Vec<Hint>>,

    pub goal: Option<Vec<Cell>>,

    pub catalogue: Option<String>,
    pub title: Option<String>,
    pub by: Option<String>,
    pub copyright: Option<String>,
    pub license: Option<String>,
    pub colors: Option<Vec<ColorDefinition>>,
}

macro_rules! add_value_fn {
    ( $key:ident<&str> ) => {
        pub fn $key(&mut self, $key: &str) -> BuildResult {
            match self.$key {
                None => {
                    self.$key = Some($key.to_string());
                    Ok(self)
                }
                Some(_) => Err(Error::DuplicateValue(
                    stringify!($key).to_string())),
            }
        }
    };
    ( $key:ident<$type:ty> ) => {
        pub fn $key(&mut self, $key: $type) -> BuildResult {
            match self.$key {
                None => {
                    self.$key = Some($key);
                    Ok(self)
                }
                Some(_) => Err(Error::DuplicateValue(
                    stringify!($key).to_string())),
            }
        }
    };
}

macro_rules! add_value_fn_with_required {
    ( $key:ident<$type:ty> $(, $required:ident )* ) => {
        pub fn $key(&mut self, $key: $type) -> BuildResult {
            $(
                if self.$required.is_none() {
                    return Err(Error::ThisNeededRequiredValue(
                        stringify!($key).to_string(),
                        stringify!($required).to_string(),
                    ));
                }
            )*
            match self.$key {
                None => {
                    self.$key = Some($key);
                    Ok(self)
                }
                Some(_) => Err(Error::DuplicateValue(
                    stringify!($key).to_string())),
            }
        }
    };
}

impl NonogramFileBuilder {
    pub fn new() -> Self {
        NonogramFileBuilder {
            width: None,
            height: None,
            rows: None,
            columns: None,
            goal: None,
            catalogue: None,
            title: None,
            by: None,
            copyright: None,
            license: None,
            colors: None,
        }
    }

    add_value_fn!(width<u64>);
    add_value_fn!(height<u64>);

    add_value_fn_with_required!(rows<Vec<Hint>>, width, height);
    add_value_fn_with_required!(columns<Vec<Hint>>, width, height);
    add_value_fn_with_required!(goal<Vec<Cell>>, width, height);

    add_value_fn!(catalogue<&str>);
    add_value_fn!(title<&str>);
    add_value_fn!(by<&str>);
    add_value_fn!(copyright<&str>);
    add_value_fn!(license<&str>);
}

pub enum Error {
    DuplicateValue(String),
    ThisNeededRequiredValue(String, String),
}

type Result<T> = std::result::Result<T, Error>;
type BuildResult<'a> = Result<&'a NonogramFileBuilder>;

// Main
pub fn main() {
    let input = "key this is the value";
    println!("{:?}", first_word_and_rest(input));

    println!("finished parser");
}

fn first_word_and_rest(input: &str) -> (&str, &str) {
    let index = input.find(' ').unwrap_or(input.len());
    let (first, reset) = input.split_at(index);
    (first, reset.trim_start())
}
