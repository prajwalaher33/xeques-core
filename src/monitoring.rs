use sysinfo::System;

pub struct Metrics {
    pub cpu: f32,
    pub memory: u64,
}

pub fn get_metrics() -> Metrics {
    let mut system = System::new_all();
    system.refresh_all();

    // CPU usage (may require 2 refreshes to stabilize)
    let cpu = system.global_cpu_info().cpu_usage();
    let memory = system.used_memory();

    Metrics {
        cpu,
        memory,
    }
}
