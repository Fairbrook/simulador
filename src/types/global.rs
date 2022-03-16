use crate::types::process::{self, Process, StatefulProcess, BLOCKED_SECONDS};

#[derive(PartialEq)]
pub enum States {
    Paused,
    Finished,
    Processing,
}

#[derive(PartialEq)]
pub enum ShouldUpdate {
    Queue,
    Blocked,
    Finished,
}

pub struct State {
    processes: Vec<StatefulProcess>,
    blocked: Vec<StatefulProcess>,
    len: u32,
    et: u32,
    elapsed: u32,
    active: usize,
    status: States,
}

impl Default for State {
    fn default() -> Self {
        State {
            processes: vec![],
            blocked: vec![],
            len: 0,
            et: 0,
            elapsed: 0,
            active: 0,
            status: States::Processing,
        }
    }
}

impl State {
    pub fn add_process(&mut self, process: Process) -> u32 {
        self.et += process.et;
        let mut stateful = StatefulProcess::from(process);
        stateful.times.arrive_time = self.elapsed;
        self.processes.push(stateful);
        self.len += 1;
        self.len
    }
    pub fn add_processes(&mut self, process: &Vec<Process>) {
        for proc in process {
            let mut proc = proc.clone();
            proc.pid = String::from(self.len.to_string());
            self.add_process(proc);
        }
    }
    pub fn start(&mut self) {
        self.processes[self.active].start(self.elapsed);
    }
    pub fn estimated(&self) -> u32 {
        self.et
    }
    fn next_proc(&mut self) {
        self.active += 1;
        if self.active >= (self.processes.len() as usize) {
            if self.blocked.len() > 0 {
                return;
            }
            self.status = States::Finished;
            return;
        }
        self.processes[self.active].start(self.elapsed);
    }
    pub fn tick(&mut self) -> Vec<ShouldUpdate> {
        let mut updates = Vec::<ShouldUpdate>::new();
        let mut last_index: i32 = -1;
        if self.status == States::Finished {
            return updates;
        }
        self.elapsed += 1;
        for proc in &mut self.processes[self.active..].iter_mut() {
            if let process::State::Finished = proc.tick() {
                updates.push(ShouldUpdate::Queue);
                updates.push(ShouldUpdate::Finished);
            }
        }
        if updates.contains(&ShouldUpdate::Queue) {
            self.next_proc();
        }
        for (i, proc) in &mut self.blocked.iter_mut().enumerate() {
            if let process::State::Ready = proc.tick() {
                updates.push(ShouldUpdate::Queue);
                last_index = i as i32;
                self.processes.push(proc.clone());
                self.processes[self.active].start(self.elapsed);
            }
        }
        if last_index > -1 {
            if (last_index + 1) as usize >= self.blocked.len() {
                self.blocked = Vec::new();
                updates.push(ShouldUpdate::Blocked);
                return updates;
            }
            self.blocked = Vec::from(&self.blocked[(last_index + 1) as usize..]);
        }
        if self.blocked.len() > 0 {
            updates.push(ShouldUpdate::Blocked);
        }
        updates
    }
    pub fn get_active(&self) -> Option<StatefulProcess> {
        if self.processes.len() <= self.active {
            return None;
        }
        Some(self.processes[self.active].clone())
    }
    pub fn get_queued(&self) -> &[StatefulProcess] {
        if let States::Finished = self.status {
            return &[];
        }
        if self.processes.len() > self.active + 1 {
            return &self.processes[self.active + 1..];
        }
        return &[];
    }
    pub fn get_all(&self) -> Vec<StatefulProcess> {
        [self.processes.clone(), self.blocked.clone()].concat()
    }
    pub fn get_blocked(&self) -> &[StatefulProcess] {
        &self.blocked[..]
    }
    pub fn get_finished(&self) -> &[StatefulProcess] {
        &self.processes[..self.active]
    }
    pub fn elapsed(&self) -> u32 {
        self.elapsed
    }
    pub fn interrupt(&mut self) {
        let active = &mut self.processes[self.active];
        active.interrupt();
        self.blocked.push(active.clone());
        self.processes.remove(self.active);
        self.et += BLOCKED_SECONDS;
        self.processes[self.active].start(self.elapsed);
    }
    pub fn error(&mut self) {
        if States::Finished == self.status {
            return;
        }
        self.et -= self.processes[self.active].error();
        self.next_proc();
    }
    pub fn pause(&mut self) {
        self.status = States::Paused;
    }
    pub fn play(&mut self) {
        self.status = States::Processing;
    }
    pub fn status(&self) -> &States {
        &self.status
    }
}
