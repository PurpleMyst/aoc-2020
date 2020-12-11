use std::{convert::TryFrom, iter::successors};

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum Cell {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

fn solve_part1(width: usize, mut seats: Vec<Cell>) -> usize {
    let mut next_seats = seats.clone();

    loop {
        next_seats.clone_from(&seats);

        let mut it = next_seats.iter_mut();

        for (y, row) in seats.chunks_exact(width).enumerate() {
            for ((x, &col), wot) in row.iter().enumerate().zip(it.by_ref()) {
                if col == Cell::Floor {
                    continue;
                }

                let occupied_neighbors = (-1..=1)
                    .map(|dy| {
                        (-1..=1)
                            .filter_map(|dx| {
                                if dx == 0 && dy == 0 {
                                    return None;
                                }

                                let x = usize::try_from((x as isize) + dx).ok()?;
                                let y = usize::try_from((y as isize) + dy).ok()?;

                                if x >= width {
                                    return None;
                                }

                                if *seats.get(y * width + x)? == Cell::OccupiedSeat {
                                    Some(())
                                } else {
                                    None
                                }
                            })
                            .count()
                    })
                    .sum::<usize>();

                let next_col = if col == Cell::EmptySeat && occupied_neighbors == 0 {
                    Cell::OccupiedSeat
                } else if occupied_neighbors >= 4 {
                    Cell::EmptySeat
                } else {
                    continue;
                };

                *wot = next_col;
            }
        }

        if next_seats == seats {
            break;
        }

        seats.clone_from(&next_seats);
    }

    seats
        .iter()
        .filter(|&&cell| cell == Cell::OccupiedSeat)
        .count()
}

fn solve_part2(width: usize, mut seats: Vec<Cell>) -> usize {
    let height = seats.len() / width;

    let mut next_seats = seats.clone();

    loop {
        next_seats.clone_from(&seats);

        for (y, row) in seats.chunks_exact(width).enumerate() {
            for (x, &col) in row.iter().enumerate() {
                if col == Cell::Floor {
                    continue;
                }

                let occupied_neighbors = (-1..=1)
                    .map(|dy| {
                        (-1..=1)
                            .filter_map(|dx| {
                                if dx == 0 && dy == 0 {
                                    return None;
                                }

                                let mut ray = successors(Some((x, y)), |&(x, y)| {
                                    let x = usize::try_from((x as isize) + dx).ok()?;
                                    let y = usize::try_from((y as isize) + dy).ok()?;

                                    if x >= width || y >= height {
                                        return None;
                                    };

                                    Some((x, y))
                                })
                                .skip(1);

                                match ray
                                    .map(|(x, y)| seats[y * width + x])
                                    .find(|&cell| cell != Cell::Floor)
                                {
                                    Some(Cell::OccupiedSeat) => Some(()),
                                    _ => None,
                                }
                            })
                            .count()
                    })
                    .sum::<usize>();

                let next_col = if col == Cell::EmptySeat && occupied_neighbors == 0 {
                    Cell::OccupiedSeat
                } else if occupied_neighbors >= 5 {
                    Cell::EmptySeat
                } else {
                    continue;
                };

                next_seats[y * width + x] = next_col;
            }
        }

        if next_seats == seats {
            break;
        }

        seats.clone_from(&next_seats);
    }

    seats
        .iter()
        .filter(|&&cell| cell == Cell::OccupiedSeat)
        .count()
}

#[inline]
pub fn solve() -> (usize, usize) {
    let mut width = 0;

    let cells: Vec<Cell> = include_str!("input.txt")
        .lines()
        .flat_map(|row| {
            width = row.len();

            row.bytes().map(|b| match b {
                b'.' => Cell::Floor,
                b'L' => Cell::EmptySeat,
                b'#' => Cell::OccupiedSeat,
                _ => unreachable!(),
            })
        })
        .collect();

    (solve_part1(width, cells.clone()), solve_part2(width, cells))
}
