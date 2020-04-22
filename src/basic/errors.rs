use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::Utf8Error;

/// ==============================first===================================
#[derive(Debug)]
struct SuperError {
    side: SuperErrorSideKick,
}

impl fmt::Display for SuperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl Error for SuperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.side)
    }
}

#[derive(Debug)]
struct SuperErrorSideKick;

impl fmt::Display for SuperErrorSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperErrorSideKick is here!")
    }
}

impl Error for SuperErrorSideKick {}

fn get_super_error() -> Result<(), SuperError> {
    Err(SuperError {
        side: SuperErrorSideKick,
    })
}

///============================second==================================

type CResult<T> = Result<T, CustomError>;

#[derive(Debug)]
enum CustomError {
    IoError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
    ParseIntError(std::num::ParseIntError),
}

impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            CustomError::IoError(e) => Some(e),
            CustomError::Utf8Error(e) => Some(e),
            CustomError::ParseIntError(e) => Some(e),
        }
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            CustomError::IoError(e) => e.fmt(f),
            CustomError::Utf8Error(e) => e.fmt(f),
            CustomError::ParseIntError(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for CustomError {
    fn from(e: std::io::Error) -> Self {
        CustomError::IoError(e)
    }
}

impl From<Utf8Error> for CustomError {
    fn from(e: Utf8Error) -> Self {
        CustomError::Utf8Error(e)
    }
}

impl From<ParseIntError> for CustomError {
    fn from(e: ParseIntError) -> Self {
        CustomError::ParseIntError(e)
    }
}

/// a way
fn read_file_1(path: &str) -> std::result::Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

fn to_utf8_1(v: &[u8]) -> std::result::Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(v)
}

fn to_u32_1(v: &str) -> std::result::Result<u32, std::num::ParseIntError> {
    v.parse::<u32>()
}

/// evolved
fn read_file(path: &str) -> CResult<String> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

fn to_utf8(v: &[u8]) -> CResult<&str> {
    let string = std::str::from_utf8(v)?;
    Ok(string)
}

fn to_u32(s: &str) -> CResult<u32> {
    let num = s.parse::<u32>()?;
    Ok(num)
}

/// The '?' operator automatically converts the underlying error type to our
/// custom error type by calling `Into<CliError>::into` which is automatically provided when
/// implementing `From`. The compiler then infers which implementation of `Into` should be used.
fn proc_second() -> CResult<u32> {
    // a way, but unable to have a return value
    let v = read_file_1("./number")?;
    let s = to_utf8_1(v.as_ref())?;
    let number = to_u32_1(s)?;
    println!("number: {}", number);

    // evolved
    let content = read_file("./number")?;
    let string = to_utf8(content.as_ref())?;
    to_u32(string)
}

pub fn proc_errors() {
    // 1
    match get_super_error() {
        Err(e) => {
            println!("Error: {}", e.to_string());
            println!("Caused by: {}", e.source().unwrap());
        }
        _ => println!("No error"),
    }

    // 2
    println!("proc second: {:?}", proc_second());
}
