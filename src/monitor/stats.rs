use crate::communication::http_requests::RequestSerializable;
use rand::Rng;
use std::collections::HashMap;
use sysinfo::{ComponentExt, ProcessExt, ProcessorExt, System as Sys, SystemExt};

/// Stores usage data relative to the node
///
/// A node is the data of a physical device running one or more instances of the cluster program.
/// In our case, a node is a RaspberryPi
///
/// # Properties
/// -`node_id`: The ID of this NodeData object
/// -`cores`: The number of cores of the node
/// -`threads`: The number of threads of the cluster program running
/// -`cpu_usage`: The percentage of the CPU used in total
/// -`total_ram`: The total RAM available on the node
/// -`used_ram`: The RAM used in the node
/// -`temperature`: The temperature in ºC of each CPU core
#[derive(Debug)]
pub struct NodeData {
    node_id: u8,
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

        let node_id = 0; // rand::thread_rng().gen();
        let used_ram = s.used_memory();
        let total_ram = s.total_memory();
        let cores = s.physical_core_count().unwrap();
        let threads = s.processors().len();
        let cpu_usage = s.global_processor_info().cpu_usage();

        let temperature = s
            .components()
            .iter()
            .filter(|comp| comp.label().starts_with("Core "))
            .map(|comp| comp.temperature())
            .collect();

        //println!("Node created {node_id}");

        NodeData {
            node_id,
            cores,
            threads,
            cpu_usage,
            total_ram,
            used_ram,
            temperature,
        }
    }

    /// Updates volatile data.
    /// - CPU usage
    /// - used RAM
    /// - temperature (ºC)
    ///
    /// # Arguments
    ///
    /// - `sys`: An instance of [Sys] to get the usage data
    pub fn update(&mut self, sys: &mut Sys) {
        sys.refresh_all();

        self.cpu_usage = sys.global_processor_info().cpu_usage();
        self.used_ram = sys.used_memory();

        self.temperature = sys
            .components()
            .iter()
            .filter(|comp| comp.label().starts_with("Core "))
            .map(|comp| comp.temperature())
            .collect();
    }

    pub fn get_id(&self) -> u8 {
        self.node_id
    }

    pub fn set_id(&mut self, id: u8) {
        self.node_id = id;
    }
}

/// Stores data relative to a process.
/// This includes both data regarding the thread itself (pid CPU usage and RAM)
/// and process' metadata (percentage of the task completed and times of each step).
///
/// A process is an instance of the dwm program running on a node.
/// There can be several instances in the same node, ideally one per core minus one.
///
/// # Properties
/// -`node_id`: The id of the node this process belongs to
/// -`pid`: The PID of this process
/// -`cpu`: The CPU usage of this process
/// -`ram`: The RAM consumed by this process
/// -`send_t`: The time it took to send data to the neighbor nodes
/// -`recv_t`: The time it took to receive data from the neighbor nodes
/// -`delay_t`: The time the delay pass took
/// -`scatter_t`: The time the scatter pass took
/// -`progress`: The progress percentage
pub struct ProcData {
    node_id: u8,
    pid: i32,
    cpu: f32,
    ram: u64,
    send_t: f32,
    recv_t: f32,
    delay_t: f32,
    scatter_t: f32,
    progress: f32,
}

impl ProcData {
    /// Generates a new HashMap with all processes with the given name
    ///
    /// # Arguments
    ///
    /// - `proc_name`: The name of the process to analyse
    /// - `node_id`: This node's ID
    /// - `sys`: An instance of [Sys] to get all the system usage data from
    pub fn fetch_all(proc_name: &str, node_id: u8, sys: &mut Sys) -> HashMap<i32, Self> {
        sys.refresh_all();

        let mut map = HashMap::new();

        for (k, v) in sys
            .processes()
            .iter()
            // .map(|(pid, proc)| proc)
            .filter(|(_pid, p)| p.name() == proc_name)
            .map(|(pid, p)| {
                (
                    *pid,
                    Self {
                        node_id,
                        pid: *pid,
                        cpu: p.cpu_usage(),
                        ram: p.memory(),
                        send_t: 0.0,
                        recv_t: 0.0,
                        delay_t: 0.0,
                        scatter_t: 0.0,
                        progress: 0.0,
                    },
                )
            })
        {
            map.insert(k, v);
        }

        return map;
    }

