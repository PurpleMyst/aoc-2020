//! Draw graph of program execution

use std::collections::HashSet;

use day08::{Interpreter, Operation};

fn main() {
    let interpreter = Interpreter::from_input(include_str!("../input.txt"));

    println!("digraph {{");

    let mut pc = 0usize;
    let mut seen = HashSet::new();

    while seen.insert(pc) {
        let instr = interpreter.instructions[pc];

        if pc == 0 {
            println!("START [shape=box color=purple]");
            println!(
                "START -> \"{} {:+}\" [color=purple]",
                instr.operation, instr.argument
            );
        }

        print!("\"{} {:+}\" -> ", instr.operation, instr.argument);

        match instr.operation {
            Operation::Acc => pc += 1,
            Operation::Jmp => pc = (pc as i64 + instr.argument) as usize,
            Operation::Nop => pc += 1,
        }

        if pc >= interpreter.instructions.len() {
            println!("END [color=green]");
            println!("{{ rank=sink; END [shape=box color=green] }}");
            break;
        }

        let instr2 = interpreter.instructions[pc];

        print!("\"{} {:+}\"", instr2.operation, instr2.argument);

        if seen.contains(&pc) {
            print!(" [color = red]");
            println!();
            println!(
                "\"{} {:+}\" [shape=box color=red]",
                instr.operation, instr.argument
            );
            println!(
                "\"{} {:+}\" [shape=box color=red]",
                instr2.operation, instr2.argument
            );
        }

        println!();
    }

    println!("}}");
}
