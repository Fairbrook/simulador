use crate::types::{
    process::{self, StatefulProcess},
    seconds_to_str,
};
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
    #[nwg_control(size: (300, 220), title: "Información del proceso", flags: "WINDOW|VISIBLE")]
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

#[derive(Default, NwgUi)]
pub struct DialogBCP {
    // process: StatefulProcess,
    labels: RefCell<Vec<nwg::Label>>,
    grid: RefCell<nwg::GridLayout>,
    #[nwg_control(size: (300, 220), title: "Numero de procesos", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [DialogBCP::close] )]
    pub window: nwg::Window,

    #[nwg_layout(parent: window)]
    layout: nwg::GridLayout,
}

impl DialogBCP {
    fn close(&self) {
        nwg::stop_thread_dispatch();
    }
    pub fn show_item(processes: Vec<StatefulProcess>) {
        thread::spawn(move || {
            let dialog =
                DialogBCP::build_ui(Default::default()).expect("Error al crear el díalogo");
            dialog.window.set_size(800, 22 * 5 * processes.len() as u32);
            let mut labels = Vec::<nwg::Label>::new();
            nwg::GridLayout::builder()
                .parent(&dialog.window)
                .max_row(Some(5 * processes.len() as u32))
                .build(&dialog.grid.borrow())
                .expect("Error al crear laylout");
            let mut index = 0;
            for process in &processes {
                let mut label: nwg::Label = Default::default();
                let state = String::from("PID: ")
                    + &process.process.pid
                    + &String::from(" | ")
                    + match process.state {
                        process::State::Blocked => "Bloqueado",
                        process::State::Error => "Error",
                        process::State::Finished => "Completado",
                        process::State::Ready => "Listo",
                        process::State::Execution => "Ejecución",
                    };
                nwg::Label::builder()
                    .parent(&dialog.window)
                    .text(&state)
                    .build(&mut label)
                    .expect("Error al crear label");
                (*dialog.grid.borrow()).add_child(0, index, &label);
                index += 1;
                labels.push(label);

                let mut label: nwg::Label = Default::default();
                let mut text =
                    String::from("  Operacion: ") + &process.process.operation.to_string();
                if let process::State::Finished = process.state {
                    text += &(String::from(" = ") + &process.result.to_string());
                }
                nwg::Label::builder()
                    .parent(&dialog.window)
                    .text(&text)
                    .build(&mut label)
                    .expect("Error al crear label");
                (*dialog.grid.borrow()).add_child(0, index, &label);
                labels.push(label);

                let mut label: nwg::Label = Default::default();
                let text = String::from("  Tiempo de llegada: ")
                    + &seconds_to_str(process.times.arrive_time);
                nwg::Label::builder()
                    .parent(&dialog.window)
                    .text(&text)
                    .build(&mut label)
                    .expect("Error al crear label");
                (*dialog.grid.borrow()).add_child(1, index, &label);
                labels.push(label);

                let mut label: nwg::Label = Default::default();
                let text = String::from("  Tiempo de finalizacion: ")
                    + &match process.state {
                        process::State::Error => seconds_to_str(process.times.finished_time),
                        process::State::Finished => seconds_to_str(process.times.finished_time),
                        _ => String::from("N/A"),
                    };
                nwg::Label::builder()
                    .parent(&dialog.window)
                    .text(&text)
                    .build(&mut label)
                    .expect("Error al crear label");
                (*dialog.grid.borrow()).add_child(2, index, &label);
                index += 1;
                labels.push(label);

                let mut label: nwg::Label = Default::default();
                let text = String::from("  Tiempo de retorno: ")
                    + &match process.state {
                        process::State::Error => seconds_to_str(process.times.ret()),
                        process::State::Finished => seconds_to_str(process.times.ret()),
                        _ => String::from("N/A"),
                    };
                nwg::Label::builder()
                    .parent(&dialog.window)
                    .text(&text)
                    .build(&mut label)
                    .expect("Error al crear label");
                (*dialog.grid.borrow()).add_child(0, index, &label);
                labels.push(label);

                let mut label: nwg::Label = Default::default();
                let text = String::from("  Tiempo de espera: ")
                    + &seconds_to_str(process.times.waiting_seconds);
                nwg::Label::builder()
                    .parent(&dialog.window)
                    .text(&text)
                    .build(&mut label)
                    .expect("Error al crear label");
                (*dialog.grid.borrow()).add_child(1, index, &label);
                labels.push(label);

                let mut label: nwg::Label = Default::default();
                let text = String::from("  Tiempo de servicio: ")
                    + &seconds_to_str(process.times.service_seconds);
                nwg::Label::builder()
                    .parent(&dialog.window)
                    .text(&text)
                    .build(&mut label)
                    .expect("Error al crear label");
                (*dialog.grid.borrow()).add_child(2, index, &label);
                index += 1;
                labels.push(label);

                let mut label: nwg::Label = Default::default();
                let text =
                    String::from("  Tiempo restante: ") + &seconds_to_str(process.remaining());
                nwg::Label::builder()
                    .parent(&dialog.window)
                    .text(&text)
                    .build(&mut label)
                    .expect("Error al crear label");
                (*dialog.grid.borrow()).add_child(0, index, &label);
                labels.push(label);

                let mut label: nwg::Label = Default::default();
                let text = String::from("  Tiempo de respuesta: ")
                    + &match process.times.service_seconds {
                        0 => String::from("N/A"),
                        _ => seconds_to_str(process.times.attendent_seconds),
                    };
                nwg::Label::builder()
                    .parent(&dialog.window)
                    .text(&text)
                    .build(&mut label)
                    .expect("Error al crear label");
                (*dialog.grid.borrow()).add_child(1, index, &label);
                labels.push(label);

                if let process::State::Blocked = process.state {
                    let mut label: nwg::Label = Default::default();
                    let time =
                        10 - (process.times.blocked_seconds - (process.times.blocked_seconds / 10));
                    nwg::Label::builder()
                        .parent(&dialog.window)
                        .text(&(String::from(" Restante bloqueado: ") + &seconds_to_str(time)))
                        .build(&mut label)
                        .expect("Error al crear label");
                    (*dialog.grid.borrow()).add_child(2, index, &label);
                    labels.push(label);
                }

                index += 2;
            }
            *dialog.labels.borrow_mut() = labels;
            nwg::dispatch_thread_events();
        });
    }
}
