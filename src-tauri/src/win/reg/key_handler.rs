use super::{DataType, Error, Result};
use windows::core::HSTRING;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::REG_SAM_FLAGS;
use windows::Win32::System::Registry::{RegCloseKey, RegOpenKeyExW, RegQueryValueExW};
use windows::Win32::System::Registry::{HKEY, REG_VALUE_TYPE};

pub(super) struct KeyHandler {
    key: HKEY,
}

impl KeyHandler {
    pub fn open(root: HKEY, sub_key: &str, samdesired: REG_SAM_FLAGS) -> Result<Self> {
        let mut k: HKEY = HKEY::default();
        unsafe {
            let ret = RegOpenKeyExW(root, &HSTRING::from(sub_key), 0, samdesired, &mut k);
            if ret != ERROR_SUCCESS {
                return Err(Error::Win32Error(ret));
            }
        }

        Ok(Self { key: k })
    }

    fn get_value_info(&self, value_name: &str) -> Result<ValueInfo> {
        let mut data_type = REG_VALUE_TYPE(0);
        let mut len = 0u32;
        unsafe {
            let ret = RegQueryValueExW(
                self.key,
                &HSTRING::from(value_name),
                None,
                Some(&mut data_type),
                None,
                Some(&mut len),
            );

            if ret != ERROR_SUCCESS {
                return Err(Error::Win32Error(ret));
            }
        }

        let ret = ValueInfo::new(DataType::try_from(data_type)?, len);
        Ok(ret)
    }

    pub fn get_dword(&self, value_name: &str) -> Result<u32> {
        let vi = self.get_value_info(value_name)?;
        if vi.data_type() != DataType::DWord {
            return Err(Error::UnexpectedDataType((
                "REG_DWORD",
                vi.data_type().str(),
            )));
        }

        let buf = self.get_value(value_name, &vi)?;
        let ret = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        Ok(ret)
    }

    fn get_value(&self, value_name: &str, vi: &ValueInfo) -> Result<Vec<u8>> {
        let mut len = vi.len();
        let mut buf = vec![0u8; len as usize];
        let mut data_type = vi.data_type().value();

        unsafe {
            let ret = RegQueryValueExW(
                self.key,
                &HSTRING::from(value_name),
                None,
                Some(&mut data_type),
                Some(buf.as_mut_ptr()),
                Some(&mut len),
            );

            if ret != ERROR_SUCCESS {
                return Err(Error::Win32Error(ret));
            }
        }

        Ok(buf)
    }
}

impl Drop for KeyHandler {
    fn drop(&mut self) {
        let null = HKEY::default();
        if self.key != null {
            unsafe {
                let _ = RegCloseKey(self.key);
                self.key = null;
            }
        }
    }
}

struct ValueInfo {
    data_type: DataType,
    len: u32,
}

impl ValueInfo {
    pub fn new(data_type: DataType, len: u32) -> Self {
        Self { data_type, len }
    }

    pub fn data_type(&self) -> DataType {
        self.data_type
    }

    pub fn len(&self) -> u32 {
        self.len
    }
}
