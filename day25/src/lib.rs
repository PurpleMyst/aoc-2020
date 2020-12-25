use rustc_hash::FxHashMap as HashMap;

const BASE: u64 = 7;
const MODULUS: u64 = 2020_12_27;

fn transform(loop_size: u64, subject: u64) -> u64 {
    powmod(subject, loop_size)
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

fn babystep_giantstep(pubkey: u64) -> Option<u64> {
    let m = (MODULUS as f64).sqrt().ceil() as u64;

    let mut table = HashMap::<u64, u64>::with_capacity_and_hasher(m as usize, Default::default());
    let mut e = 1;

    for i in 0..m {
        table.insert(e, i);
        e = (e * BASE) % MODULUS;
    }

    let factor = powmod(BASE, MODULUS - m - 1);
    e = pubkey;
    for i in 0..m {
        if let Some(x) = table.get(&e) {
            return Some(i * m + x);
        }
        e = (e * factor) % MODULUS;
    }
    None
}

#[inline]
pub fn solve() -> u64 {
    let (card_pubkey, door_pubkey) = {
        let mut it = include_str!("input.txt")
            .lines()
            .map(|n| n.parse::<u64>().unwrap());
        (it.next().unwrap(), it.next().unwrap())
    };

    let card_loopsize = babystep_giantstep(card_pubkey).unwrap();
    transform(card_loopsize, door_pubkey)
}
