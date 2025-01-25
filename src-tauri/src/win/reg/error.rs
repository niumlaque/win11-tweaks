use windows::Win32::Foundation::WIN32_ERROR;
use windows::Win32::System::Registry::REG_VALUE_TYPE;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    #[allow(clippy::enum_variant_names)]
    Win32Error(WIN32_ERROR),
    SubkeyNotFound(String),
    UnknownDataType(REG_VALUE_TYPE),
    UnexpectedDataType((&'static str, &'static str)), // expected, actual
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Win32Error(e) => write!(f, "{}", e.0),
            Self::SubkeyNotFound(k) => write!(f, "Subkey not found ({k})"),
            Self::UnknownDataType(v) => write!(f, "Unknown data type: {}", v.0),
            Self::UnexpectedDataType((expected, actual)) => {
                write!(
                    f,
                    "Unexpected data type (expected: {expected}, actual: {actual})"
                )
            }
        }
    }
}
