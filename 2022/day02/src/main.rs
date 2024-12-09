fn main() {
    part1();
    part2();
}

#[allow(dead_code)]
const TEST: &str = include_str!("../../../data/2022/day02/test.txt");
const INPUT: &str = include_str!("../../../data/2022/day02/input.txt");

fn part1() {
    let rounds = parse_input(INPUT);
    
    let total: usize = rounds.iter()
        .map(|(opponent, player)| Play::round_score(*player, *opponent))
        .sum();

    println!("Part 1: {}", total);
}

fn part2() {
    let rounds = parse_input(INPUT);
    
    let total: usize = rounds.iter()
        .map(|(opponent, player)| Play::round_score_with_intent((*player).into(), *opponent))
        .sum();


    println!("Part 2: {}", total);
}


fn parse_input(input: &'static str) -> Vec<(Play, Play)> {
    input.split("\n")
        .map(|round| {
            let opponent = round.chars().nth(0).unwrap();
            let player = round.chars().nth(2).unwrap();

            (Play::from_char(opponent), Play::from_char(player))
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn from_char(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Invalid play"),
        }
    }


    fn outcome(player: Self, opponent: Self) -> Outcome {
        match (player, opponent) {
            (Self::Rock, Self::Scissors) => Outcome::Win,
            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Scissors, Self::Paper) => Outcome::Win,

            (Self::Rock, Self::Paper) => Outcome::Lose,
            (Self::Paper, Self::Scissors) => Outcome::Lose,
            (Self::Scissors, Self::Rock) => Outcome::Lose,

            _ => Outcome::Draw,
        }
    }

    fn round_score(player: Self, opponent: Self) -> usize {
        let play_score = match player {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        };

        play_score + Self::outcome(player, opponent).value()
    }

    fn round_score_with_intent(intent: Outcome, opponent: Self) -> usize {
        for play in [Self::Rock, Self::Paper, Self::Scissors].iter() {
            if Self::outcome(*play, opponent) == intent {
                return Self::round_score(*play, opponent);
            }
        }
        unreachable!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn value(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

impl From<Play> for Outcome {
    fn from(play: Play) -> Self {
        match play {
            Play::Rock => Self::Lose,
            Play::Paper => Self::Draw,
            Play::Scissors => Self::Win,
        }
    }
}