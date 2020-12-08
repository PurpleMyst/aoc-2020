use std::collections::{HashMap, HashSet};

use itertools::Itertools as _;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Acc => f.pad("acc"),
            Operation::Jmp => f.pad("jmp"),
            Operation::Nop => f.pad("nop"),
        }
    }
}

impl Operation {
    fn toggle(&mut self) {
        match self {
            Operation::Acc => unreachable!(),
            Operation::Jmp => *self = Operation::Nop,
            Operation::Nop => *self = Operation::Jmp,
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Instruction {
    pub operation: Operation,
    pub argument: i64,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:+}", self.operation, self.argument)
    }
}

pub struct Interpreter {
    pub instructions: Box<[Instruction]>,
    pub accumulator: i64,
    pub pc: usize,

    pub executed: HashSet<usize>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum TerminationReason {
    InfiniteLoop,
    Completion,
}

impl Interpreter {
    pub fn from_input(input: &str) -> Self {
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
            executed: HashSet::with_capacity(instructions.len()),
            instructions,
            accumulator: 0,
            pc: 0,
        }
    }

    /// Run the program until either
    /// 1) We try to execute an instruction outside of the program, in which case
    ///    `TerminationReason::Completion` is returned
    /// 2) We hit an infinite loop, in which case `TerminationReason::InfiniteLoop` is returned
    pub fn run_once(&mut self) -> TerminationReason {
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

    /// Starting from the first instruction that would be repeated, iterate, in backwards order,
    /// through the jumps that led there
    pub fn trace_backwards(&mut self) -> impl Iterator<Item = usize> {
        let mut jump_source = HashMap::new();

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
                    let prev_pc = self.pc;
                    self.pc = (self.pc as i64 + argument) as usize;
                    jump_source.insert(self.pc, prev_pc);
                }

                None => {
                    debug_assert_eq!(self.pc, self.instructions.len());
                    unreachable!()
                }
            };
        }

        std::iter::successors(Some(self.pc), move |cur| jump_source.get(cur).copied())
    }

    /// Reset the interpreter to its initial conditions
    pub fn reset(&mut self) {
        self.pc = 0;
        self.accumulator = 0;
        self.executed.clear();
    }
}

#[inline]
pub fn solve() -> (i64, i64) {
    let mut interpreter = Interpreter::from_input(include_str!("input.txt"));

    // Part 1: Just run once and return the accumulator
    interpreter.run_once();
    let part1 = interpreter.accumulator;

    // Part 2: Try to change each of the jumps that leads to the looping into a
    //         NOP, returning the value of the accumulator if we find an instruction
    //         that, when changed, allows the program to run to completion
    interpreter.reset();
    let part2 = interpreter
        .trace_backwards()
        .skip(1)
        .find_map(|ancestor| {
            interpreter.reset();
            interpreter.instructions[ancestor].operation.toggle();

            if interpreter.run_once() == TerminationReason::Completion {
                return Some(interpreter.accumulator);
            }

            interpreter.instructions[ancestor].operation.toggle();
            None
        })
        .unwrap();

    (part1, part2)
}