    pub fn new(pid: i32, node_id: u8, sys: &mut Sys) -> Self {
        sys.refresh_all();
        let p1 = sys
            .processes()
            .iter()
            .filter(|(pid1, _pro)| pid == **pid1)
            .map(|(_pid1, pro)| pro)
            .last();

        match p1 {
            Some(p) => Self {
                node_id,
                pid: pid,
                cpu: p.cpu_usage(),
                ram: p.memory(),
                send_t: 0.0,
                recv_t: 0.0,
                delay_t: 0.0,
                scatter_t: 0.0,
                progress: 0.0,
            },
            None => Self {
                node_id: 0,
                pid: 0,
                cpu: 0.0,
                ram: 0,
                send_t: 0.0,
                recv_t: 0.0,
                delay_t: 0.0,
                scatter_t: 0.0,
                progress: 0.0,
            },
        }
    }

    /// Updates the volatile data of the process
    /// - RAM usage
    /// - CPU usage
    /// - progress
    ///
    /// In case this object's PID is not found, all values will be 0.
    ///
    /// # Parameters
    /// -`progress`: The progress until the end of the task
    /// -`send_t`: The time it took to send data to the neighbor nodes
    /// -`recv_t`: The time it took to receive data from the neighbor nodes
    /// -`delay_t`: The time the delay pass took
    /// -`scatter_t`: The time the scatter pass took
    /// -`sys`: An instance of [Sys] to get all the system usage data from
    pub fn update(
        &mut self,
        progress: f32,
        send_t: f32,
        recv_t: f32,
        delay_t: f32,
        scatter_t: f32,
        sys: &mut Sys,
    ) {
        sys.refresh_all();

        let proc_opt = sys.process(self.pid);

        match proc_opt {
            Some(p) => {
                self.ram = p.memory();
                self.cpu = p.cpu_usage();
                self.progress = progress;
                self.send_t = send_t;
                self.recv_t = recv_t;
                self.delay_t = delay_t;
                self.scatter_t = scatter_t;
            }
            None => {
                self.ram = 0;
                self.cpu = 0.0;
                self.progress = 0.0;
                self.send_t = 0.0;
                self.recv_t = 0.0;
                self.delay_t = 0.0;
                self.scatter_t = 0.0;
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
        let node_id = self.node_id.to_string();
        let cores = self.cores.to_string();
        let threads = self.threads.to_string();
        let cpu_usage = self.cpu_usage.to_string();
        let total_ram = self.total_ram.to_string();
        let used_ram = self.used_ram.to_string();
        let mut temperature = String::from("[");

        println!("SENDING: {node_id}, {cpu_usage}, {total_ram}, {used_ram}");

        // for v in &self.temperature {
        //     temperature.push_str(&*v.to_string());
        //     temperature.push_str(", ");
        // }
        let siz = self.temperature.len();
        for i in 0..siz {
            temperature.push_str(&self.temperature[i].to_string());
            if i < siz - 1 {
                temperature.push_str(", ");
            }
        }

        temperature.push(']');

        let res = "{\"nodeId\":".to_owned()
            + &node_id
            + ",\"cores\":"
            + &cores
            + ",\"threads\":"
            + &threads
            + ",\"cpu\":"
            + &cpu_usage
            + ",\"totalRam\":"
            + &total_ram
            + ",\"usedRam\":"
            + &used_ram
            + ",\"temperature\":"
            + &temperature
            + "}";

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
        let node_id = self.node_id.to_string();
        let pid = self.pid.to_string();
        let cpu = self.cpu.to_string();
        let ram = self.ram.to_string();
        let progress = self.progress.to_string();
        let send_t = self.send_t.to_string();
        let recv_t = self.recv_t.to_string();
        let delay_t = self.delay_t.to_string();
        let scatter_t = self.scatter_t.to_string();

        // let mut res =2
        //     String::with_capacity(25 + pid.len() + cpu.len() + ram.len() + progress.len());

        //res = "?pid=".to_owned() + &pid + "&cpu=" + &cpu + "&ram=" + &ram + "&progress=" + &progress;
        let res = "{\"pid\":".to_owned()
            + &pid
            + ",\"nodeId\":"
            + &node_id
            + ",\"cpu\":"
            + &cpu
            + ",\"ram\":"
            + &ram
            + ",\"progress\":"
            + &progress
            + ",\"sendTime\":"
            + &send_t
            + ",\"receiveTime\":"
            + &recv_t
            + ",\"delayTime\":"
            + &delay_t
            + ",\"scatterTime\":"
            + &scatter_t
            + "}";

        return res;
    }
}
