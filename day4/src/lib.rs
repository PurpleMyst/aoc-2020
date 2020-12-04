use itertools::Itertools as _;

fn between(x: &str, a: u64, b: u64) -> bool {
    let x: u64 = x.parse().unwrap();
    x >= a && x <= b
}

fn check_hgt(hgt: &str) -> bool {
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

fn check_ecl(ecl: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .any(|&color| color == ecl)
}

fn check_pid(pid: &str) -> bool {
    pid.len() == 9 && pid.bytes().all(|b| b.is_ascii_digit())
}

fn check_hcl(hcl: &str) -> bool {
    let mut bs = hcl.bytes();
    bs.next() == Some(b'#') && bs.all(|b| b.is_ascii_hexdigit())
}

pub fn solve() -> (usize, usize) {
    let mut part2 = 0;

    let part1 = include_str!("input.txt")
        .trim()
        .split("\r\n\r\n")
        .filter_map(|passport| {
            let mut byr = None;
            let mut iyr = None;
            let mut eyr = None;
            let mut hgt = None;
            let mut ecl = None;
            let mut pid = None;
            let mut hcl = None;

            passport
                .split_ascii_whitespace()
                .map(|pair| pair.splitn(2, ':').collect_tuple().unwrap())
                .for_each(|(field, value)| match field {
                    "byr" => byr = Some(value),
                    "iyr" => iyr = Some(value),
                    "eyr" => eyr = Some(value),
                    "hgt" => hgt = Some(value),
                    "ecl" => ecl = Some(value),
                    "hcl" => hcl = Some(value),
                    "pid" => pid = Some(value),
                    "cid" => {}
                    _ => unreachable!(),
                });

            let byr = byr?;
            let iyr = iyr?;
            let eyr = eyr?;
            let hgt = hgt?;
            let ecl = ecl?;
            let pid = pid?;
            let hcl = hcl?;

            if between(byr, 1920, 2002)
                && between(iyr, 2010, 2020)
                && between(eyr, 2020, 2030)
                && check_hgt(hgt)
                && check_hcl(hcl)
                && check_ecl(ecl)
                && check_pid(pid)
            {
                part2 += 1;
            }

            Some(())
        })
        .count();

    (part1, part2)
}
