enum Op {
    Add,
    Mul,
}

struct SimpleParser {
    line: &'static str,
}

impl SimpleParser {
    fn new(line: &'static str) -> Self {
        Self { line }
    }

    fn skip_one(&mut self) {
        if !self.line.is_empty() {
            self.line = &self.line[1..];
        }
    }

    fn num(&mut self) -> u64 {
        let idx = self
            .line
            .find(|ch: char| !ch.is_numeric())
            .unwrap_or(self.line.len());

        let (n, nline) = self.line.split_at(idx);
        self.line = nline;

        n.parse().unwrap()
    }

    fn atom(&mut self) -> u64 {
        if self.line.starts_with('(') {
            self.skip_one();
            self.expr()
        } else {
            self.num()
        }
    }

    fn op(&mut self) -> Op {
        self.skip_one();
        let op = if self.line.starts_with('+') {
            Op::Add
        } else if self.line.starts_with('*') {
            Op::Mul
        } else {
            unreachable!()
        };
        self.skip_one();
        self.skip_one();
        op
    }

    fn expr(&mut self) -> u64 {
        let mut acc = self.atom();

        loop {
            if self.line.starts_with(')') || self.line.is_empty() {
                self.skip_one();
                return acc;
            }

            let op = self.op();
            let rhs = self.atom();

            match op {
                Op::Add => acc += rhs,
                Op::Mul => acc *= rhs,
            }
        }
    }
}

struct AdvancedParser {
    line: &'static str,
}

impl AdvancedParser {
    fn new(line: &'static str) -> Self {
        Self { line }
    }

    fn skip_one(&mut self) {
        if !self.line.is_empty() {
            self.line = &self.line[1..];
        }
    }

    fn num(&mut self) -> u64 {
        let idx = self
            .line
            .find(|ch: char| !ch.is_numeric())
            .unwrap_or(self.line.len());

        let (n, nline) = self.line.split_at(idx);
        self.line = nline;

        n.parse().unwrap()
    }

    fn atom(&mut self) -> u64 {
        if self.line.starts_with('(') {
            self.skip_one();
            self.expr(true)
        } else {
            self.num()
        }
    }

    fn op(&mut self) -> Op {
        self.skip_one();
        let op = if self.line.starts_with('+') {
            Op::Add
        } else if self.line.starts_with('*') {
            Op::Mul
        } else {
            unreachable!()
        };
        self.skip_one();
        self.skip_one();
        op
    }

    fn expr(&mut self, eat: bool) -> u64 {
        let mut acc = self.atom();

        loop {
            if self.line.starts_with(')') || self.line.is_empty() {
                if eat {
                    self.skip_one();
                }
                break acc;
            }

            let op = self.op();

            match op {
                Op::Add => {
                    acc += self.atom();
                }

                Op::Mul => {
                    acc *= self.expr(false);
                }
            }
        }
    }
}

#[inline]
pub fn solve() -> (u64, u64) {
    let part1 = include_str!("input.txt")
        .lines()
        .map(|line| SimpleParser::new(line).expr())
        .sum::<u64>();

    let part2 = include_str!("input.txt")
        .lines()
        .map(|line| AdvancedParser::new(line).expr(true))
        .sum::<u64>();

    (part1, part2)
}
