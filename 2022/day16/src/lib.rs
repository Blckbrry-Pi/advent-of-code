#![feature(debug_closure_helpers)]
#![feature(btree_extract_if)]

use network::Network;
use state::Solver;

mod network;
mod state;

aoc_tools::aoc_sol!(day16 2022: part1, part2);

pub fn part1(input: &str) -> u16 {
    let network = parse_input(input);
    let mut solver = Solver::part1(&network);
    while solver.advance(&network) {}
    solver.best_lower_bound
}

pub fn part2(input: &str) -> u16 {
    let network = parse_input(input);
    let mut solver = Solver::part2(&network);
    while solver.advance(&network) {}
    solver.best_lower_bound
}


fn parse_input(input: &str) -> Network {
    input.parse().unwrap()
}
