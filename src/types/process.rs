pub enum Operators {
    Suma,
    Resta,
    Mult,
    Div,
    Mod,
    Pow,
}

pub struct Operation {
    pub operator: Operators,
    pub operand_a: f64,
    pub operand_b: f64,
}

impl Operation {
    pub fn to_string(&self) -> String {
        let operator = match self.operator {
            Operators::Suma => "+",
            Operators::Div => "/",
            Operators::Mult => "*",
            Operators::Resta => "-",
            Operators::Mod => "%",
            Operators::Pow => "^",
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

pub struct Process {
    pub owner: String,
    pub operation: Operation,
    pub et: u32,
    pub pid: String,
}

pub struct Batch {
    process: Vec<Process>,
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
    pub fn add_process(&mut self, process: Process) -> u32 {
        if self.process.len() >= 4 {
            return 0;
        }
        self.et += process.et;
        self.process.push(process);
        self.len += 1;
        self.len
    }
    pub fn get_processes(&self) -> &[Process] {
        self.process.as_slice()
    }
    pub fn len(&self) -> u32 {
        self.len
    }
    pub fn estimated(&self) -> u32 {
        self.et
    }
    pub fn get_active(&self) -> &Process {
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
    pub fn get_queued(&self, is_active: bool) -> &[Process] {
        if !is_active {
            return &self.process[self.active..];
        }
        if self.active + 1 >= self.len().try_into().unwrap() {
            return &[];
        }
        &self.process[self.active + 1..]
    }
    pub fn get_finished(&self) -> &[Process] {
        if self.finished {
            return &self.process[..];
        }
        &self.process[..self.active]
    }
    pub fn delta(&self) -> u32 {
        self.delta
    }
}
