use std::collections::HashSet;

/* TODO:
 * Try out u8 instead of usize
 * Width is 31. Maybe we could repeat once in the set and use bitwise AND?
 * Replace HashSet with Vec<bool>
*/

fn count(
    width: usize,
    height: usize,
    trees: &HashSet<(usize, usize)>,
    slope_right: usize,
    slope_down: usize,
) -> usize {
    (0..)
        .map(|i| ((slope_right * i) % width, (slope_down * i)))
        .take_while(|&(_, y)| y <= height)
        .filter(|&(x, y)| trees.contains(&(x, y)))
        .count()
}

pub fn solve() -> (usize, usize) {
    let mut width = 0;
    let mut height = 0;

    let trees = include_str!("input.txt")
        .lines()
        .enumerate()
        .inspect(|&(y, row)| {
            height = y;
            width = row.len();
        })
        .flat_map(|(y, row)| {
            row.bytes()
                .enumerate()
                .filter(|&(_, ch)| ch == b'#')
                .map(move |(x, _)| (x, y))
        })
        .collect::<HashSet<_>>();

    dbg!(width, height);

    let part1 = count(width, height, &trees, 3, 1);

    let part2 = part1
        * [(1, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|&(slope_right, slope_down)| count(width, height, &trees, slope_right, slope_down))
            .product::<usize>();

    (part1, part2)
}
