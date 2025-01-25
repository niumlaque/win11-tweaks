use std::num::ParseIntError;
use windows::Win32::Foundation::WIN32_ERROR;
use windows::Win32::System::Registry::REG_VALUE_TYPE;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    #[allow(clippy::enum_variant_names)]
    Win32Error(windows::core::Error),
    SubkeyNotFound(String),
    ValueNameNotFound(String),
    UnknownDataType(REG_VALUE_TYPE),
    UnexpectedDataType((&'static str, &'static str)), // expected, actual
    ParseIntError(ParseIntError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Win32Error(e) => write!(f, "Win32 Error: {}", e),
            Self::SubkeyNotFound(k) => write!(f, "Subkey not found ({k})"),
            Self::ValueNameNotFound(v) => write!(f, "Value Name not found ({v})"),
            Self::UnknownDataType(v) => write!(f, "Unknown data type: {}", v.0),
            Self::UnexpectedDataType((expected, actual)) => {
                write!(
                    f,
                    "Unexpected data type (expected: {expected}, actual: {actual})"
                )
            }
            Self::ParseIntError(e) => e.fmt(f),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseIntError(value)
    }
}

impl From<WIN32_ERROR> for Error {
    fn from(value: WIN32_ERROR) -> Self {
        Error::Win32Error(windows::core::Error::from(value))
    }
}
