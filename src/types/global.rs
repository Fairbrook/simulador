use crate::types::process::Batch;

pub struct State {
    batches: Vec<Batch>,
    len: u32,
    et: u32,
    elapsed: u32,
    delta: u32,
    active: usize,
    proceses_len: u32,
    finished: bool,
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
            finished: false,
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
    pub fn get_queued(&self) -> &[Batch] {
        if self.finished {
            return &[];
        }
        &self.batches[self.active..]
    }
    pub fn get_finished(&self) -> &[Batch] {
        &self.batches[..self.active]
    }
    fn tick(&mut self) {
        self.delta += 1;
        let batch = self.batches.get_mut(self.active).unwrap();
        batch.tick();
        if self.delta >= batch.estimated() {
            if self.len() - 1 > self.active.try_into().unwrap() {
                self.active += 1;
                self.delta = 0;
                return;
            }
            self.finished = true;
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
}