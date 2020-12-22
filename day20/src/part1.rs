use array_iterator::ArrayIterator;
use im::Vector as ImVec;

use crate::side_filler::*;
use crate::tile::Tile;

#[derive(Debug)]
pub(crate) struct State {
    pub(crate) remaining: ImVec<Tile>,
    pub(crate) next_shape: ImVec<Tile>,
    pub(crate) parent_pos: usize,
    pub(crate) previous: Option<Tile>,
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
pub(crate) fn find_image(tiles: ImVec<Tile>) -> ImVec<Tile> {
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
            return winner.next_shape;
        }

        states = next;
    }

    unreachable!()
}
