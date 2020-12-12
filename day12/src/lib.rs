use std::mem::swap;

//                                    E       S         W        N
const ANGLES: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Direction {
    Move(isize, isize),
    Rotate(isize),
    Forward(isize),
}

#[inline]
pub fn solve_part1(directions: impl Iterator<Item = Direction>) -> isize {
    let mut x = 0;
    let mut y = 0;
    let mut a = 0;

    for direction in directions {
        match direction {
            Direction::Move(dx, dy) => {
                x += dx;
                y += dy;
            }

            Direction::Rotate(da) => a += da,

            Direction::Forward(n) => {
                let (dx, dy) = ANGLES[(a % 4) as usize];
                x += dx * n;
                y += dy * n;
            }
        }
    }

    x.abs() + y.abs()
}

#[inline]
pub fn solve_part2(directions: impl Iterator<Item = Direction>) -> isize {
    let mut x: isize = 0;
    let mut y: isize = 0;

    let mut wx = 10;
    let mut wy = -1;

    for direction in directions {
        match direction {
            Direction::Move(dx, dy) => {
                wx += dx;
                wy += dy;
            }

            Direction::Rotate(da) => {
                for _ in 0..da {
                    swap(&mut wx, &mut wy);
                    wx = -wx;
                }
            }

            Direction::Forward(n) => {
                x += wx * n;
                y += wy * n;
            }
        }
    }

    x.abs() + y.abs()
}

#[inline]
pub fn parse_input() -> impl Iterator<Item = Direction> + Clone {
    include_str!("input.txt").lines().map(|line| {
        let mut it = line.chars();
        let dir = it.next().unwrap();
        let amount: isize = it.as_str().parse().unwrap();

        match dir {
            'N' => Direction::Move(0, -amount),
            'S' => Direction::Move(0, amount),
            'E' => Direction::Move(amount, 0),
            'W' => Direction::Move(-amount, 0),

            'L' => {
                debug_assert_eq!(amount % 90, 0);
                Direction::Rotate(4 - amount / 90)
            }

            'R' => {
                debug_assert_eq!(amount % 90, 0);
                Direction::Rotate(amount / 90)
            }

            'F' => Direction::Forward(amount),

            _ => unreachable!(),
        }
    })
}

#[inline]
pub fn solve() -> (isize, isize) {
    let directions = parse_input();
    (solve_part1(directions.clone()), solve_part2(directions))
}
