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

    let mut i = numbers.iter();
    let mut i_idx = 0;

    let mut j = numbers.iter();
    let mut j_idx = 0;

    let mut sum = 0;

    loop {
        match sum.cmp(&part1) {
            Ordering::Less => {
                j_idx += 1;
                sum += j.next().unwrap();
            }

            Ordering::Greater => {
                i_idx += 1;
                sum -= i.next().unwrap()
            }

            Ordering::Equal => break,
        }
    }

    let (min, max) = numbers[i_idx..j_idx].iter().minmax().into_option().unwrap();
    let part2 = min + max;

    (part1, part2)
}
