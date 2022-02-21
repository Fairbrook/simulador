#[derive(Copy, Clone)]
pub enum Operators {
    Suma,
    Resta,
    Mult,
    Div,
    Mod,
    Pow,
}

#[derive(Copy, Clone)]
pub struct Operation {
    pub operator: Operators,
    pub operand_a: f64,
    pub operand_b: f64,
}

impl Operation {
    pub fn to_string(&self) -> String {
        let operator = match self.operator {
            Operators::Suma => " + ",
            Operators::Div => " / ",
            Operators::Mult => " * ",
            Operators::Resta => " - ",
            Operators::Mod => " % ",
            Operators::Pow => " ^ ",
        };
        self.operand_a.to_string() + operator + &self.operand_b.to_string()
    }
    pub fn calc(&self) -> f64 {
        match self.operator {
            Operators::Suma => self.operand_a + self.operand_b,
            Operators::Div => self.operand_a / self.operand_b,
            Operators::Mult => self.operand_a * self.operand_b,
            Operators::Resta => self.operand_a - self.operand_b,
            Operators::Mod => self.operand_a % self.operand_b,
            Operators::Pow => self.operand_a.powf(self.operand_b),
        }
    }
}

#[derive(Clone)]
pub struct Process {
    pub owner: String,
    pub operation: Operation,
    pub et: u32,
    pub pid: String,
}

#[derive(Clone)]
pub struct StatefulProcess {
    pub process: Process,
    pub elapsed: u32,
    pub result: f64,
    pub error: bool,
    pub finished: bool,
}

impl StatefulProcess {
    pub fn resolve(&mut self) {
        self.error = false;
        self.result = self.process.operation.calc();
        self.finished = true;
        self.elapsed = self.process.et;
    }
    pub fn error(&mut self) {
        self.error = true;
        self.finished = true;
        self.elapsed = self.process.et;
    }
    pub fn from(process: Process) -> StatefulProcess {
        StatefulProcess {
            process: process.clone(),
            elapsed: 0,
            error: false,
            result: 0.0,
            finished: false,
        }
    }
}

pub struct Batch {
    process: Vec<StatefulProcess>,
    len: u32,
    et: u32,
    active: usize,
    finished: bool,
}

impl Batch {
    pub fn new() -> Batch {
        Batch {
            process: Vec::new(),
            len: 0,
            et: 0,
            active: 0,
            finished: false,
        }
    }
    pub fn add_process(&mut self, process: Process) -> u32 {
        if self.process.len() >= 4 {
            return 0;
        }
        self.et += process.et;
        self.process.push(StatefulProcess::from(process));
        self.len += 1;
        self.len
    }
    // pub fn get_processes(&self) -> &[StatefulProcess] {
    //     self.process.as_slice()
    // }
    pub fn len(&self) -> u32 {
        self.len
    }
    pub fn estimated(&self) -> u32 {
        self.et
    }
    pub fn get_active(&self) -> &StatefulProcess {
        &self.process[self.active]
    }
    pub fn tick(&mut self) -> bool {
        let mut active = &mut self.process[self.active];
        active.elapsed += 1;
        if active.elapsed >= active.process.et {
            if !active.finished {
                active.resolve();
            }
            if self.len() - 1 > self.active.try_into().unwrap() {
                self.active += 1;
                return true;
            }
            self.finished = true;
        }
        false
    }
    pub fn get_queued(&self, is_active: bool) -> &[StatefulProcess] {
        if !is_active {
            return &self.process[self.active..];
        }
        if self.active + 1 >= self.len().try_into().unwrap() {
            return &[];
        }
        &self.process[self.active + 1..]
    }
    pub fn get_finished(&self) -> &[StatefulProcess] {
        if self.finished {
            return &self.process[..];
        }
        &self.process[..self.active]
    }
    pub fn interrupt(&mut self) -> u32 {
        let active = self.process[self.active].clone();
        let elapsed = active.elapsed;
        self.process.push(active);
        self.process.remove(self.active);
        elapsed
    }

    pub fn error(&mut self) -> u32 {
        let active = &mut self.process[self.active];
        let remaning = active.process.et - active.elapsed;
        active.error();
        if self.len() - 1 > self.active.try_into().unwrap() {
            self.active += 1;
            return remaning;
        }
        self.finished = true;
        remaning
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }
}
