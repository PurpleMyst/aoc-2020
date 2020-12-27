use std::fmt::Display;

use static_assert_macro::static_assert;

// TODO: optimize rotate_ccw and flip_vertically

const ON: u8 = b'#';
pub(crate) const TILE_SIDE: usize = 10;

const TOP: usize = 0;
const LEFT: usize = 1;
const RIGHT: usize = 2;
const BOTTOM: usize = 3;

// Requirement to be able to use exactly an u8 to store interior
static_assert!(8 == TILE_SIDE - 2);

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub id: u16,
    pub edges: [u16; 4],
    pub interior: [u8; TILE_SIDE - 2],
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.edges == other.edges
    }
}

impl Eq for Tile {}

/// Reverse the bits of a 10-bit unsigned integer
fn reverse_10bits(x: u16) -> u16 {
    x.reverse_bits() >> (16 - 10)
}

impl Tile {
    pub fn from_input(input: &str) -> Self {
        let mut lines = input.lines();
        let id = lines.next().unwrap();
        let id = id["Tile ".len()..id.len() - ":".len()].parse().unwrap();

        let mut line = lines.next().unwrap().as_bytes();

        let top = line
            .iter()
            .fold(0, |acc, &ch| (acc << 1) | (ch == ON) as u16);
        let mut left = 0;
        let mut right = 0;

        let mut interior = [0u8; TILE_SIDE - 2];
        let mut it = interior.iter_mut();
        static_assert!(TILE_SIDE - 2 <= 8);

        loop {
            left = (left << 1) | (line[0] == ON) as u16;
            right = (right << 1) | (line[TILE_SIDE - 1] == ON) as u16;

            if let Some(next_line) = lines.next() {
                line = next_line.as_bytes();
            } else {
                break;
            }

            if let Some(shite) = it.next() {
                *shite = line[1..TILE_SIDE - 1]
                    .iter()
                    .fold(0, |acc, b| (acc << 1) | (*b == ON) as u8);
            }
        }

        let bottom = line
            .iter()
            .fold(0, |acc, &ch| (acc << 1) | (ch == ON) as u16);

        Self {
            id,
            edges: [top, left, right, bottom],
            interior,
        }
    }

    pub(crate) fn flip_horizontally(self) -> Self {
        let Self {
            id,
            edges: [top, left, right, bottom],
            mut interior,
        } = self;

        interior
            .iter_mut()
            .for_each(|elem| *elem = elem.reverse_bits());

        Self {
            id,
            edges: [reverse_10bits(top), right, left, reverse_10bits(bottom)],
            interior,
        }
    }

    pub fn flip_vertically(self) -> Self {
        self.rotate_cw().flip_horizontally().rotate_ccw()
    }

    pub(crate) fn rotate_cw(self) -> Self {
        let Self {
            id,
            edges: [top, left, right, bottom],
            mut interior,
        } = self;

        macro_rules! set_bit {
            ($matrix:ident, $dest_row:expr, $dest_col:expr, $src_row: expr, $src_col: expr$(,)?) => {
                let dest_idx = 8 - ($dest_col + 1);
                $matrix[$dest_row] &= !(1 << dest_idx);
                let src_idx = 8 - ($src_col + 1);
                let src_bit = ($src_row & (1 << src_idx)) >> src_idx;
                $matrix[$dest_row] |= src_bit << dest_idx;
            };
        }

        for i in 0..(TILE_SIDE - 2) / 2 {
            for j in i..(TILE_SIDE - 2) - i - 1 {
                let temp = interior[i];

                set_bit!(interior, i, j, interior[TILE_SIDE - 2 - 1 - j], i);
                set_bit!(
                    interior,
                    TILE_SIDE - 2 - 1 - j,
                    i,
                    interior[TILE_SIDE - 2 - 1 - i],
                    TILE_SIDE - 2 - 1 - j,
                );
                set_bit!(
                    interior,
                    TILE_SIDE - 2 - 1 - i,
                    TILE_SIDE - 2 - 1 - j,
                    interior[j],
                    TILE_SIDE - 2 - 1 - i,
                );
                set_bit!(interior, j, TILE_SIDE - 2 - 1 - i, temp, j);
            }
        }

        Self {
            id,
            edges: [reverse_10bits(left), bottom, top, reverse_10bits(right)],
            interior,
        }
    }

