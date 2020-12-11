use std::{convert::TryFrom, iter::successors};

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Cell {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

const WIDTH: usize = 96;
const HEIGHT: usize = 98;

#[inline]
pub fn parse_input() -> Vec<Cell> {
    let cells: Vec<Cell> = include_str!("input.txt")
        .lines()
        .flat_map(|row| {
            row.bytes().map(|b| match b {
                b'.' => Cell::Floor,
                b'L' => Cell::EmptySeat,
                b'#' => Cell::OccupiedSeat,
                _ => unreachable!(),
            })
        })
        .collect();
    debug_assert_eq!(cells.len(), WIDTH * HEIGHT);

    cells
}

#[inline]
pub fn solve_part1(seats: &mut [Cell]) -> usize {
    let mut next_seats = seats.to_vec();

    loop {
        next_seats.copy_from_slice(&seats);

        let mut it = next_seats.iter_mut();

        for (y, row) in seats.chunks_exact(WIDTH).enumerate() {
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

                                if x >= WIDTH {
                                    return None;
                                }

                                if *seats.get(y * WIDTH + x)? == Cell::OccupiedSeat {
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

        seats.copy_from_slice(&next_seats);
    }

    seats
        .iter()
        .filter(|&&cell| cell == Cell::OccupiedSeat)
        .count()
}

#[inline]
pub fn solve_part2(seats: &mut [Cell]) -> usize {
    let mut next_seats = seats.to_vec();

    loop {
        next_seats.copy_from_slice(&seats);

        for (y, row) in seats.chunks_exact(WIDTH).enumerate() {
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

                                let ray = successors(Some((x, y)), |&(x, y)| {
                                    let x = usize::try_from((x as isize) + dx).ok()?;
                                    let y = usize::try_from((y as isize) + dy).ok()?;

                                    if x >= WIDTH || y >= HEIGHT {
                                        return None;
                                    };

                                    Some((x, y))
                                })
                                .skip(1);

                                match ray
                                    .map(|(x, y)| seats[y * WIDTH + x])
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

                next_seats[y * WIDTH + x] = next_col;
            }
        }

        if next_seats == seats {
            break;
        }

        seats.copy_from_slice(&next_seats);
    }

    seats
        .iter()
        .filter(|&&cell| cell == Cell::OccupiedSeat)
        .count()
}

#[inline]
pub fn solve() -> (usize, usize) {
    let mut cells = parse_input();
    (
        {
            let mut cells = cells.clone();
            solve_part1(&mut cells[..])
        },
        solve_part2(&mut cells[..]),
    )
}
