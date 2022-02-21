use crate::types::process::Batch;

#[derive(PartialEq)]
pub enum States {
    Paused,
    Finished,
    Processing,
}

pub struct State {
    batches: Vec<Batch>,
    len: u32,
    et: u32,
    elapsed: u32,
    delta: u32,
    active: usize,
    proceses_len: u32,
    status: States,
}

impl State {
    pub fn new() -> State {
        State {
            batches: Vec::new(),
            len: 0,
            et: 0,
            elapsed: 0,
            delta: 0,
            active: 0,
            proceses_len: 0,
            status: States::Processing,
        }
    }
    pub fn add_batch(&mut self, batch: Batch) -> u32 {
        if self.batches.len() >= 4 {
            return 0;
        }
        self.et += batch.estimated();
        self.proceses_len += batch.len();
        self.batches.push(batch);
        self.len += 1;
        self.len
    }
    pub fn len(&self) -> u32 {
        self.len
    }
    pub fn estimated(&self) -> u32 {
        self.et
    }
    pub fn add_seg(&mut self) {
        self.elapsed += 1;
        self.tick()
    }
    pub fn get_active(&self) -> &Batch {
        &self.batches[self.active]
    }
    // pub fn get_queued(&self) -> &[Batch] {
    //     if let States::Finished = self.status {
    //         return &[];
    //     }
    //     &self.batches[self.active..]
    // }
    // pub fn get_finished(&self) -> &[Batch] {
    //     &self.batches[..self.active]
    // }
    pub fn get_batches(&self) -> &[Batch] {
        &self.batches[..]
    }
    fn next_batch(&mut self) {
        if self.len() - 1 > self.active.try_into().unwrap() {
            self.active += 1;
            self.delta = 0;
            return;
        }
        self.status = States::Finished;
    }
    fn tick(&mut self) {
        self.delta += 1;
        let batch = self.batches.get_mut(self.active).unwrap();
        batch.tick();
        if batch.is_finished() {
            self.next_batch();
        }
    }
    pub fn get_processes_len(&self) -> u32 {
        self.proceses_len
    }
    pub fn elapsed(&self) -> u32 {
        self.elapsed
    }
    pub fn active_index(&self) -> usize {
        self.active
    }
    pub fn interrupt(&mut self) {
        self.batches[self.active].interrupt();
    }
    pub fn error(&mut self) {
        if States::Finished == self.status {
            return;
        }
        self.et -= self.batches[self.active].error();
        if self.batches[self.active].is_finished() {
            self.next_batch();
        }
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
