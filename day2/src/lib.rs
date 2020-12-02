#[inline]
pub fn solve() -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;

    include_str!("input.txt").lines().for_each(|line| {
        let mut parts = line.splitn(3, ' ');
        let range = parts.next().unwrap();
        let letter = parts.next().unwrap().chars().next().unwrap() as u8;
        let password = parts.next().unwrap().as_bytes();

        let mut range_parts = range.splitn(2, '-');
        let low = range_parts.next().unwrap().parse::<usize>().unwrap();
        let high = range_parts.next().unwrap().parse::<usize>().unwrap();

        let present = password.iter().filter(|&&c| c == letter).count();

        if present >= low && present <= high {
            part1 += 1;
        }

        if (password[low - 1] == letter) ^ (password[high - 1] == letter) {
            part2 += 1;
        }
    });

    (part1, part2)
}
