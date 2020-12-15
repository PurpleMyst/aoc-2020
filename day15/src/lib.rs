use std::collections::HashMap;

const IDX_PART1: usize = 2020;
const IDX_PART2: usize = 30_000_000;

#[inline]
pub fn solve() -> (usize, usize) {
    let mut prev = 0;

    let mut last_seen = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .enumerate()
        .map(|(idx, n)| (n, idx + 1))
        .inspect(|(n, _)| prev = *n)
        .collect::<HashMap<_, _>>();

    last_seen.remove(&prev);

    for turn in last_seen.len() + 1..IDX_PART1 {
        let next = last_seen.get(&prev).map_or(0, |lturn| turn - lturn);
        last_seen.insert(prev, turn);
        prev = next;
    }
    let part1 = prev;

    for turn in IDX_PART1..IDX_PART2 {
        let next = last_seen.get(&prev).map_or(0, |lturn| turn - lturn);
        last_seen.insert(prev, turn);
        prev = next;
    }
    let part2 = prev;

    (part1, part2)
}
