enum Op {
    Add,
    Mul,
}

struct SimpleParser {
    line: &'static [u8],
}

impl SimpleParser {
    fn new(line: &'static str) -> Self {
        Self {
            line: line.as_bytes(),
        }
    }

    fn skip(&mut self, n: usize) {
        self.line = &self.line[n.min(self.line.len())..];
    }

    fn num(&mut self) -> u64 {
        let mut n = 0;

        loop {
            let ch = match self.line.first() {
                Some(&ch) => ch,
                None => break n,
            };

            if ch >= b'0' && ch <= b'9' {
                n = 10 * n + u64::from(ch - b'0');
                self.skip(1);
            } else {
                break n;
            }
        }
    }

    fn atom(&mut self) -> u64 {
        if self.line[0] == b'(' {
            self.skip(1);
            self.expr()
        } else {
            self.num()
        }
    }

    fn op(&mut self) -> Op {
        self.skip(1);
        let op = match self.line[0] {
            b'+' => Op::Add,
            b'*' => Op::Mul,
            _ => unreachable!(),
        };
        self.skip(2);
        op
    }

    fn expr(&mut self) -> u64 {
        let mut acc = self.atom();

        loop {
            if self.line.is_empty() || self.line[0] == b')' {
                self.skip(1);
                break acc;
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
    line: &'static [u8],
}

impl AdvancedParser {
    fn new(line: &'static str) -> Self {
        Self {
            line: line.as_bytes(),
        }
    }

    fn skip(&mut self, n: usize) {
        self.line = &self.line[n.min(self.line.len())..];
    }

    fn num(&mut self) -> u64 {
        let mut n = 0;

        loop {
            let ch = match self.line.first() {
                Some(&ch) => ch,
                None => break n,
            };

            if ch >= b'0' && ch <= b'9' {
                n = 10 * n + u64::from(ch - b'0');
                self.skip(1);
            } else {
                break n;
            }
        }
    }

    fn atom(&mut self) -> u64 {
        if self.line[0] == b'(' {
            self.skip(1);
            self.expr(true)
        } else {
            self.num()
        }
    }

    fn op(&mut self) -> Op {
        self.skip(1);
        let op = match self.line[0] {
            b'+' => Op::Add,
            b'*' => Op::Mul,
            _ => unreachable!(),
        };
        self.skip(2);
        op
    }

    fn expr(&mut self, eat: bool) -> u64 {
        let mut acc = self.atom();

        loop {
            if self.line.is_empty() || self.line[0] == b')' {
                if eat {
                    self.skip(1);
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
    let mut part1 = 0;
    let mut part2 = 0;

    for line in include_str!("input.txt").lines() {
        part1 += SimpleParser::new(line).expr();
        part2 += AdvancedParser::new(line).expr(true);
    }

    (part1, part2)
}
