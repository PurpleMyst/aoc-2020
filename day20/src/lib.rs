use std::mem::{transmute, MaybeUninit};

const IMAGE_SIDE: usize = 12;

pub mod tile;
use tile::Tile;

mod part1;
mod part2;

fn fill_row<'a>(
    tiles: &mut Vec<Tile>,
    init: Tile,
    slots: impl Iterator<Item = &'a mut MaybeUninit<Tile>>,
) -> Tile {
    // Build out the top row until the penultimate tile
    slots.fold(init, |prev, dest| {
        let (idx, next) = tiles
            .iter()
            .enumerate()
            .filter_map(|(dx, &other)| Some((dx, prev.fit_right(other)?)))
            .next()
            .unwrap();

        *dest = MaybeUninit::new(next);

        tiles.swap_remove(idx);

        next
    })
}

fn fill_image(mut tiles: Vec<Tile>, mut corners: [Tile; 4]) -> [Tile; IMAGE_SIDE * IMAGE_SIDE] {
    let mut image: [MaybeUninit<Tile>; IMAGE_SIDE * IMAGE_SIDE] =
        unsafe { MaybeUninit::uninit().assume_init() };

    // FIXME: iterate through all possible transformations and find the one that has both a fitting on the right and on the bottom
    let topleft = corners[0].rotate_cw();

    // Place the first corner in the top left
    image[0] = MaybeUninit::new(topleft);

    // Build out the top row until the penultimate tile
    let penultimate = fill_row(
        &mut tiles,
        topleft,
        image.iter_mut().take(IMAGE_SIDE - 1).skip(1),
    );

    // Find which corner fits there and place it there
    let (topright_corner_idx, topright) = corners
        .iter()
        .enumerate()
        .skip(1)
        .find_map(|(idx, &corner)| Some((idx, penultimate.fit_right(corner)?)))
        .unwrap();
    image[IMAGE_SIDE - 1] = MaybeUninit::new(topright);

    // Place the topright corner at index 1 so that the "used corners" are at the start of the array
    corners.swap(1, topright_corner_idx);

    // Now fill the "middle" rows iteratively
    let mut above = topleft;

    // Create an iterator over all the image slots
    let mut it = image.iter_mut().skip(IMAGE_SIDE);

    // For each middle row
    for _ in 1..=IMAGE_SIDE - 2 {
        // Find the tile which fits below the starting tile of the previous row
        let row_start = {
            let (idx, row_start) = tiles
                .iter()
                .enumerate()
                .find_map(|(idx, &corner)| Some((idx, above.fit_down(corner)?)))
                .unwrap();
            tiles.swap_remove(idx);
            row_start
        };

        // Place it in the image
        *it.next().unwrap() = MaybeUninit::new(row_start);

        // And fill the rest of the row
        fill_row(&mut tiles, row_start, it.by_ref().take(IMAGE_SIDE - 1));

        above = row_start;
    }

    let (idx, bottomleft) = corners
        .iter()
        .enumerate()
        .skip(2)
        .find_map(|(idx, &corner)| Some((idx, above.fit_down(corner)?)))
        .unwrap();
    corners.swap(2, idx);

    *it.next().unwrap() = MaybeUninit::new(bottomleft);

    let penultimate = fill_row(&mut tiles, bottomleft, it.by_ref().take(IMAGE_SIDE - 2));

    let bottomright = penultimate.fit_right(corners[3]).unwrap();
    *it.next().unwrap() = MaybeUninit::new(bottomright);

    debug_assert!(it.next().is_none());

    unsafe { transmute(image) }
}

#[inline]
pub fn solve() -> (usize, usize) {
    let mut tiles = include_str!("input.txt")
        .split("\n\n")
        .take_while(|line| !line.is_empty())
        .map(Tile::from_input)
        .collect::<Vec<Tile>>();

    // Find the corners and remove them from the tiles set
    let (mut corner_indices, corners) = part1::find_corners(&tiles);
    corner_indices.sort_unstable();
    corner_indices.reverse();
    corner_indices.iter().for_each(|&idx| {
        tiles.swap_remove(idx);
    });

    // Solve part 1 by multiplying together the corner IDs
    let part1 = corners
        .iter()
        .map(|corner| corner.id as usize)
        .product::<usize>();

    let shape = fill_image(tiles, corners);

    (part1, part2::solve_part2(shape))
}
