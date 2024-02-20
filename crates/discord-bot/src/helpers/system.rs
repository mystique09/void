use sysinfo::System;

pub async fn cpu_usage() -> f32 {
    let mut system = System::new();
    system.refresh_cpu();

    let usage = match system.cpus().first() {
        Some(cpu) => cpu.cpu_usage().log10(),
        None => 0.
    };

    usage
}

type FreeMemoryAvailable = f32;
type TotalMemory = f32;
type MemoryUsageResult = (FreeMemoryAvailable, TotalMemory);

pub async fn memory_usage() -> MemoryUsageResult {
    let mut system = System::new_all();
    system.refresh_cpu();

    let mem_usage = system.free_memory() as f32 / 1000.;
    let total_memory = system.total_memory() as f32 / 1000.;

    (mem_usage, total_memory)
}