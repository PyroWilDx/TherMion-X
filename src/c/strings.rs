use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub unsafe fn from_c_strings(
    strs: *const *const c_char,
    count: usize,
) -> Result<Vec<String>, String> {
    if strs.is_null() {
        return Err("Pointer is null".to_string());
    }

    let mut res = Vec::with_capacity(count);

    for i in 0..count {
        res.push(unsafe { from_c_string(*strs.add(i))? });
    }

    return Ok(res);
}

pub unsafe fn from_c_string(str: *const c_char) -> Result<String, String> {
    if str.is_null() {
        return Err("Pointer is null".to_string());
    }

    let c = unsafe { CStr::from_ptr(str) };
    return Ok(c.to_string_lossy().into_owned());
}

pub fn to_c_string(str: &str) -> Result<CString, String> {
    return CString::new(str).map_err(|err| format!("String contains null byte(s): {}", err));
}
