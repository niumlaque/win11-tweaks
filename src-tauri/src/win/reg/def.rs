use super::DataType;
use windows::Win32::System::Registry::{HKEY, HKEY_CURRENT_USER};

#[derive(Debug)]
pub enum Root {
    CurrentUser,
}

impl std::fmt::Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let root = match self {
            Root::CurrentUser => "HKCU",
        };

        root.fmt(f)
    }
}

impl From<HKEY> for Root {
    fn from(value: HKEY) -> Self {
        match value {
            HKEY_CURRENT_USER => Self::CurrentUser,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct RegDef {
    pub root: Root,
    pub sub_key: String,
    pub value_name: String,
    pub data_type: DataType,
}

impl RegDef {
    fn new(
        root: Root,
        sub_key: impl Into<String>,
        value_name: impl Into<String>,
        data_type: DataType,
    ) -> Self {
        Self {
            root,
            sub_key: sub_key.into(),
            value_name: value_name.into(),
            data_type,
        }
    }

    pub fn hkcu(
        sub_key: impl Into<String>,
        value_name: impl Into<String>,
        data_type: DataType,
    ) -> Self {
        Self::new(Root::CurrentUser, sub_key, value_name, data_type)
    }

    pub fn root(&self) -> HKEY {
        match self.root {
            Root::CurrentUser => HKEY_CURRENT_USER,
        }
    }
}

impl std::fmt::Display for RegDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value_name.is_empty() {
            write!(f, "{}\\{}", self.root, self.sub_key)
        } else {
            write!(f, "{}\\{}\\{}", self.root, self.sub_key, self.value_name)
        }
    }
}
