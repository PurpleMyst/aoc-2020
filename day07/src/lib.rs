use std::cell::RefCell;
use std::collections::HashMap;

use itertools::Itertools as _;

type Bags = HashMap<&'static str, Vec<(usize, &'static str)>>;

const WANTED: &str = "shiny gold";

struct Solver {
    bags: Bags,
    cache: RefCell<HashMap<&'static str, bool>>,
}

impl Solver {
    fn new(bags: Bags) -> Self {
        Self {
            cache: RefCell::new(HashMap::with_capacity(bags.len())),
            bags,
        }
    }

    fn contains_wanted(&self, bag: &'static str) -> bool {
        if let Some(&value) = self.cache.borrow().get(bag) {
            return value;
        }

        let value = self
            .bags
            .get(bag)
            .unwrap()
            .iter()
            .any(|&(_count, bag)| bag == WANTED || self.contains_wanted(&bag));

        self.cache.borrow_mut().insert(bag, value);

        value
    }

    fn count_inside(&self, bag: &'static str) -> usize {
        self.bags
            .get(bag)
            .unwrap()
            .iter()
            .map(|&(count, bag)| count + count * self.count_inside(bag))
            .sum::<usize>()
    }
}

#[inline]
pub fn solve() -> (usize, usize) {
    let bags: Bags = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.splitn(2, " contain ").collect_tuple().unwrap();

            let holder = lhs.rsplitn(2, ' ').nth(1).unwrap();

            let contains = if rhs.starts_with("no") {
                vec![]
            } else {
                rhs[..rhs.len() - 1]
                    .split(", ")
                    .map(|bag| {
                        let (count, color) = bag
                            .rsplitn(2, ' ')
                            .nth(1)
                            .unwrap()
                            .splitn(2, ' ')
                            .collect_tuple()
                            .unwrap();

                        (count.parse::<usize>().unwrap(), color)
                    })
                    .collect::<Vec<_>>()
            };

            (holder, contains)
        })
        .collect::<HashMap<_, _>>();

    let solver = Solver::new(bags);

    let part1 = solver
        .bags
        .keys()
        .filter(|&bag| solver.contains_wanted(bag))
        .count();

    let part2 = solver.count_inside(WANTED);

    (part1, part2)
}
