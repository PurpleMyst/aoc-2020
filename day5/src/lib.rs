use std::cmp::max;

use bitvec::prelude::*;

const HEIGHT: usize = 128;
const WIDTH: usize = 8;

fn seat_id(pass: &str) -> usize {
    let mut cs = pass.chars();

    let row = {
        let mut lo = 0;
        let mut hi = HEIGHT - 1;

        cs.by_ref().take(7).for_each(|ch| {
            let mid = (lo + hi) / 2;
            match ch {
                'F' => hi = mid,
                'B' => lo = mid + 1,
                _ => unreachable!(),
            }
        });

        debug_assert_eq!(lo, hi);
        lo
    };

    let col = {
        let mut lo = 0;
        let mut hi = WIDTH - 1;

        cs.take(3).for_each(|ch| {
            let mid = (lo + hi) / 2;
            match ch {
                'L' => hi = mid,
                'R' => lo = mid + 1,
                _ => unreachable!(),
            }
        });

        debug_assert_eq!(lo, hi);
        lo
    };

    row * WIDTH + col
}

/* TODO: each row could be a single u8, maybe we could use a "find first 0" function? */
pub fn solve() -> (usize, usize) {
    let mut passes = bitarr![Lsb0, u64; 0; WIDTH * HEIGHT];

    let part1 = include_str!("input.txt")
        .lines()
        .map(seat_id)
        .fold(0, |prev, id| {
            passes.set(id, true);
            max(prev, id)
        });

    for row in 1..HEIGHT {
        for col in 1..WIDTH {
            let id = row * 8 + col;
            if !passes[id] && passes[id - 1] {
                return (part1, id);
            }
        }
    }

    unreachable!();
}
