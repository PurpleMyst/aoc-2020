use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

#[inline]
pub fn solve_part1(initial_state: &[(i8, i8)]) -> usize {
    let mut map = initial_state
        .iter()
        .map(|&(x, y)| (x, y, 0i8))
        .collect::<HashSet<_>>();

    let mut new_peeps = Vec::new();
    let mut unalive: HashMap<_, usize> = HashMap::default();

    for _ in 0..6 {
        for &(x, y, z) in &map {
            let mut active_neighbors = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        if dx == 0 && dy == 0 && dz == 0 {
                            continue;
                        }

                        let neighbor = (x + dx, y + dy, z + dz);

                        if map.contains(&neighbor) {
                            active_neighbors += 1;
                        } else {
                            *unalive.entry(neighbor).or_default() += 1;
                        }
                    }
                }
            }

            if active_neighbors == 2 || active_neighbors == 3 {
                new_peeps.push((x, y, z))
            }
        }

        new_peeps.extend(
            unalive
                .drain()
                .filter(|&(_, neighbors)| neighbors == 3)
                .map(|(pos, _)| pos),
        );

        map.clear();
        map.extend(new_peeps.drain(..));
    }

    map.len()
}

#[inline]
pub fn solve_part2(initial_state: &[(i8, i8)]) -> usize {
    let mut map = initial_state
        .iter()
        .map(|&(x, y)| (x, y, 0i8, 0i8))
        .collect::<HashSet<_>>();

    let mut new_peeps = HashSet::default();
    let mut unalive: HashMap<_, usize> = HashMap::default();

    for _ in 0..6 {
        new_peeps.clear();
        new_peeps.reserve(map.len() * 2);
        unalive.reserve(map.len() * 8);

        for &(x, y, z, w) in &map {
            let mut active_neighbors = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        for dw in -1..=1 {
                            if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                continue;
                            }
                            let neighbor = (x + dx, y + dy, z + dz, w + dw);

                            if map.contains(&neighbor) {
                                active_neighbors += 1;
                            } else {
                                *unalive.entry(neighbor).or_default() += 1;
                            }
                        }
                    }
                }
            }

            if active_neighbors == 2 || active_neighbors == 3 {
                new_peeps.insert((x, y, z, w));
            }
        }

        new_peeps.extend(
            unalive
                .drain()
                .filter(|&(_, neighbors)| neighbors == 3)
                .map(|(pos, _)| pos),
        );

        map.clone_from(&new_peeps);
    }

    map.len()
}

#[inline]
pub fn parse_input() -> Vec<(i8, i8)> {
    include_str!("input.txt")
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes().enumerate().filter_map(move |(x, ch)| {
                if ch == b'#' {
                    Some((x as i8, y as i8))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>()
}

#[inline]
pub fn solve() -> (usize, usize) {
    let initial_state = parse_input();
    (solve_part1(&initial_state), solve_part2(&initial_state))
}
