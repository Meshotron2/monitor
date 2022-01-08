// mod communication {
//     pub mod tcp;
// }
//
// mod monitor {
//     pub mod stats;
// }

fn main() {
    monitor::run("127.0.0.1", 6789, "a.out");
}
