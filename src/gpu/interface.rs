pub trait GpuMonitor {
    fn new(name: &str) -> Result<Self, String>
    where
        Self: Sized;
}
