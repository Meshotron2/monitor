mod communication {
    pub mod tcp;
}

mod monitor {
    pub mod stats;
}

use sysinfo::{ComponentExt, Process, ProcessExt, SystemExt, System as Sys};

fn main() {
    let mut s = Sys::new();
    s.refresh_components_list();
    s.refresh_components();
    // s.refresh_all();
    // s.refresh_processes();

    s.components().iter()
        // .map(|comp| comp.label())
        .filter(|comp| comp.label().starts_with("Core "))
        .for_each(|comp| {
            println!("{}", comp.temperature())
        });
}
