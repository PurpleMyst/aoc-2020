use bitvec::prelude::*;

use itertools::Itertools as _;

const TARGET: usize = 2020;

#[inline]
pub fn solve() -> (usize, usize) {
    let mut expenses_set = bitarr![Lsb0, u64; 0; TARGET];

    let expenses = include_str!("input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .inspect(|&n| expenses_set.set(n, true))
        .collect::<Vec<_>>();

    let n = expenses
        .iter()
        .find(|&&n| {
            TARGET
                .checked_sub(n)
                .map(|m| expenses_set[m])
                .unwrap_or(false)
        })
        .unwrap();
    let part1 = n * (TARGET - n);

    let (&a, &b) = expenses
        .iter()
        .tuple_combinations::<(_, _)>()
        .find(|&(&a, &b)| {
            TARGET
                .checked_sub(a + b)
                .map(|c| expenses_set[c])
                .unwrap_or(false)
        })
        .unwrap();

    let part2 = a * b * (TARGET - (a + b));

    (part1, part2)
}
