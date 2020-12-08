use std::collections::HashSet;

use itertools::Itertools as _;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

impl Operation {
    fn toggle(&mut self) {
        match self {
            Operation::Acc => unreachable!(),
            Operation::Jmp => *self = Operation::Nop,
            Operation::Nop => *self = Operation::Jmp,
        }
    }

    fn can_toggle(&self) -> bool {
        match self {
            Operation::Acc => false,
            Operation::Jmp | Operation::Nop => true,
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct Instruction {
    operation: Operation,
    argument: i64,
}

struct Interpreter {
    instructions: Box<[Instruction]>,
    accumulator: i64,
    pc: usize,

    executed: HashSet<usize>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum TerminationReason {
    InfiniteLoop,
    Completion,
}

impl Interpreter {
    fn from_input(input: &str) -> Self {
        let instructions: Box<[Instruction]> = input
            .lines()
            .map(|line| {
                let (operation, argument) = line.splitn(2, ' ').collect_tuple().unwrap();
                let operation = match operation {
                    "acc" => Operation::Acc,
                    "jmp" => Operation::Jmp,
                    "nop" => Operation::Nop,
                    _ => unreachable!(),
                };
                let argument = argument.parse().unwrap();
                Instruction {
                    operation,
                    argument,
                }
            })
            .collect();

        Self {
            instructions,
            accumulator: 0,
            pc: 0,
            executed: HashSet::new(),
        }
    }

    // Run the program until either
    // 1) We try to execute an instruction outside of the program, in which case
    //    `TerminationReason::Completion` is returned
    // 2) We hit an infinite loop, in which case `TerminationReason::InfiniteLoop` is returned
    fn run(&mut self) -> TerminationReason {
        while self.executed.insert(self.pc) {
            match self.instructions.get(self.pc) {
                Some(Instruction {
                    operation: Operation::Nop,
                    ..
                }) => self.pc += 1,

                Some(Instruction {
                    operation: Operation::Acc,
                    argument,
                }) => {
                    self.accumulator += argument;
                    self.pc += 1;
                }

                Some(Instruction {
                    operation: Operation::Jmp,
                    argument,
                }) => {
                    self.pc = (self.pc as i64 + argument) as usize;
                }

                None => {
                    debug_assert_eq!(self.pc, self.instructions.len());
                    return TerminationReason::Completion;
                }
            };
        }

        TerminationReason::InfiniteLoop
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.accumulator = 0;
        self.executed.clear();
    }
}

#[inline]
pub fn solve() -> (i64, i64) {
    let mut interpreter = Interpreter::from_input(include_str!("input.txt"));
    interpreter.run();
    let part1 = interpreter.accumulator;

    for i in 0..interpreter.instructions.len() {
        {
            let instruction = &mut interpreter.instructions[i];
            if !instruction.operation.can_toggle() {
                continue;
            }

            instruction.operation.toggle();
        }

        interpreter.reset();
        if interpreter.run() == TerminationReason::Completion {
            return (part1, interpreter.accumulator);
        }

        interpreter.instructions[i].operation.toggle();
    }

    unreachable!();
}
