const MODULUS: u64 = 2020_12_27;

fn transform(loop_size: u64, subject: u64) -> u64 {
    (0..loop_size).fold(1, |acc, _| (acc * subject) % MODULUS)
}

fn powmod(x: u64, mut y: u64) -> u64 {
    let mut t = 1;
    let mut tmp = x % MODULUS;
    while y > 0 {
        if y & 1 > 0 {
            t = t * tmp % MODULUS;
        }

        tmp = (tmp * tmp) % MODULUS;
        y = y >> 1;
    }
    return t;
}

#[inline]
pub fn solve() -> u64 {
    let (card_pubkey, door_pubkey) = {
        let mut it = include_str!("input.txt")
            .lines()
            .map(|n| n.parse::<u64>().unwrap());
        (it.next().unwrap(), it.next().unwrap())
    };

    let card_loopsize = (0..MODULUS).find(|&n| powmod(7, n) == card_pubkey).unwrap();

    transform(card_loopsize, door_pubkey)
}
