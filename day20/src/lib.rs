use array_iterator::ArrayIterator;
use im::Vector as ImVec;

pub(crate) const TOP: usize = 0;
pub(crate) const LEFT: usize = 1;
pub(crate) const RIGHT: usize = 2;
pub(crate) const BOTTOM: usize = 3;

const IMAGE_SIDE: usize = 12;

mod tile;
use tile::Tile;

mod side_filler;

mod part1;
mod part2;

#[inline]
pub fn solve() -> (u64, u32) {
    // POSSIBLE OPTIMIZATION:
    // We know we're going to have a 12x12 image in the
    // end, we could probably optimize to just a [usize; 12*12]
    let tiles = include_str!("input.txt")
        .split("\n\n")
        .take_while(|line| !line.is_empty())
        .map(Tile::from_input)
        .collect::<ImVec<Tile>>();

    let shape = part1::find_image(tiles);

    let part1 = shape[0].id as u64
        * shape[IMAGE_SIDE - 1].id as u64
        * shape[(IMAGE_SIDE - 1) * IMAGE_SIDE].id as u64
        * shape[IMAGE_SIDE * IMAGE_SIDE - 1].id as u64;

    let interior = part2::load_interiors(&shape);

    let part2 = ArrayIterator::new(part2::possible_transformations(interior))
        .find_map(|interior| {
            let monsters = part2::count_seamonsters(interior);

            if monsters != 0 {
                let part2 = interior.iter().map(|row| row.count_ones()).sum::<u32>()
                    - monsters as u32 * part2::SEAMONSTER_AREA;
                Some(part2)
            } else {
                None
            }
        })
        .unwrap();

    (part1, part2)
}
