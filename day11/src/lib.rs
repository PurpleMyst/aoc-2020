use std::{convert::TryFrom, iter::successors};

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Cell {
    Floor,
    EmptySeat,
    OccupiedSeat,
}
use Cell::*;

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

        let mut next_seats_it = next_seats.iter_mut();

        let mut rows = seats.chunks_exact(WIDTH);

        let mut above: &[Cell] = &[];
        let mut current: &[Cell] = rows.next().unwrap();
        let mut below: &[Cell] = rows.next().unwrap();

        for _y in 0..HEIGHT {
            for ((x, &cell), next_cell_spot) in
                current.iter().enumerate().zip(next_seats_it.by_ref())
            {
                if cell == Floor {
                    continue;
                }

                let occupied_neighbors = match x.checked_sub(1) {
                    Some(px) => [
                        above.get(px),
                        above.get(x),
                        above.get(x + 1),
                        current.get(px),
                        current.get(x + 1),
                        below.get(px),
                        below.get(x),
                        below.get(x + 1),
                    ]
                    .iter()
                    .filter(|cell| matches!(cell, Some(OccupiedSeat)))
                    .count(),

                    None => [
                        above.get(x),
                        above.get(x + 1),
                        current.get(x + 1),
                        below.get(x),
                        below.get(x + 1),
                    ]
                    .iter()
                    .filter(|cell| matches!(cell, Some(OccupiedSeat)))
                    .count(),
                };

                let next_cell = if cell == EmptySeat && occupied_neighbors == 0 {
                    OccupiedSeat
                } else if cell == OccupiedSeat && occupied_neighbors >= 4 {
                    EmptySeat
                } else {
                    continue;
                };

                *next_cell_spot = next_cell;
            }

            above = current;
            current = below;
            below = rows.next().unwrap_or(&[]);
        }

        if next_seats == seats {
            break;
        }

        seats.copy_from_slice(&next_seats);
    }

    seats.iter().filter(|&&cell| cell == OccupiedSeat).count()
}

#[inline]
pub fn solve_part2(seats: &mut [Cell]) -> usize {
    let ray_casts: Vec<[Option<usize>; 8]> = seats
        .chunks_exact(WIDTH)
        .enumerate()
        .flat_map(|(y, row)| {
            // Due to the move in the closure below, we must explicitly immutably borrow seats
            let seats = &seats;
            row.iter().enumerate().map(move |(x, &col)| {
                let mut item = [None; 8];

                if col == Floor {
                    return item;
                }

                (-1..=1)
                    .flat_map(|dy| {
                        (-1..=1).filter_map(move |dx| {
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

                            Some(
                                ray.map(|(x, y)| y * WIDTH + x)
                                    .find(|&idx| seats[idx] != Cell::Floor),
                            )
                        })
                    })
                    .zip(item.iter_mut())
                    .for_each(|(val, elem)| *elem = val);

                item
            })
        })
        .collect();

    let mut next_seats = seats.to_vec();

    loop {
        next_seats.copy_from_slice(&seats);

        let mut rays = ray_casts.iter();

        for (y, row) in seats.chunks_exact(WIDTH).enumerate() {
            for ((x, &col), rays) in row.iter().enumerate().zip(rays.by_ref()) {
                if col == Floor {
                    continue;
                }

                let occupied_neighbors = rays
                    .iter()
                    .filter(|idx| idx.map_or(false, |idx| seats[idx] == OccupiedSeat))
                    .count();

                let next_col = if col == EmptySeat && occupied_neighbors == 0 {
                    OccupiedSeat
                } else if occupied_neighbors >= 5 {
                    EmptySeat
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

    seats.iter().filter(|&&cell| cell == OccupiedSeat).count()
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
