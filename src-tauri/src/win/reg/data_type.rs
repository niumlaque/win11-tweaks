use windows::Win32::System::Registry::REG_VALUE_TYPE;
use windows::Win32::System::Registry::{
    REG_BINARY, REG_DWORD, REG_EXPAND_SZ, REG_MULTI_SZ, REG_QWORD, REG_SZ,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DataType {
    Binary,
    DWord,
    QWord,
    String,
    MultiString,
    ExpandString,
}

impl DataType {
    pub(crate) fn value(&self) -> REG_VALUE_TYPE {
        match self {
            Self::Binary => REG_BINARY,
            Self::DWord => REG_DWORD,
            Self::QWord => REG_QWORD,
            Self::String => REG_SZ,
            Self::MultiString => REG_MULTI_SZ,
            Self::ExpandString => REG_EXPAND_SZ,
        }
    }

    pub fn str(&self) -> &'static str {
        match self {
            Self::Binary => "REG_BINARY",
            Self::DWord => "REG_DWORD",
            Self::QWord => "REG_QWORD",
            Self::String => "REG_SZ",
            Self::MultiString => "REG_MULTI_SZ",
            Self::ExpandString => "REG_EXPAND_SZ",
        }
    }
}

impl TryFrom<REG_VALUE_TYPE> for DataType {
    type Error = super::Error;

    fn try_from(value: REG_VALUE_TYPE) -> Result<Self, Self::Error> {
        let v = match value {
            REG_BINARY => Self::Binary,
            REG_DWORD => Self::DWord,
            REG_QWORD => Self::QWord,
            REG_SZ => Self::String,
            REG_MULTI_SZ => Self::MultiString,
            REG_EXPAND_SZ => Self::ExpandString,
            v => return Err(Self::Error::UnknownDataType(v)),
        };

        Ok(v)
    }
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str())
    }
}
