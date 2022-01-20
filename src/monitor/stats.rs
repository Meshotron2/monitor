use std::collections::HashMap;
use std::panic::resume_unwind;
use sysinfo::{ComponentExt, ProcessExt, SystemExt, System as Sys, ProcessorExt};
use crate::communication::http_requests::RequestSerializable;

/// Stores usage data relative to the node
pub struct NodeData {
    cores: usize,
    threads: usize,
    cpu_usage: f32,
    total_ram: u64,
    used_ram: u64,
    temperature: Vec<f32>,
}

impl NodeData {
    /// Populates a new NodeData struct with data retrieved with sysinfo
    pub fn new() -> Self {
        let s = Sys::new_all();

        let used_ram = s.used_memory();
        let total_ram = s.total_memory();
        let cores = s.physical_core_count().unwrap();
        let threads = s.processors().len();
        let cpu_usage = s.global_processor_info().cpu_usage();

        let temperature = s.components().iter()
            .filter(|comp| comp.label().starts_with("Core "))
            .map(|comp| comp.temperature())
            .collect();

        return NodeData { cores, threads, cpu_usage, total_ram, used_ram, temperature };
    }

    /// Updates volatile data.
    /// - CPU usage
    /// - used RAM
    /// - temperature (ºC)
    pub fn update(&mut self, sys: &mut Sys) {
        sys.refresh_all();

        self.cpu_usage = sys.global_processor_info().cpu_usage();
        self.used_ram = sys.used_memory();

        self.temperature = sys.components().iter()
            .filter(|comp| comp.label().starts_with("Core "))
            .map(|comp| comp.temperature())
            .collect();
    }
}

/// Stores data relative to a process
/// This includes both data regarding the thread itself (pid CPU usage and RAM)
/// and progress of whatever process is running
pub struct ProcData {
    pid: i32,
    cpu: f32,
    ram: u64,
    progress: usize,
}

impl ProcData {
    /// Generates a new HashMap with all processes with the given name
    pub fn new(proc_name: &str, sys: &mut Sys) -> HashMap<i32, Self> {
        sys.refresh_all();

        return HashMap::from_iter(sys.processes().iter()
            // .map(|(pid, proc)| proc)
            .filter(|(_pid, p)| p.name() == proc_name)
            .map(|(pid, p)| (*pid, Self {
                pid: *pid,
                cpu: p.cpu_usage(),
                ram: p.memory(),
                progress: 0,
            }))
        );
    }

    /// Updates the volatile data of the process
    /// - RAM usage
    /// - CPU usage
    /// - progress
    pub fn update(&mut self, progress: usize, sys: &mut Sys) {
        sys.refresh_all();

        let proc_opt = sys.process(self.pid);

        match proc_opt {
            Some(p) => {
                self.ram = p.memory();
                self.cpu = p.cpu_usage();
                self.progress = progress;
            }
            None => {
                self.ram = 0;
                self.cpu = 0.0;
                self.progress = 0;
            }
        };
    }
}

impl RequestSerializable for NodeData {
    fn serialize(&self) -> String {
        /*
        cores: usize,
    threads: usize,
    cpu_usage: f32,
    total_ram: u64,
    used_ram: u64,
    temperature: Vec<f32>,
         */
        let cores = self.cores.to_string();
        let threads = self.threads.to_string();
        let cpu_usage = self.cpu_usage.to_string();
        let total_ram = self.total_ram.to_string();
        let used_ram = self.used_ram.to_string();
        let mut temperature = String::new();

        for v in &self.temperature {
            temperature.push_str(&*v.to_string());
            temperature.push_str("_");
        }

        let mut res = String::with_capacity(61 + cores.len() + threads.len()
            + cpu_usage.len() + total_ram.len() + used_ram.len() + temperature.len());

        res = "?cores=".to_owned() + &cores + "&threads=" + &threads + "&cpu_usage=" + &cpu_usage +
            "&total_ram=" + &total_ram + "&used_ram=" + &used_ram + "&temperature=" + &temperature;

        return res;
    }
}

impl RequestSerializable for ProcData {
    fn serialize(&self) -> String {
        /*
        pid: i32,
    cpu: f32,
    ram: u64,
    progress: usize,
         */
        let pid = self.pid.to_string();
        let cpu = self.cpu.to_string();
        let ram = self.ram.to_string();
        let progress = self.progress.to_string();

        let mut res = String::with_capacity(25 + pid.len() + cpu.len() + ram.len() + progress.len());

        res = "?pid=".to_owned() + &pid + "&cpu=" + &cpu + "&ram=" + &ram + "&progress=" + &progress;

        return res;
    }
}
