use std::collections::HashMap;

use itertools::Itertools as _;

fn between(x: &str, a: u64, b: u64) -> bool {
    let x: u64 = x.parse().unwrap();
    x >= a && x <= b
}

fn hgt(hgt: &str) -> bool {
    match hgt
        .splitn(2, |ch: char| ch.is_ascii_alphabetic())
        .collect_tuple()
    {
        Some((hgt, un)) => {
            if un.ends_with("m") {
                between(hgt, 150, 193)
            } else {
                between(hgt, 59, 76)
            }
        }

        _ => false,
    }
}

fn ecl(ecl: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .any(|&color| color == ecl)
}

fn pid(pid: &str) -> bool {
    pid.len() == 9 && pid.bytes().all(|b| b.is_ascii_digit())
}

fn hcl(hcl: &str) -> bool {
    let mut bs = hcl.bytes();
    bs.next() == Some(b'#') && bs.all(|b| b.is_ascii_hexdigit())
}

pub fn solve() -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;

    include_str!("input.txt")
        .trim()
        .split("\r\n\r\n")
        .for_each(|passport| {
            let hs = passport
                .split_ascii_whitespace()
                .map(|pair| pair.splitn(2, ':').collect_tuple().unwrap())
                .collect::<HashMap<_, _>>();

            if ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .find(|&n| !hs.contains_key(n))
                .is_some()
            {
                return;
            }

            part1 += 1;

            if between(hs["byr"], 1920, 2002)
                && between(hs["iyr"], 2010, 2020)
                && between(hs["eyr"], 2020, 2030)
                && hgt(hs["hgt"])
                && hcl(hs["hcl"])
                && ecl(hs["ecl"])
                && pid(hs["pid"])
            {
                part2 += 1;
            }
        });

    (part1, part2)
}
