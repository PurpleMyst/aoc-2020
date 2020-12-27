use array_iterator::ArrayIterator;
use static_assert_macro::static_assert;

use super::IMAGE_SIDE;
use crate::tile::{Tile, TILE_SIDE};

const SEAMONSTER_TOP: u128 = 0b00000000000000000010;
const SEAMONSTER_MID: u128 = 0b10000110000110000111;
const SEAMONSTER_BTM: u128 = 0b01001001001001001000;

const SEAMONSTER_WIDTH: usize = 20;
pub(crate) const SEAMONSTER_AREA: usize = (SEAMONSTER_TOP.count_ones()
    + SEAMONSTER_MID.count_ones()
    + SEAMONSTER_BTM.count_ones()) as usize;

static_assert!((TILE_SIDE - 2) * IMAGE_SIDE <= 128);
pub(crate) type InteriorImage = [u128; IMAGE_SIDE * (TILE_SIDE - 2)];

fn rotate_cw(mut interior: InteriorImage) -> InteriorImage {
    macro_rules! set_bit {
        ($matrix:ident, $dest_row:expr, $dest_col:expr, $src_row: expr, $src_col: expr$(,)?) => {
            let dest_idx = IMAGE_SIDE * (TILE_SIDE - 2) - ($dest_col + 1);
            $matrix[$dest_row] &= !(1 << dest_idx);
            let src_idx = IMAGE_SIDE * (TILE_SIDE - 2) - ($src_col + 1);
            let src_bit = ($src_row & (1 << src_idx)) >> src_idx;
            $matrix[$dest_row] |= src_bit << dest_idx;
        };
    }

    for i in 0..IMAGE_SIDE * (TILE_SIDE - 2) / 2 {
        for j in i..IMAGE_SIDE * (TILE_SIDE - 2) - i - 1 {
            let temp = interior[i];

            set_bit!(
                interior,
                i,
                j,
                interior[IMAGE_SIDE * (TILE_SIDE - 2) - 1 - j],
                i
            );
            set_bit!(
                interior,
                IMAGE_SIDE * (TILE_SIDE - 2) - 1 - j,
                i,
                interior[IMAGE_SIDE * (TILE_SIDE - 2) - 1 - i],
                IMAGE_SIDE * (TILE_SIDE - 2) - 1 - j,
            );
            set_bit!(
                interior,
                IMAGE_SIDE * (TILE_SIDE - 2) - 1 - i,
                IMAGE_SIDE * (TILE_SIDE - 2) - 1 - j,
                interior[j],
                IMAGE_SIDE * (TILE_SIDE - 2) - 1 - i,
            );
            set_bit!(interior, j, IMAGE_SIDE * (TILE_SIDE - 2) - 1 - i, temp, j);
        }
    }

    interior
}

fn flip_horizontally(mut interior: InteriorImage) -> InteriorImage {
    interior
        .iter_mut()
        .for_each(|elem| *elem = elem.reverse_bits() >> (128 - IMAGE_SIDE * (TILE_SIDE - 2)));

    interior
}

fn possible_transformations(interior: InteriorImage) -> [InteriorImage; 8] {
    [
        interior,
        rotate_cw(interior),
        rotate_cw(rotate_cw(interior)),
        rotate_cw(rotate_cw(rotate_cw(interior))),
        flip_horizontally(interior),
        rotate_cw(flip_horizontally(interior)),
        rotate_cw(rotate_cw(flip_horizontally(interior))),
        rotate_cw(rotate_cw(rotate_cw(flip_horizontally(interior)))),
    ]
}

fn load_interiors(shape: [Tile; IMAGE_SIDE * IMAGE_SIDE]) -> InteriorImage {
    let mut result = [0u128; IMAGE_SIDE * (TILE_SIDE - 2)];
    let mut shape = shape.iter();

    result.chunks_exact_mut(TILE_SIDE - 2).for_each(|row| {
        shape
            .by_ref()
            .take(IMAGE_SIDE)
            .enumerate()
            .for_each(|(x, tile)| {
                tile.interior.iter().enumerate().for_each(|(yoff, &tile)| {
                    row[yoff] |= u128::from(tile) << ((TILE_SIDE - 2) * (IMAGE_SIDE - (x + 1)));
                });
            })
    });

    result
}

fn count_seamonsters(map: InteriorImage) -> usize {
    let mut cnt = 0;

    for window in map.windows(3) {
        let top = window[0];
        let mid = window[1];
        let btm = window[2];

        for i in 0..IMAGE_SIDE * (TILE_SIDE - 2) - SEAMONSTER_WIDTH {
            let is_monster = (top & (SEAMONSTER_TOP << i)) == (SEAMONSTER_TOP << i)
                && (mid & (SEAMONSTER_MID << i)) == (SEAMONSTER_MID << i)
                && (btm & (SEAMONSTER_BTM << i)) == (SEAMONSTER_BTM << i);

            if is_monster {
                cnt += 1;
            }
        }
    }

    cnt
}

pub(crate) fn solve_part2(shape: [Tile; IMAGE_SIDE * IMAGE_SIDE]) -> usize {
    ArrayIterator::new(possible_transformations(load_interiors(shape)))
        .find_map(|map| match count_seamonsters(map) {
            0 => None,
            monsters => Some(
                map.iter()
                    .map(|row| row.count_ones() as usize)
                    .sum::<usize>()
                    - monsters * SEAMONSTER_AREA,
            ),
        })
        .unwrap()
}
