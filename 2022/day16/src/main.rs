#![feature(debug_closure_helpers)]
#![feature(btree_extract_if)]

use network::Network;
use state::Solver;

mod network;
mod state;

fn main() {
    part1();
    part2();
}

#[allow(dead_code)]
const TEST: &str = include_str!("../../../data/2022/day16/test.txt");
const INPUT: &str = include_str!("../../../data/2022/day16/input.txt");

fn part1() {
    let network = parse_input(INPUT);
    let mut solver = Solver::part1(&network);
    while solver.advance(&network) {}
    println!("Part 1: {}", solver.best_lower_bound);
}

fn part2() {
    let network = parse_input(INPUT);
    let mut solver = Solver::part2(&network);
    while solver.advance(&network) {}
    println!("Part 2: {}", solver.best_lower_bound);
}


fn parse_input(input: &'static str) -> Network {
    input.parse().unwrap()
}
