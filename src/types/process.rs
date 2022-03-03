pub const BLOCKED_SECONDS: u32 = 10;

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
pub enum State {
    Execution,
    Blocked,
    Error,
    Finished,
    Ready,
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

#[derive(Clone, Default)]
pub struct Times {
    pub start_time: u32,
    pub finished_time: u32,
    pub attendent_seconds: u32,
    pub service_seconds: u32,
    pub waiting_seconds: u32,
    pub blocked_seconds: u32,
}

impl Times {
    pub fn ret(&self) -> u32 {
        self.finished_time - self.start_time
    }
}

#[derive(Clone)]
pub struct StatefulProcess {
    pub process: Process,
    pub result: f64,
    pub state: State,
    pub times: Times,
}

impl StatefulProcess {
    pub fn current_time(&self) -> u32 {
        self.times.start_time
            + self.times.attendent_seconds
            + self.times.service_seconds
            + self.times.blocked_seconds
            + self.times.waiting_seconds
    }
    pub fn resolve(&mut self) {
        self.result = self.process.operation.calc();
        self.state = State::Finished;
        self.times.finished_time = self.current_time();
    }
    pub fn error(&mut self) -> u32 {
        self.state = State::Error;
        self.times.finished_time = self.current_time();
        self.process.et - self.times.service_seconds
    }
    pub fn from(process: Process) -> StatefulProcess {
        StatefulProcess {
            process: process.clone(),
            times: Times {
                ..Default::default()
            },
            result: 0.0,
            state: State::Ready,
        }
    }
    pub fn start(&mut self, timestamp: u32) {
        self.state = State::Execution;
        if self.times.service_seconds == 0{
            self.times.attendent_seconds = timestamp;
        }
    }

    pub fn interrupt(&mut self) {
        self.state = State::Blocked;
    }

    pub fn tick(&mut self) -> &State {
        match self.state {
            State::Blocked => {
                self.times.blocked_seconds += 1;
                if self.times.blocked_seconds % BLOCKED_SECONDS == 0 {
                    self.state = State::Ready;
                }
            }
            State::Execution => {
                self.times.service_seconds += 1;
                if self.times.service_seconds >= self.process.et {
                    self.state = State::Finished;
                    self.resolve();
                }
            }
            State::Ready => {
                self.times.waiting_seconds += 1;
            }
            _ => {}
        };
        &self.state
    }
}
