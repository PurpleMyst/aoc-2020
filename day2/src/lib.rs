use itertools::Itertools as _;

#[inline]
pub fn solve() -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;

    include_str!("input.txt").lines().for_each(|line| {
        let (range, letter, password) = line.splitn(3, ' ').collect_tuple().unwrap();
        let letter = letter.chars().next().unwrap() as u8;
        let password = password.as_bytes();

        let (low, high) = range
            .splitn(2, '-')
            .map(|bound| bound.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        let present = bytecount::naive_count_32(password, letter);
        if present >= low && present <= high {
            part1 += 1;
        }

        if (password[low - 1] == letter) ^ (password[high - 1] == letter) {
            part2 += 1;
        }
    });

    (part1, part2)
}
