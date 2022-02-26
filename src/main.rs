mod types;
mod ui;
mod utils;
use rand::prelude::*;
use utils::{process_to_batch, random_processes};
pub const PROCESS_PER_BATCH: u32 = 3;
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

fn main() {
    let mut rng = thread_rng();
    let list = random_processes(10, &mut rng);
    ui::start(process_to_batch(list)).unwrap();
}
