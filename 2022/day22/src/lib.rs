mod instructions;
mod map;

use std::str::FromStr;
use crate::instructions::{parse_instructions, Instruction};
use crate::map::Map;

type Scalar = isize;

aoc_tools::aoc_sol!(day22 2022 test: part1, part2);
aoc_tools::pos!(Scalar; +y => D);

pub fn part1(input: &str) -> Scalar {
    let (mut map, instructions) = parse_input(input);
    map.setup_part1_redirects();
    let mut state = map.start();
    for instruction in instructions {
        state = state.handle(&map, instruction);
    }
    state.password()
}

pub fn part2(input: &str) -> i64 {
    let _ = parse_input(input);
    0
}

fn parse_input(input: &str) -> (Map, Vec<Instruction>) {
    let (map, instructions) = input.split_once("\n\n").unwrap();
    (
        map.parse().unwrap(),
        parse_instructions(instructions),
    )
}
