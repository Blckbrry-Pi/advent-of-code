#![feature(box_patterns)]

use monkeys::{MonkeyEntry, Riddle};

mod monkeys;
mod algebra;

aoc_tools::aoc_sol!(day21 2022: part1, part2);

pub fn part1(input: &str) -> i64 {
    let mut riddle = parse_input(input);
    riddle.reduce_from_root().as_part1_sol()
}

pub fn part2(input: &str) -> i64 {
    let mut riddle = parse_input(input);
    riddle.setup_for_part_2();
    riddle.reduce_from_root().as_part2_sol()
}

pub fn parse_input(input: &str) -> Riddle {
    let entries: Vec<MonkeyEntry> = input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse())
        .collect::<Result<_, _>>()
        .unwrap();
    Riddle {
        monkeys: entries
            .into_iter()
            .map(|MonkeyEntry(ident, monkey)| (ident, monkey))
            .collect(),
    }
}
