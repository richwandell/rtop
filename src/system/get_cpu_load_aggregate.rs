use systemstat::{System, Platform, saturating_sub_bytes, CPULoad, PlatformCpuLoad};
use std::time::Duration;
use std::thread;

pub fn get_cpu_load_aggregate() -> CPULoad {
    let sys = System::new();

    return match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            thread::sleep(Duration::from_secs_f64(0.2));
            cpu.done().unwrap()
        },
        Err(x) => CPULoad {
            user: 0.0,
            nice: 0.0,
            system: 0.0,
            interrupt: 0.0,
            idle: 0.0,
            platform: PlatformCpuLoad {}
        }
    }
}