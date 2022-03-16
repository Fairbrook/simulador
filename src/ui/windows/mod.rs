mod dialogs;
mod partials;
extern crate native_windows_gui as nwg;
use crate::{
    types::{
        global::{ShouldUpdate, State, States},
        seconds_to_str,
    },
    utils::random_processes,
};
use nwd::NwgUi;
use partials::{BlockedQueue, FinishedList, ReadyQueue, Runing};
use rand::prelude::*;
use std::{cell::RefCell, thread};

#[derive(Default, NwgUi)]
pub struct Main {
    dialog_data: RefCell<Option<thread::JoinHandle<u32>>>,
    state: RefCell<State>,
    rng: RefCell<ThreadRng>,

    #[nwg_control(size: (1500, 700), title: "Simulador de procesos", flags: "WINDOW|VISIBLE|RESIZABLE")]
    #[nwg_events( OnWindowClose: [Main::close], OnInit:[Main::open_dialog], OnKeyPress:[Main::on_key_press(SELF, EVT_DATA)] )]
    window: nwg::Window,

    #[nwg_control(interval:1000, stopped:true)]
    #[nwg_events(OnTimerTick:[Main::tick])]
    #[allow(deprecated)]
    timer: nwg::Timer,

    #[nwg_control]
    #[nwg_events(OnNotice:[Main::read_dialog_output])]
    dialog_notice: nwg::Notice,

    #[nwg_layout(parent:window, spacing: 2)]
    layout: nwg::GridLayout,

    #[nwg_control(text:"Transcurrido: ", h_align:nwg::HTextAlign::Right)]
    #[nwg_layout_item(layout:layout,row:10,col:0)]
    timer_label: nwg::Label,

    #[nwg_control(text:"00:00", h_align:nwg::HTextAlign::Left)]
    #[nwg_layout_item(layout:layout,row:10,col:1)]
    global_timer: nwg::Label,

    #[nwg_control(text:"Estimado: ", h_align:nwg::HTextAlign::Right)]
    #[nwg_layout_item(layout:layout,row:10,col:2)]
    et_label: nwg::Label,

    #[nwg_control(text:"00:00", h_align:nwg::HTextAlign::Left)]
    #[nwg_layout_item(layout:layout,row:10,col:3)]
    et_timer: nwg::Label,

    #[nwg_control(text:"", h_align:nwg::HTextAlign::Right)]
    #[nwg_layout_item(layout:layout,row:10,col:4)]
    state_label: nwg::Label,

    #[nwg_control(flags: "VISIBLE")]
    #[nwg_layout_item(layout: layout, row:0,  col: 0, col_span:3, row_span: 10)]
    queue_frame: nwg::Frame,

    #[nwg_control]
    #[nwg_layout_item(layout: layout, row:0,  col: 3, col_span:3, row_span: 10)]
    runing_frame: nwg::Frame,

    #[nwg_control(flags: "VISIBLE")]
    #[nwg_layout_item(layout: layout, row:0,  col: 6, col_span:3, row_span: 10)]
    blocked_frame: nwg::Frame,

    #[nwg_control(flags: "VISIBLE")]
    #[nwg_layout_item(layout: layout, row:0,  col: 9, col_span:3, row_span: 10)]
    ready_frame: nwg::Frame,

    #[nwg_partial(parent: queue_frame)]
    #[nwg_events((data_view, OnKeyPress):[Main::on_key_press(SELF, EVT_DATA)] )]
    queue_ui: ReadyQueue,

    #[nwg_partial(parent: blocked_frame)]
    #[nwg_events((data_view, OnKeyPress):[Main::on_key_press(SELF, EVT_DATA)] )]
    blocked_ui: BlockedQueue,

    #[nwg_partial(parent: ready_frame)]
    #[nwg_events((data_view, OnKeyPress):[Main::on_key_press(SELF, EVT_DATA)], (data_view,  MousePressLeftUp):[Main::on_select_item(SELF)] )]
    finished_ui: FinishedList,

    #[nwg_partial(parent: runing_frame)]
    runing_ui: Runing,
}

impl Main {
    fn close(&self) {
        nwg::stop_thread_dispatch();
    }

