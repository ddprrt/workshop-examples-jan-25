use std::{fs::File, io::Read, num::ParseIntError};

fn open_file(file_name: Option<&str>) -> Result<String, std::io::Error> {
    let mut file = File::open(file_name.unwrap_or("input.txt"))?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

#[derive(Debug)]
enum NumberFromFileError {
    ParseError(ParseIntError),
    IoError(std::io::Error),
}

impl std::fmt::Display for NumberFromFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberFromFileError::ParseError(err) => {
                write!(f, "Error parsing number from File {}", err)
            }
            NumberFromFileError::IoError(err) => {
                write!(f, "Error reading number from File {}", err)
            }
        }
    }
}

impl std::error::Error for NumberFromFileError {}

impl From<ParseIntError> for NumberFromFileError {
    fn from(val: ParseIntError) -> Self {
        NumberFromFileError::ParseError(val)
    }
}

impl From<std::io::Error> for NumberFromFileError {
    fn from(val: std::io::Error) -> Self {
        NumberFromFileError::IoError(val)
    }
}

impl From<ErrorWithId> for NumberFromFileError {
    fn from(val: ErrorWithId) -> Self {
        NumberFromFileError::IoError(val.err)
    }
}

struct ErrorWithId {
    id: i32,
    err: std::io::Error,
}

impl From<(i32, std::io::Error)> for ErrorWithId {
    fn from(value: (i32, std::io::Error)) -> Self {
        ErrorWithId {
            id: value.0,
            err: value.1,
        }
    }
}

type _MyResult<T> = Result<T, Box<dyn std::error::Error>>;

fn get_number_from_file(file_name: Option<&str>) -> Result<u64, NumberFromFileError> {
    let id = 1;
    let buf = open_file(file_name).map_err(|e| ErrorWithId::from((id, e)))?;
    let num = buf.trim().parse()?;
    Ok(num)
}

fn accepts_option(_i: impl Into<Option<i64>>) {}

fn accepts_string(_s: impl ToString) {
    let _s = _s.to_string();
}

fn main() {
    accepts_string("Hello");
    accepts_string(100);
    accepts_option(100);
    accepts_option(None);

    match get_number_from_file(Some("user_id.txt")) {
        Ok(result) => println!("{result}"),
        Err(NumberFromFileError::ParseError(e)) => eprintln!("1: {}", e),
        Err(NumberFromFileError::IoError(e)) => eprintln!("2: {}", e),
    };

    if let Ok(value) = open_file(None) {
        println!("{}", value);
    }
    match open_file(Some("input-not-existent.txt")) {
        Ok(value) => println!("{}", value),
        Err(err) => eprintln!("{}", err),
    };
}
