use nwd::NwgUi;
use nwg::NativeUi;

use std::{cell::RefCell, thread};

#[derive(Default, NwgUi)]
pub struct DialogNumber {
    data: RefCell<Option<u32>>,
    #[nwg_control(size: (220, 120), title: "Numero de procesos", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [DialogNumber::close] )]
    window: nwg::Window,

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,

    #[nwg_control(text: "Número de procesos:")]
    #[nwg_layout_item(layout: layout, row:0, col:0, col_span:4)]
    label: nwg::Label,

    #[nwg_control(focus:true)]
    #[nwg_layout_item(layout: layout,
        row: 1,col:0, col_span:4, row_span:1
    )]
    input: nwg::TextInput,

    #[nwg_control(text: "Aceptar")]
    #[nwg_layout_item(layout: layout,
        row:2, col:2, col_span:2, row_span:1
    )]
    #[nwg_events(OnButtonClick: [DialogNumber::enter])]
    accept_button: nwg::Button,
}

impl DialogNumber {
    fn close(&self) {
        nwg::stop_thread_dispatch();
    }
    fn enter(&self) {
        let mut data = self.data.borrow_mut();
        let input_value = self.input.text();
        let input_value = input_value.parse::<u32>();
        match input_value {
            Ok(value) => {
                *data = Some(value);
                self.close();
            }
            Err(_) => {}
        }
    }
    pub fn ask_number(sender: nwg::NoticeSender) -> thread::JoinHandle<u32> {
        thread::spawn(move || {
            let dialog =
                DialogNumber::build_ui(Default::default()).expect("Error al crear el díalogo");
            nwg::dispatch_thread_events();
            sender.notice();
            dialog.data.take().unwrap_or(0)
        })
    }
}
