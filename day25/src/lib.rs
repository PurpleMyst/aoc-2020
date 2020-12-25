const MODULUS: u64 = 2020_12_27;

fn transform(loop_size: u64, subject: u64) -> u64 {
    (0..loop_size).fold(1, |acc, _| (acc * subject) % MODULUS)
}

#[inline]
pub fn solve() -> u64 {
    let (card_pubkey, door_pubkey) = {
        let mut it = include_str!("input.txt")
            .lines()
            .map(|n| n.parse::<u64>().unwrap());
        (it.next().unwrap(), it.next().unwrap())
    };

    let (card_loopsize, _) = (2..MODULUS)
        .scan(7, |v, n| {
            *v = (*v * 7) % MODULUS;
            Some((n, *v))
        })
        .find(|&(_, v)| v == card_pubkey)
        .unwrap();

    transform(card_loopsize, door_pubkey)
}