    pub(crate) fn rotate_ccw(self) -> Self {
        self.rotate_cw().rotate_cw().rotate_cw()
    }

    pub fn could_fit(&self, other: &Self) -> bool {
        other
            .edges
            .iter()
            .any(|&edge| self.edges.contains(&edge) || self.edges.contains(&reverse_10bits(edge)))
    }

    /// Rotate the given tile so that it fits on the right
    pub fn fit_right(&self, other: Self) -> Option<Self> {
        let edge_value = self.edges[RIGHT];

        // our right edge must line with their left edge
        if let Some(edge) = other.edges.iter().position(|&e| e == edge_value) {
            Some(match edge {
                TOP => other.rotate_ccw().flip_vertically(),
                LEFT => other,
                RIGHT => other.flip_horizontally(),
                BOTTOM => other.rotate_cw(),

                _ => unreachable!(),
            })
        } else if let Some(edge) = other
            .edges
            .iter()
            .position(|&e| reverse_10bits(e) == edge_value)
        {
            Some(match edge {
                TOP => other.rotate_ccw(),
                LEFT => other.flip_vertically(),
                RIGHT => other.flip_horizontally().flip_vertically(),
                BOTTOM => other.rotate_cw().flip_vertically(),

                _ => unreachable!(),
            })
        } else {
            None
        }
    }

    /// Rotate the given tile so that it fits on the bottom
    pub fn fit_down(&self, other: Self) -> Option<Self> {
        let edge_value = self.edges[BOTTOM];

        // our bottom edge must line with their top edge
        if let Some(edge) = other.edges.iter().position(|&e| e == edge_value) {
            Some(match edge {
                TOP => other,
                LEFT => other.rotate_cw().flip_horizontally(),
                RIGHT => other.rotate_ccw(),
                BOTTOM => other.flip_vertically(),

                _ => unreachable!(),
            })
        } else if let Some(edge) = other
            .edges
            .iter()
            .position(|&e| reverse_10bits(e) == edge_value)
        {
            Some(match edge {
                TOP => other.flip_horizontally(),
                LEFT => other.rotate_cw(),
                RIGHT => other.rotate_ccw().flip_horizontally(),
                BOTTOM => other.flip_vertically().flip_horizontally(),

                _ => unreachable!(),
            })
        } else {
            None
        }
    }

    pub fn possible_transformations(self) -> [Self; 8] {
        [
            self,
            self.rotate_cw(),
            self.rotate_cw().rotate_cw(),
            self.rotate_cw().rotate_cw().rotate_cw(),
            self.flip_horizontally(),
            self.flip_horizontally().rotate_cw(),
            self.flip_horizontally().rotate_cw().rotate_cw(),
            self.flip_horizontally().rotate_cw().rotate_cw().rotate_cw(),
        ]
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;

        writeln!(
            f,
            "{}",
            format!("{:010b}", self.edges[0])
                .replace("0", ".")
                .replace("1", "#")
        )?;

        for i in (1..9).rev() {
            let l = if self.edges[1] & (1 << i) != 0 {
                "#"
            } else {
                "."
            };
            write!(f, "{}", l)?;

            let interior = self.interior[i - 1];
            for j in (0..8).rev() {
                let ch = if interior & (1 << j) != 0 { "#" } else { "." };
                write!(f, "{}", ch)?;
            }

            let r = if self.edges[2] & (1 << i) != 0 {
                "#"
            } else {
                "."
            };
            write!(f, "{}", r)?;

            writeln!(f)?;
        }

        writeln!(
            f,
            "{}",
            format!("{:010b}", self.edges[3])
                .replace("0", ".")
                .replace("1", "#")
        )?;

        Ok(())
    }
}
