use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;


pub fn to_actual_os_string(s: impl AsRef<OsStr>) -> Vec<u16> {
    s.as_ref().encode_wide().chain(once(0)).collect()
}