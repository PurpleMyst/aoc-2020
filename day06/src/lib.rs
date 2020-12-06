// Calculate a participant's answers as an integer whose set bit positions
// represent each answer's letter as an offset from 'a'
fn answers<I: IntoIterator<Item = u8>>(participant: I) -> u32 {
    participant
        .into_iter()
        .fold(0u32, |prev, ch| prev | (1 << (ch - b'a')))
}

#[inline]
pub fn solve() -> (u32, u32) {
    include_str!("input.txt")
        .split("\n\n")
        .map(|group| {
            let part1 = answers(group.lines().flat_map(|s| s.bytes())).count_ones();

            let part2 = {
                group
                    .lines()
                    .fold(u32::MAX, |common, participant| {
                        common & answers(participant.bytes())
                    })
                    .count_ones()
            };

            (part1, part2)
        })
        .fold((0, 0), |(acc1, acc2), (ans1, ans2)| {
            (acc1 + ans1, acc2 + ans2)
        })
}
