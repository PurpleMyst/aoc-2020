use std::mem::{transmute, MaybeUninit};

use crate::tile::Tile;

/// Find the corners of the image by taking advantage of the constraint that
/// "the outermost edges won't line up with any other tiles."
pub(crate) fn find_corners(tiles: &[Tile]) -> ([usize; 4], [Tile; 4]) {
    let mut indices = [0; 4];

    let mut corners: [MaybeUninit<Tile>; 4] = unsafe { MaybeUninit::uninit().assume_init() };

    tiles
        .iter()
        .copied()
        .enumerate()
        .filter(|&(idx, tile)| {
            tiles
                .iter()
                .take(idx)
                .chain(tiles.iter().skip(idx + 1))
                .filter(|other| tile.could_fit(other))
                .count()
                == 2
        })
        .zip(indices.iter_mut().zip(corners.iter_mut()))
        .for_each(|((idx, tile), (idx_dest, tile_dest))| {
            *idx_dest = idx;
            *tile_dest = MaybeUninit::new(tile);
        });

    (indices, unsafe { transmute(corners) })
}
