use std::collections::VecDeque;

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
    pub fn hash(&self) -> u128 {
        let mut val = 0;
        for &card in &self.cards {
            val <<= 5;
            val |= card as u128;
        }
        val
    }
    pub fn top(&self, count: u8) -> Self {
        Self {
            cards: self.cards.iter().take(count as usize).copied().collect(),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Winner { P1, P2 }

const DEBUG: bool = false;

fn recursive_combat(player1: Player, player2: Player, game_id_mut: &mut usize, recurse: bool) -> (Winner, u64) {
    let game_id = *game_id_mut;
    *game_id_mut += 1;
    let mut round = 1;
    if DEBUG {
        println!("=== Game {game_id} ===");
    }

    let mut player1 = player1;
    let mut player2 = player2;
    let mut seen = HashSet::new();
    while !player1.cards.is_empty() && !player2.cards.is_empty() {
        if DEBUG {
            println!();
            println!("-- Round {round} (Game {game_id}) --");
            println!("Player 1's deck: {player1:?}");
            println!("Player 2's deck: {player2:?}");
        }

        // Infinite game prevention rule
        if !seen.insert((player1.hash(), player2.hash())) {
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
        let winner = if p1_has_enough && p2_has_enough && recurse {
            if DEBUG {
                println!("Playing a sub-game to determine the winner...");
                println!();
            }
            let winner = recursive_combat(player1.top(p1), player2.top(p2), game_id_mut, recurse).0;
            if DEBUG {
                println!("...anyway, back to game {game_id}.");
            }
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
    recursive_combat(player1, player2, &mut 1, false).1
}

pub fn part2(input: &str) -> u64 {
    let (player1, player2) = parse_input(input);
    recursive_combat(player1, player2, &mut 1, true).1
}

fn parse_input(input: &str) -> (Player, Player) {
    let (player1, player2) = input.split_once("\n\nPlayer 2:\n").unwrap();
    let player1: VecDeque<_> = player1.lines().skip(1).map(|l| l.parse::<u8>().unwrap()).collect();
    let player2: VecDeque<_> = player2.trim().lines().map(|l| l.parse::<u8>().unwrap()).collect();
    (Player { cards: player1 }, Player { cards: player2 })
}
