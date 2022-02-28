mod types;
mod ui;
mod utils;
use utils::{process_to_batch};
pub const PROCESS_PER_BATCH: u32 = 3;
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

fn main() {
    ui::start().unwrap();
}
