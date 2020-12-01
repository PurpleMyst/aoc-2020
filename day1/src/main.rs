use std::collections::HashSet;

use itertools::Itertools as _;

const TARGET: u64 = 2020;

fn main() {
    let expenses = include_str!("input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<HashSet<_>>();

    let n = expenses
        .iter()
        .find(|&&n| {
            TARGET
                .checked_sub(n)
                .map(|m| expenses.contains(&m))
                .unwrap_or(false)
        })
        .unwrap();
    println!("{}", n * (2020 - n));

    let (&a, &b) = expenses
        .iter()
        .tuple_combinations::<(_, _)>()
        .find(|&(&a, &b)| {
            TARGET
                .checked_sub(a + b)
                .map(|c| expenses.contains(&c))
                .unwrap_or(false)
        })
        .unwrap();

    println!("{}", a * b * (2020 - (a + b)));
}
