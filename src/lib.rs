mod communication {
    pub mod tcp;
}

mod monitor {
    pub mod stats;
}

fn load_node() -> monitor::stats::NodeData {
    monitor::stats::fetch_node_data()
}