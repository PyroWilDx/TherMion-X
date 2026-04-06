use super::interface::GpuMonitor;
use crate::ffi::strings::to_c_string;
use crate::gpu::amd::ffi::{ADLX_GetGPU, ADLX_InitPerformanceService};
use std::ffi::c_void;
use std::ptr::null_mut;

mod ffi {
    use std::ffi::{c_char, c_int, c_void};

    unsafe extern "C" {
        pub fn ADLX_InitPerformanceService(svc: *mut *mut c_void) -> c_int;

        pub fn ADLX_GetGPU(gpu: *mut *mut c_void, name: *const c_char) -> c_int;
    }
}

pub struct AmdGpu {
    adlx_performance_svc: *mut c_void,
    gpu: *mut c_void,
}

impl GpuMonitor for AmdGpu {
    fn new(name: &str) -> Result<Self, String> {
        let mut adlx_performance_svc: *mut c_void = null_mut();
        let ok = unsafe { ADLX_InitPerformanceService(&mut adlx_performance_svc) };
        if ok != 0 {
            return Err(format!(
                "Failed to call ADLX_InitPerformanceService: {}",
                ok
            ));
        }

        let c_name = to_c_string(&name)?;

        let mut gpu: *mut c_void = null_mut();
        let ok = unsafe { ADLX_GetGPU(&mut gpu, c_name.as_ptr()) };
        if ok != 0 {
            return Err(format!(
                "Failed to call ADLX_GetGPU with name {}: {}",
                name, ok
            ));
        }

        return Ok(Self {
            adlx_performance_svc: adlx_performance_svc,
            gpu: gpu,
        });
    }
}
