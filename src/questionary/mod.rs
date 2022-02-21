// use crate::types::process;
pub mod questions;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io;

// pub fn process_q(process_list: &[process::Process]) -> process::Process {
//     let pids: Vec<&str> = process_list.iter().map(|p| &p.pid[..]).collect();
//     let name = questions::text("Nombre del programador:");
//     let et = questions::numeric("Tiempo estimado de ejecución (seg):");
//     let pid = questions::unique("Identificador del programa:", pids.as_slice());
//     let operator = questions::operator();
//     let operand_a = questions::float("Operador a:");
//     let operand_b = questions::float("Operador b:");
//     process::Process {
//         owner: name,
//         et: u32::try_from(et).unwrap(),
//         pid: pid,
//         operation: process::Operation {
//             operator: operator,
//             operand_a: operand_a,
//             operand_b: operand_b,
//         },
//     }
// }

pub fn start() -> Result<i32, io::Error> {
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Green),
        Print("Simulador de procesos"),
        Print("\n"),
        ResetColor,
    )?;
    let proc_num = questions::numeric("Numero de procesos: ");
    // let mut i = 0;
    // let mut process_list: Vec<process::Process> = Vec::new();
    // while i < proc_num {
    //     execute!(
    //         io::stdout(),
    //         SetForegroundColor(Color::Green),
    //         Print(String::from("\n\nProceso #")),
    //         Print(i.to_string()),
    //         Print("\n"),
    //         ResetColor,
    //     )?;
    //     process_list.push(process_q(process_list.as_slice()));
    //     i += 1;
    // }
    let cont = questions::confirm();
    if !cont {
        return Ok(0);
    }
    Ok(proc_num)
}
