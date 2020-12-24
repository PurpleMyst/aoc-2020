#![feature(new_uninit)]

use std::fmt::Display;

/// How many cups do we pick up?
const PICKUP_NUM: usize = 3;

const MOVES_PART1: usize = 100;

const CARDS_PART1: usize = 10;
const CARDS_PART2: usize = 1_000_001;

const MOVES_PART2: usize = 10_000_000;

pub struct CupsDisplay {
    successors: [u8; CARDS_PART1],
}

impl Display for CupsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut n = 1;
        loop {
            n = self.successors[n as usize];
            if n == 1 {
                break;
            }
            write!(f, "{}", n)?;
        }

        Ok(())
    }
}

fn play_round_part1(current_cup: u8, successors: &mut [u8; CARDS_PART1]) -> u8 {
    // Pick up the cups after the current one
    let mut hand = [0; PICKUP_NUM];
    for dest in hand.iter_mut() {
        let picked_up = successors[current_cup as usize];
        let previous_successor = successors[picked_up as usize];
        successors[current_cup as usize] = previous_successor;
        *dest = picked_up;
    }

    // Find the destination cup
    let dest = (1..=3)
        .take_while(|&n| n < current_cup)
        .map(|n| current_cup - n)
        .chain(
            (current_cup.checked_sub(4).unwrap_or(1)..successors.len() as u8)
                .rev()
                .filter(|&n| n != current_cup),
        )
        .find(|&n| !hand.iter().any(|&m| m == n))
        .unwrap();

    // Splice our hand into the cups after the destination cup
    hand.iter().copied().fold(dest, |prev, cur| {
        let skip = successors[prev as usize];
        successors[prev as usize] = cur;
        successors[cur as usize] = skip;
        cur
    });

    // Return the new current cup
    successors[current_cup as usize]
}

fn play_round_part2(current_cup: u32, successors: &mut [u32; CARDS_PART2]) -> u32 {
    // Pick up the cups after the current one
    let mut hand = [0; PICKUP_NUM];

    macro_rules! get_mut {
        ($idx:expr) => {
            unsafe {
                match successors.get_mut($idx as usize) {
                    Some(val) => val,
                    None => std::hint::unreachable_unchecked(),
                }
            }
        };
    }

    let past_hand = hand.iter_mut().fold(*get_mut!(current_cup), |prev, dest| {
        let previous_successor = *get_mut!(prev);
        *dest = prev;
        previous_successor
    });

    *get_mut!(current_cup) = past_hand;

    // Find the destination cup
    let dest = (1..=3)
        .take_while(|&n| n < current_cup)
        .map(|n| current_cup - n)
        .chain(
            (current_cup.checked_sub(4).unwrap_or(1)..successors.len() as u32)
                .rev()
                .filter(|&n| n != current_cup),
        )
        .find(|&n| !hand.iter().any(|&m| m == n))
        .unwrap();

    // Splice our hand into the cups after the destination cup
    hand.iter().copied().fold(dest, |prev, cur| {
        let prev = get_mut!(prev);
        let skip = *prev;
        *prev = cur;
        *get_mut!(cur) = skip;
        cur
    });

    // Return the new current cup
    *get_mut!(current_cup)
}

#[inline]
pub fn parse_input() -> Vec<u8> {
    include_str!("input.txt")
        .trim()
        .bytes()
        .map(|b| b - b'0')
        .collect::<Vec<_>>()
}

#[inline]
pub fn solve_part1(cups: &[u8]) -> CupsDisplay {
    let current_cup = cups[0];
    let mut successors = [0; CARDS_PART1];
    let last = cups.iter().skip(1).fold(current_cup, |prev, &cur| {
        successors[prev as usize] = cur;
        cur
    });
    successors[last as usize] = current_cup;

    (0..MOVES_PART1).fold(current_cup, |current_cup, _| {
        play_round_part1(current_cup, &mut successors)
    });

    CupsDisplay { successors }
}

#[inline]
pub fn solve_part2(initial_cups: &[u8]) -> u64 {
    let current_cup = initial_cups[0] as u32;

    // SAFETY: trust me
    let mut successors: Box<[u32; CARDS_PART2]> = unsafe { Box::new_uninit().assume_init() };

    let last = initial_cups
        .iter()
        .skip(1)
        .map(|&n| n as u32)
        .chain(CARDS_PART1 as u32..CARDS_PART2 as u32)
        .fold(current_cup, |prev, cur| {
            successors[prev as usize] = cur;
            cur
        });
    successors[last as usize] = current_cup;

    (0..MOVES_PART2).fold(current_cup, |current_cup, _| {
        play_round_part2(current_cup, &mut successors)
    });

    let a = successors[1];
    let b = successors[a as usize];
    a as u64 * b as u64
}

#[inline]
pub fn solve() -> (CupsDisplay, u64) {
    let cups = parse_input();
    (solve_part1(&cups), solve_part2(&cups))
}
