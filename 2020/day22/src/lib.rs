use std::{collections::VecDeque, num::Wrapping};

aoc_tools::aoc_sol!(day22 2020: part1, part2);

#[derive(Debug, Clone)]
struct Player {
    cards: VecDeque<u8>,
}
impl Player {
    pub fn score(&self) -> u64 {
        self.cards.iter()
            .rev()
            .enumerate()
            .map(|(i, &card)| (i as u64 + 1) * card as u64)
            .sum()
    }
    pub fn hash(&self) -> u32 {
        let mut val = Wrapping(0);
        for &card in &self.cards {
            val <<= 1;
            val ^= card as u32;
        }
        val.0
    }
    pub fn top(&self, count: u8, reserve: usize) -> Self {
        let mut new_cards = VecDeque::with_capacity(reserve);
        for i in 0..count as usize {
            new_cards.push_back(self.cards[i]);
        }
        Self {
            cards: new_cards,
        }
    }
}


struct InfiniteGameGuard(HashSet<u32>);
impl InfiniteGameGuard {
    pub fn new() -> Self {
        Self(HashSet::new())
    }
    pub fn should_break(&mut self, player1: &Player, player2: &Player) -> bool {
        !self.0.insert(player1.hash() ^ player2.hash())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Winner { P1, P2 }

const DEBUG: bool = false;

fn recursive_combat<const RECURSIVE: bool>(player1: Player, player2: Player, game_id_mut: &mut usize) -> (Winner, u64) {
    let game_id = *game_id_mut;
    *game_id_mut += 1;
    let mut round = 1;
    if DEBUG {
        println!("=== Game {game_id} ===");
    }

    let count = player1.cards.len() + player2.cards.len();

    let mut player1 = player1;
    let mut player2 = player2;
    let mut guard = InfiniteGameGuard::new();
    while !player1.cards.is_empty() && !player2.cards.is_empty() {
        if DEBUG {
            println!();
            println!("-- Round {round} (Game {game_id}) --");
            println!("Player 1's deck: {player1:?}");
            println!("Player 2's deck: {player2:?}");
        }

        // Infinite game prevention rule
        if RECURSIVE && round > count * 6 && guard.should_break(&player1, &player2) {
            return (Winner::P1, player1.score());
        }

        // Draw top cards
        let p1 = player1.cards.pop_front().unwrap();
        let p2 = player2.cards.pop_front().unwrap();

        if DEBUG {
            println!("Player 1 plays: {p1}");
            println!("Player 2 plays: {p2}");
        }

        // Figure out if the special recursive case can even occur
        let p1_has_enough = p1 as usize <= player1.cards.len();
        let p2_has_enough = p2 as usize <= player2.cards.len();

        // Determine the winner of the round
        let winner = if p1_has_enough && p2_has_enough && RECURSIVE {
            if DEBUG {
                println!("Playing a sub-game to determine the winner...");
                println!();
            }

            let reserve = p1 as usize + p2 as usize;
            let new_player1 = player1.top(p1, reserve);
            let new_player2 = player2.top(p2, reserve);
            let winner = recursive_combat::<RECURSIVE>(new_player1, new_player2, game_id_mut).0;
            if DEBUG { println!("...anyway, back to game {game_id}.") }
            winner
        } else {
            if p1 > p2 {
                Winner::P1
            } else {
                Winner::P2
            }
        };

        // Put the cards back on the bottom of the winner's deck
        if winner == Winner::P1 {
            if DEBUG { print!("Player 1") }
            player1.cards.push_back(p1);
            player1.cards.push_back(p2);
        } else {
            if DEBUG { print!("Player 2") }
            player2.cards.push_back(p2);
            player2.cards.push_back(p1);
        }
        if DEBUG {
            println!(" wins round {round} of game {game_id}!");
        }
        round += 1;
    }
    if DEBUG {
        print!("The winner of game {game_id} is ");
    }
    if player2.cards.is_empty() {
        if DEBUG { println!("player 1!\n") }
        (Winner::P1, player1.score())
    } else {
        if DEBUG { println!("player 2!\n") }
        (Winner::P2, player2.score())
    }
}

pub fn part1(input: &str) -> u64 {
    let (player1, player2) = parse_input(input);
    recursive_combat::<false>(player1, player2, &mut 1).1
}

pub fn part2(input: &str) -> u64 {
    let (player1, player2) = parse_input(input);
    recursive_combat::<true>(player1, player2, &mut 1).1
}

fn parse_input(input: &str) -> (Player, Player) {
    let (player1, player2) = input.split_once("\n\nPlayer 2:\n").unwrap();
    let player1: VecDeque<_> = player1.lines().skip(1).map(|l| l.parse::<u8>().unwrap()).collect();
    let player2: VecDeque<_> = player2.trim().lines().map(|l| l.parse::<u8>().unwrap()).collect();
    (Player { cards: player1 }, Player { cards: player2 })
}
