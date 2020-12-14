use std::collections::HashMap;

const MASK_HEADER: &str = "mask = ";
const SET_HEADER: &str = "mem[";

const INT_SIZE: usize = 36;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Instruction {
    Mask { mask: u64, metamask: u64 },
    Set { index: usize, value: u64 },
}

pub fn solve_part1(instructions: &[Instruction]) -> u64 {
    let mut mem = [0u64; 1 << 16];

    let mut mask = 0;
    let mut metamask = 0;
    for &instruction in instructions {
        match instruction {
            Instruction::Mask {
                mask: next_mask,
                metamask: next_metamask,
            } => {
                mask = next_mask;
                metamask = next_metamask;
            }

            Instruction::Set { index, value } => {
                mem[index] = (value & !metamask) | mask;
            }
        }
    }

    mem.iter().sum()
}

fn set(memory: &mut HashMap<u64, u64>, metamask: u64, address: u64, value: u64, i: usize, v: u64) {
    // println!(
    //     "{} \x1b[{}m{:0width$b}\x1b[0m",
    //     " ".repeat((0..i).filter(|j| metamask & (1 << j) == 0).count()),
    //     31 + (i % 10),
    //     v,
    //     width = i,
    // );

    if i == INT_SIZE {
        memory.insert(v, value);
        return;
    }

    if metamask & (1 << i) != 0 {
        // bit is set
        set(
            memory,
            metamask,
            address,
            value,
            i + 1,
            v | (address & (1 << i)),
        )
    } else {
        // bit is floaty
        set(memory, metamask, address, value, i + 1, v);
        set(memory, metamask, address, value, i + 1, v | (1 << i));
    }
}

pub fn solve_part2(instructions: &[Instruction]) -> u64 {
    let mut memory = HashMap::with_capacity(1 << 16);

    let mut mask = 0;
    let mut metamask = 0;
    for &instruction in instructions {
        match instruction {
            Instruction::Mask {
                mask: next_mask,
                metamask: next_metamask,
            } => {
                mask = next_mask;
                metamask = next_metamask;
            }

            Instruction::Set { index, value } => {
                let index = index as u64 | mask;

                set(&mut memory, metamask, index, value, 0, 0);
            }
        }
    }

    memory.values().sum()
}

pub fn parse_input() -> Vec<Instruction> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            if line.starts_with(MASK_HEADER) {
                let mut mask = 0;
                let mut metamask = 0;

                line.bytes()
                    .skip(MASK_HEADER.len())
                    .enumerate()
                    .filter(|&(_, b)| b != b'X')
                    .for_each(|(i, b)| {
                        let i = INT_SIZE - (i + 1);
                        mask |= u64::from(b - b'0') << i;
                        metamask |= 1 << i;
                    });

                Instruction::Mask { mask, metamask }
            } else {
                let mut parts = line[SET_HEADER.len()..].splitn(2, "] = ");
                let index = parts.next().unwrap().parse::<usize>().unwrap();
                let value = parts.next().unwrap().parse::<u64>().unwrap();
                Instruction::Set { index, value }
            }
        })
        .collect()
}

#[inline]
pub fn solve() -> (u64, u64) {
    let instructions = parse_input();
    (solve_part1(&instructions), solve_part2(&instructions))
}
