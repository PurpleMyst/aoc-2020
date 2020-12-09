use std::cmp::Ordering;

use itertools::Itertools as _;

const PREAMBLE: usize = 25;

fn pairs<T: Copy>(items: &[T]) -> impl Iterator<Item = (T, T)> + '_ {
    items
        .iter()
        .enumerate()
        .flat_map(move |(idx, &a)| items.iter().skip(idx + 1).map(move |&b| (a, b)))
}

#[inline]
pub fn solve() -> (u64, u64) {
    let numbers = include_str!("input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();

    let part1 = numbers
        .windows(PREAMBLE + 1)
        .find_map(|window| {
            let (&target, rest) = window.split_last().unwrap();

            // POSSIBLE OPTIMIZATION:
            // You don't really need to re-calculate all the pairs every time,
            // you can just calculate the ones that involve the new element and
            // remove the ones that involved the old first element
            if pairs(rest).map(|(a, b)| a + b).all(|sum| sum != target) {
                Some(target)
            } else {
                None
            }
        })
        .unwrap();

    let mut left_it = numbers.iter();
    let mut left_idx = 0;

    let mut right_it = numbers.iter();
    let mut right_idx = 0;

    let mut sum = 0;

    loop {
        match sum.cmp(&part1) {
            Ordering::Less => {
                sum += right_it.next().unwrap();
                right_idx += 1;
            }

            Ordering::Greater => {
                sum -= left_it.next().unwrap();
                left_idx += 1;
            }

            Ordering::Equal => break,
        }
    }

    let (min, max) = numbers[left_idx..right_idx]
        .iter()
        .minmax()
        .into_option()
        .unwrap();
    let part2 = min + max;

    (part1, part2)
}
