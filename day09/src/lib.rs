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

            if pairs(rest).map(|(a, b)| a + b).all(|sum| sum != target) {
                Some(target)
            } else {
                None
            }
        })
        .unwrap();

    let part2 = (2..=numbers.len())
        .find_map(|size| {
            let mut sum: u64 = numbers.iter().take(size).sum();

            if sum == part1 {
                let (min, max) = numbers.iter().take(size).minmax().into_option().unwrap();
                return Some(min + max);
            }

            for (prev, (idx, &item)) in numbers.iter().zip(numbers.iter().enumerate().skip(size)) {
                sum = sum - prev + item;

                if sum == part1 {
                    let (min, max) = numbers
                        .iter()
                        .skip(idx - size + 1)
                        .take(size)
                        .minmax()
                        .into_option()
                        .unwrap();

                    return Some(min + max);
                }
            }

            None
        })
        .unwrap();

    (part1, part2)
}