    fn open_dialog(&self) {
        *self.rng.borrow_mut() = thread_rng();
        self.queue_ui.setup();
        self.finished_ui.setup();
        self.blocked_ui.setup();
        self.timer.start();
        // *self.dialog_data.borrow_mut() = Some(dialogs::DialogNumber::ask_number(
        //     self.dialog_notice.sender(),
        // ));
    }

    fn update_state_label(&self) {
        let state_text: &str = match self.state.borrow().status() {
            States::Finished => "Terminado",
            States::Paused => "Pausado",
            States::Processing => "Procesando",
        };
        self.state_label.set_text(state_text);
    }

    fn update(&self, should_update: &[ShouldUpdate]) {
        let state = self.state.borrow();
        self.global_timer.set_text(&seconds_to_str(state.elapsed()));
        self.runing_ui.upate(state.get_active());
        if should_update.contains(&ShouldUpdate::Queue) {
            self.queue_ui.set_list(&Vec::from(state.get_queued()));
        }
        if should_update.contains(&ShouldUpdate::Finished) {
            self.update_state_label();
            self.finished_ui.set_list(&Vec::from(state.get_finished()));
        }
        if should_update.contains(&ShouldUpdate::Blocked) {
            self.blocked_ui.set_list(&Vec::from(state.get_blocked()));
        }
        if let States::Finished = state.status() {
            dialogs::DialogBCP::show_item(self.state.borrow().get_all());
            self.timer.stop();
        }
    }

    fn tick(&self) {
        let dirty = self.state.borrow_mut().tick();
        self.update(&dirty[..]);
    }

    fn read_dialog_output(&self) {
        let mut rng = thread_rng();
        let data = self.dialog_data.borrow_mut().take();
        match data {
            Some(handle) => {
                let result = handle.join().unwrap();
                if result == 0 {
                    self.close();
                    return;
                }
                self.timer.start();
                let processes = random_processes(result as i32, &mut rng);
                {
                    let mut state = self.state.borrow_mut();
                    state.add_processes(&processes);
                    state.start();
                    self.et_timer
                        .set_text(&seconds_to_str(state.estimated())[..]);
                }
                self.queue_ui.setup();
                self.finished_ui.setup();
                self.blocked_ui.setup();
                self.update(&[ShouldUpdate::Queue]);
                self.update_state_label();
            }
            None => {}
        }
    }

    fn on_select_item(&self) {
        let index = match self.finished_ui.data_view.selected_item() {
            Some(val) => val as i32,
            None => -1,
        };
        if index < 0 {
            return;
        }
        let finished = Vec::from(self.state.borrow().get_finished());
        let proc = finished[index as usize].clone();
        dialogs::DialogDetails::show_item(proc);
    }

    fn on_key_press(&self, key: &nwg::EventData) {
        match key.on_key() {
            nwg::keys::_P => {
                self.timer.stop();
                self.state.borrow_mut().pause();
                self.update_state_label();
            }
            nwg::keys::_C => {
                {
                    let mut state = self.state.borrow_mut();
                    if let States::Paused = state.status() {
                        state.play();
                        self.timer.start();
                    }
                }
                self.update_state_label();
            }
            nwg::keys::_I => {
                self.state.borrow_mut().interrupt();
                self.update(&[ShouldUpdate::Queue, ShouldUpdate::Blocked]);
            }
            nwg::keys::_E => {
                self.state.borrow_mut().error();
                self.update(&[ShouldUpdate::Queue, ShouldUpdate::Finished]);
            }
            nwg::keys::_N => {
                let processes = random_processes(1, &mut self.rng.borrow_mut());
                {
                    let mut state = self.state.borrow_mut();
                    state.add_processes(&processes);
                    state.start();
                    self.et_timer
                        .set_text(&seconds_to_str(state.estimated())[..]);
                }
                self.update(&[ShouldUpdate::Queue]);
                self.update_state_label();
            }
            nwg::keys::_B => {
                dialogs::DialogBCP::show_item(self.state.borrow().get_all());
                self.timer.stop();
                self.state.borrow_mut().pause();
                self.update_state_label();
            }
            _ => {}
        };
    }
}
