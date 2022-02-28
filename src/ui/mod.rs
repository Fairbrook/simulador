use crate::types::{
    global::{State, States},
    process::Batch,
};
mod windows;
use nwd::NwgUi;
use nwg::NativeUi;
use std::time::{Duration, Instant};
extern crate native_windows_gui as nwg;

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    nwg::init().expect("Fallo la inicialización");
    let _app = windows::Dialog::build_ui(Default::default()).expect("Fallo al construir el ui");
    nwg::dispatch_thread_events();
    let _app = windows::Main::build_ui(Default::default()).expect("Fallo al construir el ui");
    nwg::dispatch_thread_events();
    Ok(())
}
