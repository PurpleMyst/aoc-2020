use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let expenses = include_str!("input.txt")
        .lines()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<HashSet<_>>();

    let n = expenses
        .iter()
        .find(|&&n| {
            2020u64
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
            2020u64
                .checked_sub(a + b)
                .map(|c| expenses.contains(&c))
                .unwrap_or(false)
        })
        .unwrap();

    println!("{}", a * b * (2020 - (a + b)));
}
