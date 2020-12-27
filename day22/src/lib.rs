use std::collections::{HashSet, VecDeque};

fn play_combat(player1: &mut VecDeque<u8>, player2: &mut VecDeque<u8>) {
    if player1.front() > player2.front() {
        let f = player1.pop_front().unwrap();
        player1.push_back(f);
        player1.push_back(player2.pop_front().unwrap());
    } else {
        let f = player2.pop_front().unwrap();
        player2.push_back(f);
        player2.push_back(player1.pop_front().unwrap());
    }
}

fn load_deck(deck: &str) -> VecDeque<u8> {
    deck.lines()
        .skip(1)
        .map(|n| n.parse::<u8>().unwrap())
        .collect()
}

pub fn load_input() -> (VecDeque<u8>, VecDeque<u8>) {
    let mut decks = include_str!("input.txt").split("\n\n").map(load_deck);
    (decks.next().unwrap(), decks.next().unwrap())
}

fn calculate_score(deck: &VecDeque<u8>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(a, &b)| (a + 1) * b as usize)
        .sum::<usize>()
}

pub fn solve_part1(mut player1: VecDeque<u8>, mut player2: VecDeque<u8>) -> usize {
    while !player1.is_empty() && !player2.is_empty() {
        play_combat(&mut player1, &mut player2);
    }
    let winner = if player1.is_empty() { player2 } else { player1 };
    calculate_score(&winner)
}

struct RecursiveCombatPlayer {
    history: HashSet<Vec<u8>>,
    player1: VecDeque<u8>,
    player2: VecDeque<u8>,
}

#[derive(Debug)]
enum Winner {
    None,
    Player1,
    Player2,
}

impl RecursiveCombatPlayer {
    fn play_round(&mut self) -> Winner {
        let mut snapshot = Vec::new();
        snapshot.extend(self.player1.iter().copied());
        snapshot.push(0xF0);
        snapshot.extend(self.player2.iter().copied());
        if !self.history.insert(snapshot) {
            return Winner::Player1;
        }

        let card1 = self.player1.pop_front().unwrap();
        let card2 = self.player2.pop_front().unwrap();

        if self.player1.len() >= card1 as usize && self.player2.len() >= card2 as usize {
            let subdeck1 = self.player1.iter().take(card1 as usize).copied().collect();
            let subdeck2 = self.player2.iter().take(card2 as usize).copied().collect();
            let mut subgame = Self {
                history: Default::default(),
                player1: subdeck1,
                player2: subdeck2,
            };

            match subgame.play_game() {
                Winner::Player1 => {
                    self.player1.push_back(card1);
                    self.player1.push_back(card2);
                }
                Winner::Player2 => {
                    self.player2.push_back(card2);
                    self.player2.push_back(card1);
                }
                Winner::None => unreachable!(),
            }
        } else if card1 > card2 {
            self.player1.push_back(card1);
            self.player1.push_back(card2);
        } else {
            self.player2.push_back(card2);
            self.player2.push_back(card1);
        }

        Winner::None
    }

    // p1 won = true
    // p2 won = false
    fn play_game(&mut self) -> Winner {
        while !self.player1.is_empty() && !self.player2.is_empty() {
            match self.play_round() {
                Winner::None => {}
                winner => return winner,
            }
        }
        if !self.player1.is_empty() {
            Winner::Player1
        } else {
            Winner::Player2
        }
    }
}

pub fn solve_part2(player1: VecDeque<u8>, player2: VecDeque<u8>) -> usize {
    let mut game = RecursiveCombatPlayer {
        history: Default::default(),
        player1,
        player2,
    };

    let winner = match game.play_game() {
        Winner::Player1 => game.player1,
        Winner::Player2 => game.player2,

        Winner::None => unreachable!(),
    };

    calculate_score(&winner)
}

#[inline]
pub fn solve() -> (usize, usize) {
    let (player1, player2) = load_input();

    let part1 = solve_part1(player1.clone(), player2.clone());
    let part2 = solve_part2(player1, player2);
    (part1, part2)
}
