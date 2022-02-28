use nwd::NwgUi;
use nwg::stretch::{
    geometry::Size,
    style::{Dimension as D, FlexDirection},
};
extern crate native_windows_gui as nwg;
use std::{cell::RefCell, thread};

#[derive(Default, NwgUi)]
pub struct Main {
    dialog_data: RefCell<Option<thread::JoinHandle<String>>>,

    #[nwg_control(size: (300, 115), position: (300, 300), title: "Simulador de procesos", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [Main::close], OnInit:[Main::open_dialog] )]
    window: nwg::Window,
}

impl Main {
    fn close(&self) {
        nwg::stop_thread_dispatch();
    }
    fn open_dialog(&self) {}
}

#[derive(Default, NwgUi)]
pub struct Dialog {
    data: RefCell<Option<String>>,
    #[nwg_control(size: (300, 115), position: (300, 300), title: "Numero de procesos", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [Dialog::close] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, flex_direction: FlexDirection::Row)]
    layout: nwg::FlexboxLayout,

    #[nwg_control]
    #[number_select]
    #[nwg_layout_item(layout: layout,
        // margin: MARGIN,
        max_size: Size { width: D::Points(200.0), height: D::Undefined },
        size: Size { width: D::Percent(0.5), height: D::Auto }
    )]
    input: nwg::NumberSelect,
}

impl Dialog {
    fn close(&self) {
        nwg::stop_thread_dispatch();
    }
    fn ask_number(sender: nwg::NoticeSender) -> thread::JoinHandle<String> {
        thread::spawn(move || {
            let dialog = Dialog::build_ui(Default::default()).expect("Error al crear el díalogo");
            ngw::dispatch_thread_events(); 
            sender.notice();
            dialog.
        })
    }
}
