mod questionary;
mod types;
mod ui;
// use questionary;

fn process_to_batch(processes: Vec<types::process::Process>) -> Vec<types::Batch> {
    let mut res: Vec<types::Batch> = Vec::new();
    let mut batch = types::Batch::new();
    for p in processes {
        if batch.add_process(p) >= 4 {
            res.push(batch);
            batch = types::Batch::new();
        }
    }
    res.push(batch);
    res
}

fn main() {
    let list = questionary::start().unwrap();
    // let list: Vec<types::process::Process> = vec![
    //     types::process::Process {
    //         owner: String::from("Kevin"),
    //         operation: types::process::Operation {
    //             operator: types::process::Operators::Div,
    //             operand_a: 1.0,
    //             operand_b: 2.0,
    //         },
    //         et: 10,
    //         pid: String::from("proc 1"),
    //     },
    //     types::process::Process {
    //         owner: String::from("Kevin"),
    //         operation: types::process::Operation {
    //             operator: types::process::Operators::Div,
    //             operand_a: 1.0,
    //             operand_b: 2.0,
    //         },
    //         et: 10,
    //         pid: String::from("proc 2"),
    //     },
    //     types::process::Process {
    //         owner: String::from("Kevin"),
    //         operation: types::process::Operation {
    //             operator: types::process::Operators::Div,
    //             operand_a: 1.0,
    //             operand_b: 2.0,
    //         },
    //         et: 10,
    //         pid: String::from("proc 3"),
    //     },
    //     types::process::Process {
    //         owner: String::from("Kevin"),
    //         operation: types::process::Operation {
    //             operator: types::process::Operators::Div,
    //             operand_a: 1.0,
    //             operand_b: 2.0,
    //         },
    //         et: 10,
    //         pid: String::from("proc 4"),
    //     },
    //     types::process::Process {
    //         owner: String::from("Kevin"),
    //         operation: types::process::Operation {
    //             operator: types::process::Operators::Div,
    //             operand_a: 1.0,
    //             operand_b: 2.0,
    //         },
    //         et: 10,
    //         pid: String::from("proc 5"),
    //     },
    //     types::process::Process {
    //         owner: String::from("Kevin"),
    //         operation: types::process::Operation {
    //             operator: types::process::Operators::Div,
    //             operand_a: 1.0,
    //             operand_b: 2.0,
    //         },
    //         et: 10,
    //         pid: String::from("proc 6"),
    //     },
    // ];
    if list.len()==0{
        println!("Cancelado por el usuario");
        return;
    }
    ui::start(process_to_batch(list)).unwrap();
    // for process in list {
    //     println!("{}", process.pid)
    // }
}
