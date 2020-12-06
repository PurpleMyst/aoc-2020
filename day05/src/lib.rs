use std::cmp::max;

const HEIGHT: usize = 128;
const WIDTH: usize = 8;

fn position(boarding_pass: &str) -> (usize, usize) {
    let mut cs = boarding_pass.bytes();

    let row = {
        let mut lo = 0;
        let mut hi = HEIGHT - 1;

        cs.by_ref().take(7).for_each(|ch| {
            let mid = (lo + hi) / 2;
            match ch {
                b'F' => hi = mid,
                b'B' => lo = mid + 1,
                _ => unreachable!(),
            }
        });

        debug_assert_eq!(lo, hi);
        lo
    };

    let col = {
        let mut lo = 0;
        let mut hi = WIDTH - 1;

        cs.take(3).for_each(|ch| {
            let mid = (lo + hi) / 2;
            match ch {
                b'L' => hi = mid,
                b'R' => lo = mid + 1,
                _ => unreachable!(),
            }
        });

        debug_assert_eq!(lo, hi);
        lo
    };

    (col, row)
}

#[inline]
pub fn solve() -> (usize, usize) {
    let mut rows = [0u8; HEIGHT];

    let part1 = include_str!("input.txt")
        .lines()
        .map(position)
        .fold(0, |prev, (col, row)| {
            rows[row] |= 1 << col;

            max(prev, row * WIDTH + col)
        });

    let part2 = rows
        .iter()
        .enumerate()
        .skip(2) // Assumption: we won't be sitting in the first 2 rows which are edge case-y
        .find_map(|(y, row)| {
            let col = row.leading_ones();
            if col != 8 {
                Some(y * WIDTH + (WIDTH - col as usize - 1))
            } else {
                None
            }
        })
        .unwrap();

    (part1, part2)
}
