use std::mem::replace;

const IDX_PART1: usize = 2020;
const IDX_PART2: usize = 30_000_000;
const SEED_LEN: usize = 6;

#[inline]
pub fn solve() -> (usize, usize) {
    let mut prev = 0;

    let mut last_seen = vec![0; IDX_PART2];

    include_str!("input.txt")
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .enumerate()
        .inspect(|(_idx, n)| prev = *n)
        .for_each(|(idx, n)| last_seen[n] = idx + 1);

    let mut sequence = (SEED_LEN..).map(|turn| {
        let next = match last_seen[prev] {
            0 => 0,
            lturn => turn - lturn,
        };
        last_seen[prev] = turn;
        replace(&mut prev, next)
    });

    let part1 = sequence.nth(IDX_PART1 - SEED_LEN).unwrap();
    let part2 = sequence
        .nth(IDX_PART2 - (IDX_PART1 - SEED_LEN) - (SEED_LEN + 1))
        .unwrap();

    (part1, part2)
}
