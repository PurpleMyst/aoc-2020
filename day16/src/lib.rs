#[derive(Debug, Clone, Copy)]
struct Field {
    name: &'static str,
    left: (u16, u16),
    right: (u16, u16),
}

impl Field {
    fn is_valid(&self, value: u16) -> bool {
        (value >= self.left.0 && value <= self.left.1)
            || (value >= self.right.0 && value <= self.right.1)
    }

    fn from_input(line: &'static str) -> Self {
        let mut sides = line.splitn(2, ": ");
        let name = sides.next().unwrap();

        let mut ranges = sides.next().unwrap().splitn(2, " or ");

        let range = |r: &str| {
            let mut r = r.splitn(2, '-');
            (
                r.next().unwrap().parse().unwrap(),
                r.next().unwrap().parse().unwrap(),
            )
        };

        let left = range(ranges.next().unwrap());
        let right = range(ranges.next().unwrap());

        Self { name, left, right }
    }
}

#[derive(Clone, Copy)]
struct FieldPossibilities<'a> {
    fields: &'a [Field],
    mask: u32,
}

impl<'a> FieldPossibilities<'a> {
    fn new(fields: &'a [Field]) -> Self {
        Self {
            mask: (1 << fields.len()) - 1,
            fields,
        }
    }

    fn restrict(&mut self, value: u16) {
        self.fields.iter().enumerate().for_each(|(idx, field)| {
            if !field.is_valid(value) {
                self.mask &= !(1 << idx);
            }
        })
    }
}

#[inline]
pub fn solve() -> (u16, u64) {
    let mut sections = include_str!("input.txt").split("\n\n");

    let fields = sections
        .next()
        .unwrap()
        .lines()
        .map(Field::from_input)
        .collect::<Vec<_>>();

    let mut field_possibilities = vec![FieldPossibilities::new(&fields); fields.len()];

    let my_ticket = sections
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u16>().unwrap())
        .collect::<Vec<_>>();

    let nearby_tickets = sections.next().unwrap().lines().skip(1);

    let mut part1 = 0;

    // For each nearby ticket...
    nearby_tickets.for_each(|ticket| {
        // Separate out its fields
        let ticket_fields = ticket.split(',').map(|n| n.parse().unwrap());

        // Build an iterator that only gives iterates over the completely invalid fields
        let mut invalid_fields = ticket_fields
            .clone()
            .filter(|&n| !fields.iter().any(|field| field.is_valid(n)));

        // If that iterator isn't empty, then calculate its sum, add it to part 1's answer and ignore this ticket for part 2
        if let Some(n) = invalid_fields.next() {
            part1 += n + invalid_fields.sum::<u16>();
            return;
        }

        // Now that we know that this ticket is valid, let's use its fields to restrict the possibilities for each field index
        ticket_fields
            .zip(field_possibilities.iter_mut())
            .for_each(|(ticket_field, field_possibility)| field_possibility.restrict(ticket_field));
    });

    // Take the field possibilities and couple them to their indices, so that we
    // sort them by how many possibilities they have while retaining index
    // information
    let mut field_possibilities = field_possibilities
        .into_iter()
        .enumerate()
        .collect::<Vec<_>>();
    field_possibilities.sort_by_key(|(_, x)| x.mask.count_ones());

    // This mask represents which fields are still unknown
    let mut unknown_fields = (1 << fields.len()) - 1;

    let mut part2: u64 = 1;

    for (field_ticket_idx, field_possibility) in field_possibilities {
        // Only consider fields for which we don't alerady have a mask
        let mask = field_possibility.mask & unknown_fields;

        // If we did everything correctly, this field index only has one
        // possibilitty, so XOR it with the unknown fields to mark it as known
        debug_assert_eq!(mask.count_ones(), 1);
        unknown_fields ^= mask;

        // Calculate which field this index is by calculating how many trailing
        // zeroes the mask has, since the one's position represents which field
        // this is
        let idx = mask.trailing_zeros() as usize;
        let field = fields[idx];

        if field.name.starts_with("departure") {
            let my_value = my_ticket[field_ticket_idx];
            part2 *= my_value as u64;
        }
    }

    (part1, part2)
}
