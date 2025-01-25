pub mod reg;
use windows::core::*;
pub use windows::Win32::System::Registry::REG_DWORD;
use windows::Win32::System::WindowsProgramming::GetUserNameW;
use windows::Win32::UI::WindowsAndMessaging::*;

pub fn message_box(text: impl AsRef<str>, caption: impl AsRef<str>) {
    unsafe {
        MessageBoxW(
            None,
            &HSTRING::from(text.as_ref()),
            &HSTRING::from(caption.as_ref()),
            MB_OK,
        );
    }
}

pub fn get_username() -> std::result::Result<String, windows::core::Error> {
    // バッファを用意 (Windowsのユーザー名は通常256文字以下)
    let mut buffer = vec![0u16; 256];
    let mut size = buffer.len() as u32;

    unsafe {
        if GetUserNameW(PWSTR(buffer.as_mut_ptr()), &mut size).is_ok() {
            Ok(String::from_utf16_lossy(&buffer[..(size as usize - 1)]))
        } else {
            Err(windows::core::Error::from_win32())
        }
    }
}
