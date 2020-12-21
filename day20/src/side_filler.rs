use array_iterator::ArrayIterator;
use im::Vector as ImVec;

use crate::tile::Tile;
use crate::{State, BOTTOM, LEFT, RIGHT, TOP};

pub(crate) trait SideFiller {
    fn initial_parent(width: usize, height: usize) -> usize;
    fn fits(parent: Tile, previous: Option<Tile>, candidate: &Tile) -> bool;
    fn next_state(
        previous_state: &State,
        width: usize,
        height: usize,
        candidate_idx: usize,
        candidate: Tile,
    ) -> State;
    fn should_continue(width: usize, height: usize, parent_pos: usize) -> bool;
}

pub(crate) struct RightSideFiller;

impl SideFiller for RightSideFiller {
    fn initial_parent(width: usize, height: usize) -> usize {
        width * height - 1
    }

    fn fits(parent: Tile, previous: Option<Tile>, candidate: &Tile) -> bool {
        candidate.edges[LEFT] == parent.edges[RIGHT]
            && previous.map_or(true, |previous| {
                previous.edges[TOP] == candidate.edges[BOTTOM]
            })
    }

    fn next_state(
        previous_state: &State,
        width: usize,
        _height: usize,
        candidate_idx: usize,
        candidate: Tile,
    ) -> State {
        let mut remaining = previous_state.remaining.clone();
        remaining.remove(candidate_idx);

        // Insert the candidate into its proper position
        let mut next_shape = previous_state.next_shape.clone();
        next_shape.insert(previous_state.parent_pos + 1, candidate);

        State {
            remaining,
            next_shape,
            parent_pos: previous_state.parent_pos.saturating_sub(width),
            previous: Some(candidate),
        }
    }

    fn should_continue(width: usize, _height: usize, parent_pos: usize) -> bool {
        parent_pos > width - 1
    }
}

pub(crate) struct BottomSideFiller;

impl SideFiller for BottomSideFiller {
    fn initial_parent(width: usize, height: usize) -> usize {
        width * (height - 1)
    }

    fn fits(parent: Tile, previous: Option<Tile>, candidate: &Tile) -> bool {
        candidate.edges[TOP] == parent.edges[BOTTOM]
            && previous.map_or(true, |previous| {
                previous.edges[RIGHT] == candidate.edges[LEFT]
            })
    }

    fn next_state(
        previous_state: &State,
        _width: usize,
        _height: usize,
        candidate_idx: usize,
        candidate: Tile,
    ) -> State {
        // The tile we used is not "remaining" anymore
        let mut remaining = previous_state.remaining.clone();
        remaining.remove(candidate_idx);

        // Insert the candidate into its proper position
        let mut next_shape = previous_state.next_shape.clone();
        next_shape.push_back(candidate);

        State {
            remaining,
            next_shape,
            parent_pos: previous_state.parent_pos + 1,
            previous: Some(candidate),
        }
    }

    fn should_continue(width: usize, height: usize, parent_pos: usize) -> bool {
        parent_pos < (width * height) - 1
    }
}

pub(crate) fn fill_side<SF: SideFiller>(
    remaining: ImVec<Tile>,
    shape: ImVec<Tile>,

    width: usize,
    height: usize,
) -> Vec<State> {
    // States left to explore
    let mut states = vec![State {
        remaining,
        next_shape: shape.clone(),
        parent_pos: SF::initial_parent(width, height),
        previous: None,
    }];

    // States which've fulfilled the requirement of filling the right side
    let mut result = Vec::new();

    while let Some(state) = states.pop() {
        // Which tile do we need to fit with?
        let parent = shape[state.parent_pos];

        // Out of all remaining tiles, find all the possible transformed tiles which fit in the position we want to insert into
        let candidates =
            state
                .remaining
                .iter()
                .copied()
                .enumerate()
                .flat_map(|(candidate_idx, candidate)| {
                    ArrayIterator::new(candidate.possible_transformations())
                        .filter(|candidate| {
                            // Does this fit with the parent?
                            SF::fits(parent, state.previous, candidate)
                        })
                        .map(move |candidate| (candidate_idx, candidate))
                });

        // For each candidate, calculate its state
        let next_states = candidates.map(|(candidate_idx, candidate)| {
            SF::next_state(&state, width, height, candidate_idx, candidate)
        });

        if SF::should_continue(width, height, state.parent_pos) {
            // If we've still got stuff left to place, put it in the states set
            states.extend(next_states);
        } else {
            // Otherwise, put it in the result set
            result.extend(next_states);
        }
    }

    result
}
