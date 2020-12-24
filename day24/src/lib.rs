use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

#[derive(Debug, Hash, Clone, Copy, Default, PartialEq, Eq)]
pub struct Hexagon {
    // Axial coordinates
    q: i64,
    r: i64,
}

impl Hexagon {
    fn e(self) -> Self {
        Self {
            q: self.q + 1,
            r: self.r,
        }
    }

    fn se(self) -> Self {
        Self {
            q: self.q,
            r: self.r + 1,
        }
    }

    fn sw(self) -> Self {
        Self {
            q: self.q - 1,
            r: self.r + 1,
        }
    }

    fn w(self) -> Self {
        Self {
            q: self.q - 1,
            r: self.r,
        }
    }

    fn nw(self) -> Self {
        Self {
            q: self.q,
            r: self.r - 1,
        }
    }

    fn ne(self) -> Self {
        Self {
            q: self.q + 1,
            r: self.r - 1,
        }
    }

    fn from_input(input: &str) -> Self {
        let mut it = input.chars();
        let mut h = Self::default();

        while let Some(ch) = it.next() {
            h = match ch {
                'e' => h.e(),
                'w' => h.w(),

                's' => match it.next().unwrap() {
                    'e' => h.se(),
                    'w' => h.sw(),
                    _ => unreachable!(),
                },

                'n' => match it.next().unwrap() {
                    'e' => h.ne(),
                    'w' => h.nw(),
                    _ => unreachable!(),
                },

                _ => unreachable!(),
            }
        }

        h
    }

    fn neighbors(self) -> [Self; 6] {
        [
            self.e(),
            self.se(),
            self.ne(),
            self.w(),
            self.sw(),
            self.nw(),
        ]
    }
}

pub fn solve_part2(mut black: HashSet<Hexagon>) -> usize {
    let mut next_state = HashSet::default();
    let mut white: HashMap<Hexagon, usize> = HashMap::default();

    for _ in 0..100 {
        next_state.clear();
        next_state.reserve(black.len() * 2);
        white.reserve(black.len() * 8);

        for tile in black.iter() {
            let mut active_neighbors = 0;

            for &neighbor in tile.neighbors().iter() {
                if black.contains(&neighbor) {
                    active_neighbors += 1;
                } else {
                    *white.entry(neighbor).or_default() += 1;
                }
            }

            if active_neighbors == 1 || active_neighbors == 2 {
                next_state.insert(*tile);
            }
        }

        next_state.extend(
            white
                .drain()
                .filter(|&(_, neighbors)| neighbors == 2)
                .map(|(tile, _)| tile),
        );

        black.clone_from(&next_state);
    }
    black.len()
}

#[inline]
pub fn solve() -> (usize, usize) {
    let mut black = HashSet::default();

    include_str!("input.txt")
        .lines()
        .map(Hexagon::from_input)
        .for_each(|h| {
            if black.contains(&h) {
                black.remove(&h);
            } else {
                black.insert(h);
            }
        });

    let part1 = black.len();
    let part2 = solve_part2(black);

    (part1, part2)
}
