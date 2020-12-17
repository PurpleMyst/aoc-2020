fn solve_part1(adapters: &[u8]) -> usize {
    let mut ones = 0;
    let mut threes = 0;

    for (a, b) in adapters.iter().zip(adapters.iter().skip(1)) {
        match b - a {
            1 => ones += 1,
            3 => threes += 1,
            _ => {}
        }
    }

    ones * threes
}

fn solve_part2(adapters: &[u8]) -> u64 {
    let mut edges = [0; 256];
    edges[0] = 1;
    edges[1] = 1;
    edges[2] = 2;
    debug_assert!(adapters.contains(&0) && adapters.contains(&1) && adapters.contains(&2));

    for &n in adapters.iter().skip(3) {
        edges[n as usize] =
            edges[(n - 1) as usize] + edges[(n - 2) as usize] + edges[(n - 3) as usize];
    }

    edges[*adapters.iter().max().unwrap() as usize]
}

#[inline]
pub fn solve() -> (usize, u64) {
    let mut adapters = include_str!("input.txt")
        .lines()
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    adapters.sort_unstable();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);

    let part1 = solve_part1(&adapters);
    let part2 = solve_part2(&adapters);

    (part1, part2)
}
