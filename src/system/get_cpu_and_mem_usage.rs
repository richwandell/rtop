use systemstat::{System, Platform, saturating_sub_bytes, CPULoad, PlatformCpuLoad, Memory, PlatformMemory};
use std::time::Duration;
use std::thread;

pub fn get_cpu_and_mem_usage() -> (CPULoad, Memory) {
    let sys = System::new();

    let cpu = match sys.cpu_load_aggregate() {
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
    };

    let memory = match sys.memory() {
        Ok(mem) => {
            saturating_sub_bytes(mem.total, mem.free);
            mem
        },
        Err(x) => {
            Memory {
                total: Default::default(),
                free: Default::default(),
                platform_memory: PlatformMemory {
                    load: 0,
                    total_phys: Default::default(),
                    avail_phys: Default::default(),
                    total_pagefile: Default::default(),
                    avail_pagefile: Default::default(),
                    total_virt: Default::default(),
                    avail_virt: Default::default(),
                    avail_ext: Default::default()
                }
            }
        }
    };

    return (cpu, memory)
}