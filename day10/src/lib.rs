use std::collections::BinaryHeap;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
struct Node {
    length: usize,
    jolts: u8,
    deltas: [usize; 3],
}

fn solve_part1(adapters: &[bool; 256], highest: u8) -> Node {
    let mut q = BinaryHeap::new();

    q.push(Node {
        length: 0,
        jolts: highest,
        deltas: [0; 3],
    });

    while let Some(node) = q.pop() {
        if node.jolts == 0 {
            return node;
        }

        q.extend((1..=3).filter_map(|delta| {
            if node.jolts < delta || !adapters[(node.jolts - delta) as usize] {
                return None;
            }

            let jolts = node.jolts - delta;
            let length = node.length + 1;
            let mut deltas = node.deltas;
            deltas[(delta - 1) as usize] += 1;
            let node = Node {
                length,
                jolts,
                deltas,
            };

            Some(node)
        }));
    }

    unreachable!()
}

fn count_walks(adapters: &[bool; 256], highest: u8, target: u8) -> u64 {
    let mut q = BinaryHeap::new();

    q.push(Node {
        length: 0,
        jolts: highest,
        deltas: [0; 3],
    });

    let mut total = 0;
    while let Some(node) = q.pop() {
        if node.jolts == target {
            total += 1;
        }

        q.extend((1..=3).filter_map(|delta| {
            if node.jolts < delta || !adapters[(node.jolts - delta) as usize] {
                return None;
            }

            let jolts = node.jolts - delta;
            let length = node.length + 1;
            let mut deltas = node.deltas;
            deltas[(delta - 1) as usize] += 1;
            let node = Node {
                length,
                jolts,
                deltas,
            };

            Some(node)
        }));
    }

    total
}

struct Solver {
    adapters: [bool; 256],
    block: Vec<u8>,
    walks: u64,
}

impl Solver {
    fn new(adapters: [bool; 256]) -> Self {
        Self {
            adapters,
            block: vec![],
            walks: 1,
        }
    }

    fn walk(&mut self, mut jolts: u8) {
        loop {
            self.block.clear();

            self.next_complex_block(jolts);

            if self.block.is_empty() {
                if let Some(next_jolts) = (1..=3)
                    .map(|delta| jolts + delta)
                    .find(|&next_jolts| self.adapters[next_jolts as usize])
                {
                    jolts = next_jolts;
                    continue;
                } else {
                    break;
                }
            }

            self.block.insert(0, jolts);
            jolts = *self.block.last().unwrap();

            self.walks *= self.current_block_walks(jolts);
        }
    }

    fn current_block_walks(&mut self, start: u8) -> u64 {
        let mut adapters = [false; 256];

        self.block.iter().for_each(|&n| adapters[n as usize] = true);

        let target = *self.block.first().unwrap();

        count_walks(&adapters, start as u8, target as u8)
    }

    fn next_complex_block(&mut self, start: u8) {
        let mut next = 0u8;

        for delta in 1..=3 {
            if self.adapters[(start + delta) as usize] {
                next |= 1 << delta;
            }
        }

        if next.count_ones() <= 1 {
            return;
        }

        for delta in 1..=3 {
            if self.adapters[(start + delta) as usize] {
                match self.block.binary_search(&(start + delta)) {
                    Ok(..) => {}
                    Err(idx) => self.block.insert(idx, start + delta),
                }

                self.next_complex_block(start + delta);
            }
        }
    }
}

fn solve_part2(adapters: [bool; 256]) -> u64 {
    let mut solver = Solver::new(adapters);
    solver.walk(0);

    solver.walks
}

#[inline]
pub fn solve() -> (usize, u64) {
    let mut adapters = [false; 256];

    let highest = include_str!("input.txt")
        .lines()
        .map(|n| n.parse::<u8>().unwrap())
        .inspect(|&n| adapters[n as usize] = true)
        .max()
        .unwrap();

    assert!(adapters[highest as usize]);

    let highest = highest + 3;
    adapters[highest as usize] = true;
    adapters[0] = true;

    let node = solve_part1(&adapters, highest);

    let part1 = node.deltas[0] * node.deltas[2];

    let part2 = solve_part2(adapters);

    (part1, part2)
}
