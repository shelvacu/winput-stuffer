use std::io;

use windows::Win32::Foundation::LPARAM;
use windows::Win32::UI::WindowsAndMessaging as wm_sys;

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct WindowMessage(u32);

impl WindowMessage {
    pub fn register(message: impl AsRef<str>) -> io::Result<Self> {

        let res = unsafe { wm_sys::RegisterWindowMessageW(message.as_ref()) };
        if res == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self(res))
        }
    }

    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl PartialEq<u32> for WindowMessage {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

/// So that windowmessages can directly compare against the result of get_extra_info
impl PartialEq<LPARAM> for WindowMessage {
    fn eq(&self, other: &LPARAM) -> bool {
        let inner:isize = self.0.try_into().unwrap();
        inner == other.0
    }
}

pub fn get_extra_info() -> LPARAM {
    unsafe { wm_sys::GetMessageExtraInfo() }
}