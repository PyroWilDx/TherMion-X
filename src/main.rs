use std::{thread, time::Duration};

unsafe extern "C" {
    fn GetGPUUsage(index: i32) -> i32;
}

fn main() {
    let mut usages: Vec<i32> = Vec::new();
    let mut elapsed: u128 = 0;

    loop {
        let usage = unsafe { GetGPUUsage(1) };
        usages.push(usage);

        thread::sleep(Duration::from_millis(10));
        elapsed += 10;

        if elapsed >= 1000 {
            if !usages.is_empty() {
                let sum: i32 = usages.iter().sum();
                println!("GPU Usage: {:.2}%", sum as f64 / usages.len() as f64);
            }

            usages.clear();
            elapsed = 0;
        }
    }
}
