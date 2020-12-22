use std::fmt::Display;

const ON: u8 = b'#';
pub(crate) const TILE_SIDE: usize = 10;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Tile {
    pub(crate) id: u16,
    pub(crate) edges: [u16; 4],
    pub(crate) interior: [u8; TILE_SIDE - 2],
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.edges == other.edges
    }
}

impl Eq for Tile {}

impl Tile {
    pub(crate) fn from_input(input: &str) -> Self {
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
        debug_assert!(TILE_SIDE - 2 <= 8);

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
            edges: [
                top.reverse_bits() >> (16 - 10),
                right,
                left,
                bottom.reverse_bits() >> (16 - 10),
            ],
            interior,
        }
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
            edges: [
                left.reverse_bits() >> (16 - 10),
                bottom,
                top,
                right.reverse_bits() >> (16 - 10),
            ],
            interior,
        }
    }

    pub(crate) fn possible_transformations(self) -> [Self; 8] {
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
            let r = if self.edges[2] & (1 << i) != 0 {
                "#"
            } else {
                "."
            };
            writeln!(f, "{}        {}", l, r)?;
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
