use crate::adlx::adlx::ffi::{ADLX_FreeGPUNames, ADLX_GetGPUNames, ADLX_Init};
use crate::ffi::strings::from_c_strings;
use std::ffi::{c_char, c_int};
use std::ptr::null_mut;

mod ffi {
    use std::ffi::{c_char, c_int};

    unsafe extern "C" {
        pub fn ADLX_Init() -> c_int;

        pub fn ADLX_GetGPUNames(names: *mut *mut *mut c_char, count: *mut c_int) -> c_int;
        pub fn ADLX_FreeGPUNames(names: *mut *mut c_char, count: c_int) -> c_int;
    }
}

pub fn init_adlx() -> Result<(), String> {
    let ok = unsafe { ADLX_Init() };
    if ok != 0 {
        return Err(format!("Failed to call ADLX_Init: {}", ok));
    }

    return Ok(());
}

pub fn get_gpu_names() -> Result<Vec<String>, String> {
    let mut c_names: *mut *mut c_char = null_mut();
    let mut count: c_int = 0;

    let ok = unsafe { ADLX_GetGPUNames(&mut c_names, &mut count) };
    if ok != 0 {
        return Err(format!("Failed to call ADLX_GetGPUNames: {}", ok));
    }

    let res = unsafe {
        let names = from_c_strings(c_names as *const *const c_char, count as usize);

        let _ = ADLX_FreeGPUNames(c_names, count);

        names?
    };

    return Ok(res);
}
