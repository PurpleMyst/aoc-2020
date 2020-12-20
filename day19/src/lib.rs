use std::{debug_assert_eq, hint::unreachable_unchecked, mem::MaybeUninit};

// POSSIBLE OPTIMIZATION: we're not really iterating over strings, just bits where "a" is 0 and "b" is 1

use arrayvec::ArrayVec;

#[derive(Clone)]
enum Rule {
    Terminator(char),
    Seq(ArrayVec<[u8; 2]>),
    Or(ArrayVec<[u8; 2]>, ArrayVec<[u8; 2]>),
}

type Rules = [Option<Rule>; 0xFF];

fn parse_rule(rule: &'static str) -> Rule {
    let mut parts = rule.split(' ');

    if rule.starts_with('"') {
        debug_assert!(rule.ends_with('"'));
        debug_assert_eq!(rule.len(), 3);
        return Rule::Terminator(rule.chars().nth(1).unwrap());
    }

    let fst: ArrayVec<[u8; 2]> = parts
        .by_ref()
        .take_while(|&part| part != "|")
        .map(|part| part.parse().unwrap())
        .collect();

    let snd: ArrayVec<[u8; 2]> = parts
        .filter(|&part| part != "|")
        .map(|part| part.parse().unwrap())
        .collect();

    if !snd.is_empty() {
        Rule::Or(fst, snd)
    } else {
        Rule::Seq(fst)
    }
}

fn matches_seq<'a>(rules: &Rules, seq: &[u8], line: &'a str) -> Option<&'a str> {
    seq.iter()
        .try_fold(line, |acc, &rule| matches_by_idx(rules, rule, acc))
}

fn matches_by_idx<'a>(rules: &Rules, rule: u8, line: &'a str) -> Option<&'a str> {
    match rules[rule as usize].as_ref() {
        Some(Rule::Terminator(ch)) => {
            if line.starts_with(*ch) {
                Some(&line[ch.len_utf8()..])
            } else {
                None
            }
        }

        Some(Rule::Seq(seq)) => matches_seq(rules, &seq, line),

        Some(Rule::Or(fst, snd)) => {
            matches_seq(rules, &fst, line).or_else(|| matches_seq(rules, &snd, line))
        }

        None => unsafe { unreachable_unchecked() },
    }
}

#[inline]
pub fn solve() -> (usize, usize) {
    // SAFETY: this is literally in the MaybeUninit docs
    let mut rules: [MaybeUninit<Option<Rule>>; 0xFF] =
        unsafe { MaybeUninit::uninit().assume_init() };
    for rule in &mut rules {
        *rule = MaybeUninit::new(None);
    }
    let mut rules = unsafe { std::mem::transmute::<_, Rules>(rules) };

    let mut input = include_str!("input.txt").lines();

    input
        .by_ref()
        .take_while(|l| !l.is_empty())
        .for_each(|line| {
            let (lhs, rhs) = {
                let mut it = line.splitn(2, ": ");
                (it.next().unwrap(), it.next().unwrap())
            };
            let lhs = lhs.parse::<u8>().unwrap();
            let rhs = parse_rule(rhs);
            rules[lhs as usize] = Some(rhs);
        });

    let mut part1 = 0;
    let mut part2 = 0;

    input.for_each(|mut line| {
        // Part 1: Straight up regex match
        if matches_by_idx(&rules, 0, line) == Some("") {
            part1 += 1;
        }

        // Part 2: Count "left parens", Count "right parens", we should have at
        //         least one of each and more left than right
        let mut left = 0;
        while let Some(rest) = matches_by_idx(&rules, 42, line) {
            left += 1;
            line = rest;
        }

        let mut right = 0;
        while let Some(rest) = matches_by_idx(&rules, 31, line) {
            right += 1;
            line = rest;
        }

        if line.is_empty() && left >= 1 && right >= 1 && right < left && (left - right) >= 1 {
            part2 += 1;
        }
    });

    (part1, part2)
}
