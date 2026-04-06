mod adlx;
mod ffi;
mod gpu;

use crate::adlx::adlx::{get_gpu_names, init_adlx};
use crate::gpu::amd::AmdGpu;
use crate::gpu::interface::GpuMonitor;

fn main() {
    match init_adlx() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Failed to initialize ADLX: {}", err);
            return;
        }
    }

    let names = get_gpu_names();
    if names.is_err() {
        eprintln!("Failed to get GPU names: {}", names.unwrap_err());
        return;
    }

    let names = names.unwrap();

    for name in &names {
        println!("{}", name);
    }

    let _ = AmdGpu::new(&names[1]);
}
