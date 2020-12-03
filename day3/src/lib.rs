use bitvec::prelude::*;

fn count(
    width: usize,
    height: usize,
    trees: &BitVec,
    slope_right: usize,
    slope_down: usize,
) -> usize {
    (0..height / slope_down)
        .map(|i| ((slope_right * i) % width, (slope_down * i)))
        .filter(|&(x, y)| trees[y * width + x])
        .count()
}

pub fn solve() -> (usize, usize) {
    let mut width = 0;
    let mut height = 0;

    let trees = include_str!("input.txt")
        .trim()
        .lines()
        .inspect(|row| {
            height += 1;
            width = row.len();
        })
        .flat_map(|row| row.bytes().map(|ch| ch == b'#'))
        .collect::<BitVec>();

    let part1 = count(width, height, &trees, 3, 1);

    let part2 = part1
        * [(1, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|&(slope_right, slope_down)| count(width, height, &trees, slope_right, slope_down))
            .product::<usize>();

    (part1, part2)
}
