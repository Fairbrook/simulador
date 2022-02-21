mod questionary;
mod types;
mod ui;
use rand::prelude::*;
const PROCESS_PER_BATCH: u32 = 3;

fn process_to_batch(processes: Vec<types::process::Process>) -> Vec<types::process::Batch> {
    let mut res: Vec<types::process::Batch> = Vec::new();
    let mut batch = types::process::Batch::new();
    for p in processes {
        if batch.add_process(p) >= PROCESS_PER_BATCH {
            res.push(batch);
            batch = types::process::Batch::new();
        }
    }
    if batch.len() > 0 {
        res.push(batch);
    }
    res
}

fn random_operator(rng: &mut ThreadRng) -> types::process::Operators {
    let rnd: u16 = rng.gen_range(0..5);
    if rnd == 1 {
        return types::process::Operators::Resta;
    }
    if rnd == 2 {
        return types::process::Operators::Div;
    }
    if rnd == 3 {
        return types::process::Operators::Mult;
    }
    if rnd == 4 {
        return types::process::Operators::Pow;
    }
    if rnd == 5 {
        return types::process::Operators::Mod;
    }
    return types::process::Operators::Suma;
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

fn random_processes(num: i32, rng: &mut ThreadRng) -> Vec<types::process::Process> {
    let mut i = num;
    let mut list = Vec::new();
    while i > 0 {
        list.push(random_process(num - i + 1, rng));
        i -= 1;
    }
    list
}

fn main() {
    let mut rng = thread_rng();
    let num = questionary::start().unwrap();
    if num == 0 {
        println!("Cancelado por el usuario");
        return;
    }
    let list = random_processes(num, &mut rng);
    ui::start(process_to_batch(list)).unwrap();
}
