use super::key_handler::KeyHandler;
use super::{DataType, Result, Value};
use windows::Win32::System::Registry::HKEY;
use windows::Win32::System::Registry::KEY_READ;

pub struct Registry {
    root: HKEY,
    sub_key: String,
    value_name: String,
}

impl Registry {
    pub fn new(root: HKEY, sub_key: impl Into<String>, value_name: impl Into<String>) -> Self {
        Self {
            root,
            sub_key: sub_key.into(),
            value_name: value_name.into(),
        }
    }

    pub fn get_value(&self, data_type: DataType) -> Result<Value> {
        let ret = match data_type {
            DataType::DWord => Value::DWord(self.get_dword()?),
            _ => unimplemented!(),
        };

        Ok(ret)
    }

    pub fn get_dword(&self) -> Result<u32> {
        let handler = KeyHandler::open(self.root, &self.sub_key, KEY_READ)?;
        handler.get_dword(&self.value_name)
    }
}
