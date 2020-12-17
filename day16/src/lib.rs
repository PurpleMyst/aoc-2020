const FIELDS: usize = 20;
const MAX_FIELD_VALUE: usize = 1000;

#[derive(Debug, Clone, Copy)]
struct FieldRange {
    name: &'static str,
    left: (u16, u16),
    right: (u16, u16),
}

impl Default for FieldRange {
    fn default() -> Self {
        Self {
            name: "",
            left: (0, 0),
            right: (0, 0),
        }
    }
}

impl FieldRange {
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

fn parse_ticket(s: &'static str) -> [u16; FIELDS] {
    let mut ticket = [0; FIELDS];
    s.split(',')
        .map(|n| n.parse::<u16>().unwrap())
        .zip(ticket.iter_mut())
        .for_each(|(val, elem)| *elem = val);
    ticket
}

#[inline]
pub fn solve() -> (u16, u64) {
    let mut sections = include_str!("input.txt").split("\n\n");

    // Parse the individual field ranges
    let mut field_ranges = [FieldRange::default(); FIELDS];
    sections
        .next()
        .unwrap()
        .lines()
        .map(FieldRange::from_input)
        .zip(field_ranges.iter_mut())
        .for_each(|(val, elem)| *elem = val);

    // Calculate a lookup table for each value to a bitmask of the field ranges it could correspond to
    let mut field_ranges_lut = [0u32; MAX_FIELD_VALUE];
    field_ranges_lut
        .iter_mut()
        .enumerate()
        .for_each(|(value, mask)| {
            *mask = field_ranges
                .iter()
                .enumerate()
                .fold(0, |mask, (i, field_range)| {
                    mask | ((field_range.is_valid(value as _) as u32) << i)
                });
        });

    // Parse our ticket
    let my_ticket = parse_ticket(sections.next().unwrap().lines().nth(1).unwrap());

    // Make an iterator of nearby ticket
    let nearby_tickets = sections.next().unwrap().lines().skip(1).map(parse_ticket);

    // Initialize field possibility bitmasks, each field index could be any of the field ranges
    let mut field_possibilities = [(0, (1 << FIELDS) - 1); FIELDS];
    field_possibilities
        .iter_mut()
        .enumerate()
        .for_each(|(idx, (i, _))| *i = idx);

    let mut part1 = 0;

    // For each nearby ticket...
    nearby_tickets.for_each(|ticket| {
        // Sum its invalid fields
        let mut invalid_fields = ticket
            .iter()
            .filter(|&&field_value| field_ranges_lut[field_value as usize] == 0)
            .peekable();

        if invalid_fields.peek().is_some() {
            part1 += invalid_fields.sum::<u16>();
            return;
        }

        // For each field value, AND the bitmasks representing the possible
        // field ranges it fits into with all the previous ones
        ticket.iter().zip(field_possibilities.iter_mut()).for_each(
            |(&field_value, (_, field_possibility))| {
                *field_possibility &= field_ranges_lut[field_value as usize]
            },
        );
    });

    // Sort our field possibility bitmasks by how many ones they have
    field_possibilities.sort_unstable_by_key(|(_, mask)| mask.count_ones());

    let mut unknown_fields = (1 << FIELDS) - 1;

    let mut part2: u64 = 1;

    for &(field_ticket_idx, mask) in field_possibilities.iter() {
        // Only consider fields for which we don't alerady have a mask
        let mask = mask & unknown_fields;

        // If we did everything correctly, this field index only has one
        // possibilitty, so XOR it with the unknown fields to mark it as known
        debug_assert_eq!(mask.count_ones(), 1);
        unknown_fields ^= mask;

        // Calculate which field range this field index corresponds to by
        // calculating how many trailing zeroes the mask has, since the one's
        // position represents which field range this is
        let field_range_idx = mask.trailing_zeros() as usize;

        // In my input, the first six fields all start with departure
        if field_range_idx < 6 {
            debug_assert!(field_ranges[field_range_idx].name.starts_with("departure"));
            let my_value = my_ticket[field_ticket_idx];
            part2 *= my_value as u64;
        }
    }

    (part1, part2)
}
