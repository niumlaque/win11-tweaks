use super::{DataType, Error, Result, Root};
use windows::core::HSTRING;
use windows::Win32::Foundation::{ERROR_SUCCESS, WIN32_ERROR};
use windows::Win32::System::Registry::REG_SAM_FLAGS;
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW,
};
use windows::Win32::System::Registry::{HKEY, REG_OPTION_NON_VOLATILE, REG_VALUE_TYPE};

pub(super) struct KeyHandler {
    key: HKEY,
}

impl KeyHandler {
    pub fn open(root: HKEY, sub_key: &str, samdesired: REG_SAM_FLAGS) -> Result<Self> {
        let mut k: HKEY = HKEY::default();
        unsafe {
            let ret = RegOpenKeyExW(root, &HSTRING::from(sub_key), 0, samdesired, &mut k);

            if ret != ERROR_SUCCESS {
                if check_no_key_error(ret) {
                    return Err(Error::SubkeyNotFound(format!(
                        "{}\\{}",
                        Root::from(root),
                        sub_key
                    )));
                } else {
                    return Err(Error::from(ret));
                }
            }
        }

        Ok(Self { key: k })
    }

    pub fn create_or_open(root: HKEY, sub_key: &str, samdesired: REG_SAM_FLAGS) -> Result<Self> {
        let mut k: HKEY = HKEY::default();
        unsafe {
            let ret = RegCreateKeyExW(
                root,
                &HSTRING::from(sub_key),
                0,
                &HSTRING::default(),
                REG_OPTION_NON_VOLATILE,
                samdesired,
                None,
                &mut k,
                None,
            );

            if ret != ERROR_SUCCESS {
                return Err(Error::from(ret));
            }
        }

        Ok(Self { key: k })
    }

    fn get_value_info(&self, value_name: &str) -> Result<Option<ValueInfo>> {
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
                if check_no_key_error(ret) {
                    return Ok(None);
                } else {
                    return Err(Error::from(ret));
                }
            }
        }

        let ret = ValueInfo::new(DataType::try_from(data_type)?, len);
        Ok(Some(ret))
    }

    pub fn get_dword(&self, value_name: &str) -> Result<u32> {
        let vi = self.get_value_info(value_name)?;
        if let Some(vi) = vi {
            if vi.data_type() != DataType::DWord {
                return Err(Error::UnexpectedDataType((
                    "REG_DWORD",
                    vi.data_type().str(),
                )));
            }

            let buf = self.get_value(value_name, &vi)?;
            let ret = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
            Ok(ret)
        } else {
            Err(Error::ValueNameNotFound(value_name.into()))
        }
    }

    pub fn get_string(&self, value_name: &str) -> Result<String> {
        let vi = self.get_value_info(value_name)?;
        if let Some(vi) = vi {
            if vi.data_type() != DataType::String {
                return Err(Error::UnexpectedDataType(("REG_SZ", vi.data_type().str())));
            }

            let buf = self.get_value(value_name, &vi)?;
            let utf16_data: Vec<u16> = buf
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .collect();
            let ret = String::from_utf16_lossy(&utf16_data)
                .trim_end_matches('\0')
                .to_string();
            Ok(ret)
        } else {
            Err(Error::ValueNameNotFound(value_name.into()))
        }
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
                return Err(Error::from(ret));
            }
        }

        Ok(buf)
    }

    pub fn set_dword(&self, value_name: &str, value: u32) -> Result<()> {
        let vi = self.get_value_info(value_name)?;
        if let Some(vi) = vi {
            if vi.data_type() != DataType::DWord {
                return Err(Error::UnexpectedDataType((
                    "REG_DWORD",
                    vi.data_type().str(),
                )));
            }
        }

        let buf = value.to_ne_bytes();
        self.set_value(value_name, DataType::DWord, &buf)?;
        Ok(())
    }

    pub fn set_string(&self, value_name: &str, value: &str) -> Result<()> {
        let vi = self.get_value_info(value_name)?;
        if let Some(vi) = vi {
            if vi.data_type() != DataType::String {
                return Err(Error::UnexpectedDataType(("REG_SZ", vi.data_type().str())));
            }
        }

        let data_utf16: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();
        let buf: Vec<u8> = data_utf16.iter().flat_map(|&u| u.to_le_bytes()).collect();
        self.set_value(value_name, DataType::String, &buf)
    }

    fn set_value(&self, value_name: &str, data_type: DataType, buffer: &[u8]) -> Result<()> {
        unsafe {
            let ret = RegSetValueExW(
                self.key,
                &HSTRING::from(value_name),
                0,
                data_type.value(),
                Some(buffer),
            );

            if ret != ERROR_SUCCESS {
                if check_no_key_error(ret) {
                    return Err(Error::ValueNameNotFound(value_name.into()));
                } else {
                    return Err(Error::from(ret));
                }
            }
        }
        Ok(())
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

fn check_no_key_error(e: WIN32_ERROR) -> bool {
    let err = windows::core::Error::from(e);
    // use windows::Win32::Foundation::REGDB_E_CLASSNOTREG;
    // let no_key_err = windows::core::Error::from(REGDB_E_CLASSNOTREG);
    use windows::Win32::Foundation::ERROR_FILE_NOT_FOUND;
    let no_key_err = windows::core::Error::from(ERROR_FILE_NOT_FOUND);
    err == no_key_err
}
