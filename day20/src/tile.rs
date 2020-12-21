use std::fmt::Display;

const ON: u8 = b'#';
const TILE_SIDE: usize = 10;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Tile {
    pub(crate) id: u16,
    pub(crate) edges: [u16; 4],
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

        loop {
            left = (left << 1) | (line[0] == ON) as u16;
            right = (right << 1) | (line[TILE_SIDE - 1] == ON) as u16;

            if let Some(next_line) = lines.next() {
                line = next_line.as_bytes();
            } else {
                break;
            }
        }

        let bottom = line
            .iter()
            .fold(0, |acc, &ch| (acc << 1) | (ch == ON) as u16);

        Self {
            id,
            edges: [top, left, right, bottom],
        }
    }

    pub(crate) fn flip_horizontally(self) -> Self {
        let Self {
            id,
            edges: [top, left, right, bottom],
        } = self;
        Self {
            id,
            edges: [
                top.reverse_bits() >> (16 - 10),
                right,
                left,
                bottom.reverse_bits() >> (16 - 10),
            ],
        }
    }

    // pub(crate) fn flip_vertically(self) -> Self {
    //     let Self {
    //         id,
    //         edges: [top, left, right, bottom],
    //     } = self;
    //     Self {
    //         id,
    //         edges: [bottom, left, right, top],
    //     }
    // }

    pub(crate) fn rotate_cw(self) -> Self {
        let Self {
            id,
            edges: [top, left, right, bottom],
        } = self;
        Self {
            id,
            // FIXME: reverse bits as a 10bit integer
            edges: [
                left.reverse_bits() >> (16 - 10),
                bottom,
                top,
                right.reverse_bits() >> (16 - 10),
            ],
        }
    }

    // pub(crate) fn rotate_ccw(self) -> Self {
    //     let Self {
    //         id,
    //         edges: [top, left, right, bottom],
    //     } = self;
    //     Self {
    //         id,
    //         edges: [right, top, bottom, left],
    //     }
    // }

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
                // .replace("1", "\x1b[31m#\x1b[0m")
                .replace("1", "#")
        )?;

        for i in (1..9).rev() {
            let l = if self.edges[1] & (1 << i) != 0 {
                // "\x1b[31m#\x1b[0m"
                "#"
            } else {
                "."
            };
            let r = if self.edges[2] & (1 << i) != 0 {
                // "\x1b[31m#\x1b[0m"
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
                // .replace("1", "\x1b[31m#\x1b[0m")
                .replace("1", "#")
        )?;

        Ok(())
    }
}
