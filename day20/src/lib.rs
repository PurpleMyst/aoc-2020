use array_iterator::ArrayIterator;
use im::Vector as ImVec;

pub(crate) const TOP: usize = 0;
pub(crate) const LEFT: usize = 1;
pub(crate) const RIGHT: usize = 2;
pub(crate) const BOTTOM: usize = 3;

mod tile;
use tile::Tile;

mod side_filler;
use side_filler::*;

#[derive(Debug)]
pub(crate) struct State {
    remaining: ImVec<Tile>,
    next_shape: ImVec<Tile>,
    parent_pos: usize,
    previous: Option<Tile>,
}

/// Fill out a given state to form the next biggest square
fn next_squares(
    remaining: ImVec<Tile>,
    shape: ImVec<Tile>,
    side: usize,
) -> impl Iterator<Item = State> {
    assert_eq!(shape.len(), side * side);
    let rs = fill_side::<RightSideFiller>(remaining, shape, side, side);

    rs.into_iter().flat_map(move |state| {
        fill_side::<BottomSideFiller>(state.remaining, state.next_shape, side + 1, side)
    })
}

/// Given a set of tiles, find a square which uses them all and return it
fn find_image(tiles: ImVec<Tile>) -> (usize, ImVec<Tile>) {
    let mut states = tiles
        .iter()
        .enumerate()
        .flat_map(|(idx, &seed)| {
            ArrayIterator::new(seed.possible_transformations()).map(move |seed| (idx, seed))
        })
        .map(|(idx, seed)| {
            let mut remaining = tiles.clone();
            remaining.remove(idx);
            let next_shape = ImVec::unit(seed);
            State {
                remaining,
                next_shape,
                parent_pos: 0,
                previous: None,
            }
        })
        .collect::<Vec<_>>();

    for side in 1.. {
        if states.is_empty() {
            break;
        }

        let (mut finished, next): (Vec<_>, Vec<_>) = states
            .drain(..)
            .flat_map(|state| next_squares(state.remaining, state.next_shape, side))
            .partition(|state| state.remaining.is_empty());

        if let Some(winner) = finished.pop() {
            return (side + 1, winner.next_shape);
        }

        states = next;
    }

    unreachable!()
}

#[inline]
pub fn solve() -> (u64, u64) {
    let tiles = include_str!("input.txt")
        .split("\n\n")
        .take_while(|line| !line.is_empty())
        .map(Tile::from_input)
        .collect::<ImVec<Tile>>();

    let (side, shape) = find_image(tiles);

    let part1 = shape[0].id as u64
        * shape[side - 1].id as u64
        * shape[(side - 1) * side].id as u64
        * shape[side * side - 1].id as u64;

    (part1, 0)
}
