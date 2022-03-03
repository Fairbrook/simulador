use crate::types::{process::StatefulProcess, seconds_to_str};
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

#[derive(Default, NwgUi)]
pub struct DialogDetails {
    // process: StatefulProcess,
    #[nwg_control(size: (300, 220), title: "Numero de procesos", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [DialogDetails::close] )]
    window: nwg::Window,

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,

    #[nwg_control(text: "PID: ")]
    #[nwg_layout_item(layout: layout, row:0, col:0)]
    pid_label: nwg::Label,

    #[nwg_control(text: "Tiempo de Llegada:")]
    #[nwg_layout_item(layout: layout, row:2, col:0, col_span:2)]
    start_label: nwg::Label,
    #[nwg_control(text: "00:00")]
    #[nwg_layout_item(layout: layout, row:2, col:2)]
    start_time: nwg::Label,

    #[nwg_control(text: "Tiempo de Finalizacion:")]
    #[nwg_layout_item(layout: layout, row:3, col:0, col_span:2)]
    finished_label: nwg::Label,
    #[nwg_control(text: "00:00")]
    #[nwg_layout_item(layout: layout, row:3, col:2)]
    finished_time: nwg::Label,

    #[nwg_control(text: "Tiempo de Retorno:")]
    #[nwg_layout_item(layout: layout, row:4, col:0, col_span:2)]
    return_label: nwg::Label,
    #[nwg_control(text: "00:00")]
    #[nwg_layout_item(layout: layout, row:4, col:2)]
    return_time: nwg::Label,

    #[nwg_control(text: "Tiempo de Respuesta:")]
    #[nwg_layout_item(layout: layout, row:5, col:0, col_span:2)]
    response_label: nwg::Label,
    #[nwg_control(text: "00:00")]
    #[nwg_layout_item(layout: layout, row:5, col:2)]
    response_time: nwg::Label,

    #[nwg_control(text: "Tiempo de Espera:")]
    #[nwg_layout_item(layout: layout, row:6, col:0, col_span:2)]
    wait_label: nwg::Label,
    #[nwg_control(text: "00:00")]
    #[nwg_layout_item(layout: layout, row:6, col:2)]
    wait_time: nwg::Label,

    #[nwg_control(text: "Tiempo de Servicio:")]
    #[nwg_layout_item(layout: layout, row:7, col:0, col_span:2)]
    service_label: nwg::Label,
    #[nwg_control(text: "00:00")]
    #[nwg_layout_item(layout: layout, row:7, col:2)]
    service_time: nwg::Label,
}

impl DialogDetails {
    fn close(&self) {
        nwg::stop_thread_dispatch();
    }
    pub fn show_item(process: StatefulProcess) {
        thread::spawn(move || {
            let _dialog =
                DialogDetails::build_ui(Default::default()).expect("Error al crear el díalogo");
            _dialog
                .pid_label
                .set_text(&format!("PID:{}", process.process.pid));
            _dialog
                .start_time
                .set_text(&seconds_to_str(process.times.start_time)[..]);
            _dialog
                .finished_time
                .set_text(&seconds_to_str(process.times.finished_time)[..]);
            _dialog
                .return_time
                .set_text(&seconds_to_str(process.times.ret())[..]);
            _dialog
                .response_time
                .set_text(&seconds_to_str(process.times.attendent_seconds)[..]);
            _dialog
                .wait_time
                .set_text(&seconds_to_str(process.times.waiting_seconds)[..]);
            _dialog
                .service_time
                .set_text(&seconds_to_str(process.times.service_seconds)[..]);
            nwg::dispatch_thread_events();
        });
    }
}
