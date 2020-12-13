use ring_algorithm::chinese_remainder_theorem;

/// Return the earliest time we can catch a bus after the earliest time we can
/// catch *any* bus
///
/// This boils down to solving the equation:
/// n * bus_id >= earliest
/// n >= earliest / bus_id
/// n = ceil(earliest / bus_id)
fn can_catch_at(earliest: usize, bus_id: usize) -> usize {
    bus_id * ((earliest + (bus_id - 1)) / bus_id)
}

/// Solve part 1 by finding the bus with the minimum earliest-catchable-time.
/// Pretty straightforward
#[inline]
pub fn solve_part1<I>(earliest: usize, buses: I) -> usize
where
    I: IntoIterator<Item = usize>,
{
    let bus_id = buses
        .into_iter()
        .min_by_key(|&bus_id| can_catch_at(earliest, bus_id))
        .unwrap();

    let waiting = can_catch_at(earliest, bus_id) - earliest;
    bus_id * waiting
}

/// Solve part 2 by modelling the requirement as a system of modular equations
/// and solving it by using the CRT
///
/// Given a bus id k at offset i, and supposing our solution to the problem is a time t, then
/// to satisfy the requirement the following relation must hold:
/// (t + i) mod k = 0
/// Which can be arranged to the form
/// t mod k = -i
/// A system of this kind of equation can be solved by utilizing the aforementioned chinese remainder theorem
#[inline]
pub fn solve_part2(buses: &[(usize, usize)]) -> usize {
    let (rhs, moduli): (Vec<_>, Vec<_>) = buses
        .iter()
        .map(|&(i, bus_id)| (-(i as isize), bus_id as isize))
        .unzip();

    chinese_remainder_theorem(&rhs, &moduli).unwrap() as usize
}

#[inline]
pub fn parse_input() -> (usize, Vec<(usize, usize)>) {
    let mut input = include_str!("input.txt").lines();
    let earliest = input.next().unwrap().parse::<usize>().unwrap();
    let buses = input
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|&(_, bus_id)| bus_id != "x")
        .map(|(i, bus_id)| (i, bus_id.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();
    (earliest, buses)
}

#[inline]
pub fn solve() -> (usize, usize) {
    let (earliest, buses) = parse_input();
    let part1 = solve_part1(earliest, buses.iter().map(|&(_, bus_id)| bus_id));
    let part2 = solve_part2(&buses);

    (part1, part2)
}
