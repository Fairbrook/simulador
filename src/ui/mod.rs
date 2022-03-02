mod windows;
use nwg::NativeUi;
extern crate native_windows_gui as nwg;

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    nwg::init().expect("Fallo la inicialización");
    let _app = windows::Main::build_ui(Default::default()).expect("Fallo al construir el ui");
    nwg::dispatch_thread_events();
    Ok(())
}
