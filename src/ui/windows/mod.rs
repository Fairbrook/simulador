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
    queue_ui: ReadyQueue,

    #[nwg_partial(parent: blocked_frame)]
    blocked_ui: BlockedQueue,

    #[nwg_partial(parent: ready_frame)]
    finished_ui: FinishedList,

    #[nwg_partial(parent: runing_frame)]
    runing_ui: Runing,
}

impl Main {
    fn close(&self) {
        nwg::stop_thread_dispatch();
    }

    fn open_dialog(&self) {
        *self.dialog_data.borrow_mut() = Some(dialogs::DialogNumber::ask_number(
            self.dialog_notice.sender(),
        ));
    }

    fn update(&self, should_update: &[ShouldUpdate]) {
        let state = self.state.borrow();
        self.global_timer.set_text(&seconds_to_str(state.elapsed()));
        self.runing_ui.upate(state.get_active());
        if should_update.contains(&ShouldUpdate::Queue) {
            self.queue_ui.set_list(&Vec::from(state.get_queued()));
        }
        if should_update.contains(&ShouldUpdate::Finished) {
            self.finished_ui.set_list(&Vec::from(state.get_finished()));
        }
        if should_update.contains(&ShouldUpdate::Blocked) {
            self.blocked_ui.set_list(&Vec::from(state.get_blocked()));
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
                }
                self.timer.start();
                let processes = random_processes(result as i32, &mut rng);
                {
                    let mut state = self.state.borrow_mut();
                    state.add_processes(&processes);
                    state.start();
                }
                self.queue_ui.setup();
                self.finished_ui.setup();
                self.blocked_ui.setup();
                self.update(&[ShouldUpdate::Queue]);
            }
            None => {}
        }
    }

    fn on_key_press(&self, key: &nwg::EventData) {
        match key.on_key() {
            nwg::keys::_P => {
                self.timer.stop();
                self.state.borrow_mut().pause();
            }
            nwg::keys::_C => {
                let mut state = self.state.borrow_mut();
                if let States::Paused = state.status() {
                    state.play();
                    self.timer.start();
                }
            }
            nwg::keys::_I => {
                self.state.borrow_mut().interrupt();
                self.update(&[ShouldUpdate::Queue, ShouldUpdate::Blocked]);
            }
            nwg::keys::_E => {
                self.state.borrow_mut().error();
                self.update(&[ShouldUpdate::Queue, ShouldUpdate::Finished]);
            }
            _ => {}
        };
    }
}
