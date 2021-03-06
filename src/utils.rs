use crate::types;
use rand::prelude::*;

fn random_operator(rng: &mut ThreadRng) -> types::process::Operators {
    let rnd: u16 = rng.gen_range(0..5);
    match rnd {
        1 => types::process::Operators::Resta,
        2 => types::process::Operators::Div,
        3 => types::process::Operators::Mult,
        4 => types::process::Operators::Pow,
        5 => types::process::Operators::Mod,
        _ => types::process::Operators::Suma,
    }
}

fn random_process(index: i32, rng: &mut ThreadRng) -> types::process::Process {
    types::process::Process {
        owner: String::from(""),
        et: rng.gen_range(7..18),
        pid: index.to_string(),
        operation: types::process::Operation {
            operator: random_operator(rng),
            operand_a: (rng.gen_range(1.0..100.0) as f64).round(),
            operand_b: (rng.gen_range(1.0..100.0) as f64).round(),
        },
    }
}

pub fn random_processes(num: i32, rng: &mut ThreadRng) -> Vec<types::process::Process> {
    let mut i = num;
    let mut list = Vec::new();
    while i > 0 {
        list.push(random_process(num - i + 1, rng));
        i -= 1;
    }
    list
}
