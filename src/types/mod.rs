pub mod process;

pub fn seconds_to_str(seconds: u32) -> String {
    let minutes = seconds / 60;
    let rest = seconds % 60;
    let rest_str = if rest < 10 {
        String::from("0") + &rest.to_string()
    } else {
        rest.to_string()
    };
    minutes.to_string() + ":" + &rest_str[..]
}

pub struct Batch {
    process: Vec<process::Process>,
    len: u32,
    et: u32,
    active: usize,
    delta: u32,
    finished: bool,
}

impl Batch {
    pub fn new() -> Batch {
        Batch {
            process: Vec::new(),
            len: 0,
            et: 0,
            active: 0,
            delta: 0,
            finished: false,
        }
    }
    pub fn add_process(&mut self, process: process::Process) -> u32 {
        if self.process.len() >= 4 {
            return 0;
        }
        self.et += process.et;
        self.process.push(process);
        self.len += 1;
        self.len
    }
    pub fn get_processes(&self) -> &[process::Process] {
        self.process.as_slice()
    }
    pub fn len(&self) -> u32 {
        self.len
    }
    pub fn estimated(&self) -> u32 {
        self.et
    }
    pub fn get_active(&self) -> &process::Process {
        &self.process[self.active]
    }
    pub fn tick(&mut self) -> bool {
        self.delta += 1;
        let process = self.get_active();
        if self.delta >= process.et {
            if self.len() - 1 > self.active.try_into().unwrap() {
                self.active += 1;
                self.delta = 0;
                return true;
            }
            self.finished = true;
        }
        false
    }
    pub fn get_queued(&self, is_active: bool) -> &[process::Process] {
        if !is_active {
            return &self.process[self.active..];
        }
        if self.active + 1 >= self.len().try_into().unwrap() {
            return &[];
        }
        &self.process[self.active + 1..]
    }
    pub fn get_finished(&self) -> &[process::Process] {
        if self.finished {
            return &self.process[..];
        }
        &self.process[..self.active]
    }
    pub fn delta(&self) -> u32 {
        self.delta
    }
}

pub struct GobalState {
    batches: Vec<Batch>,
    len: u32,
    et: u32,
    elapsed: u32,
    delta: u32,
    active: usize,
    proceses_len: u32,
    finished: bool,
}

impl GobalState {
    pub fn new() -> GobalState {
        GobalState {
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
        self.et += batch.et;
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
        if self.delta >= batch.et {
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
    // pub fn queued_process_len(&self) -> i32 {
    //     let batches_queued = self.get_queued();
    //     let mut res = 0;
    //     for (i, b) in batches_queued.iter().enumerate() {
    //         res += b.get_queued(i == 0).len();
    //     }
    //     res.try_into().unwrap()
    // }
    // pub fn finished_processes(&self) -> types::process::Process {}
    pub fn active_index(&self) -> usize {
        self.active
    }
}
