use bitvec::prelude::*;

const TARGET: usize = 2020;

fn pairs<T: Copy>(items: &[T]) -> impl Iterator<Item = (T, T)> + '_ {
    items
        .iter()
        .enumerate()
        .flat_map(move |(idx, &a)| items.iter().skip(idx + 1).map(move |&b| (a, b)))
}

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

    let (a, b) = pairs(&expenses)
        .find(|&(a, b)| TARGET.checked_sub(a + b).map_or(false, |c| expenses_set[c]))
        .unwrap();

    let part2 = a * b * (TARGET - (a + b));

    (part1, part2)
}
