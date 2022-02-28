use nwd::NwgUi;
extern crate native_windows_gui as nwg;

#[derive(Default, NwgUi)]
pub struct Main{
    #[nwg_control(size: (300, 115), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [Main::close] )]
    window: nwg::Window,
}

impl Main{
    fn close(&self){
        nwg::stop_thread_dispatch();
    }
}

#[derive(Default, NwgUi)]
pub struct Dialog{
    #[nwg_control(size: (300, 115), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [Dialog::close] )]
    window: nwg::Window,
}

impl Dialog{
    fn close(&self){
        nwg::stop_thread_dispatch();
    }
}