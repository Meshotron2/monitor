use sysinfo::{ComponentExt, Process, ProcessExt, SystemExt, System as Sys, ProcessorExt, Pid};

pub struct NodeData {
    cores: usize,
    threads: usize,
    cpu_usage: f32,
    total_ram: u64,
    used_ram: u64,
    temperature: [f32],
}

impl NodeData {
    pub fn new() -> Box<NodeData> {
        let mut s = Sys::new_all();

        let used_ram = s.used_memory();
        let total_ram = s.total_memory();
        let cores = s.physical_core_count().unwrap();
        let threads = s.processors().len();
        let cpu_usage = sys.global_processor_info().cpu_usage();

        let temperature = s.components().iter()
            .filter(|comp| comp.label().starts_with("Core "))
            .map(|comp| comp.temperature())
            .collect();

        return Box::new(NodeData { cores, threads, cpu_usage, total_ram, used_ram, temperature });
    }

    pub fn update(&mut self, sys: &mut Sys) {
        sys.refresh_all();

        self.cpu_usage = sys.global_processor_info().cpu_usage();
        self.used_ram = s.used_memory();

        self.temperature = s.components().iter()
            .filter(|comp| comp.label().starts_with("Core "))
            .map(|comp| comp.temperature())
            .collect();
    }
}

struct ProcData {
    pid: i32,
    cpu: f32,
    ram: u64,
    progress: f32,
}

impl ProcData {
    pub fn new(proc_name: &str) -> Vec<ProcData> {
        let mut s = Sys::new_all();

        return Vec::from_iter(s.processes().iter()
            // .map(|(pid, proc)| proc)
            .filter(|(pid, p)| p.name() == proc_name)
            .map(|(pid, p)| ProcData {
                pid: *pid,
                cpu: p.cpu_usage(),
                ram: p.memory(),
                progress: 0.0,
            })
        );
    }

    pub fn update(&mut self, proc_name: &str, progress: f32, sys: &mut Sys) {
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
                self.progress = 0.0;
            }
        };
    }
}