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