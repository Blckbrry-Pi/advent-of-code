use network::Network;

mod network;

fn main() {
    part1();
    part2();
}

const TEST: &str = include_str!("../../../data/2022/day16/test.txt");
const INPUT: &str = include_str!("../../../data/2022/day16/input.txt");

fn part1() {
    let network = parse_input(INPUT);
    let start = network.start_state_p1();
    let total = network.find_best(start);

    println!("Part 1: {}", total);
}

fn part2() {
    let network = parse_input(INPUT);
    // println!("Part 2: {}", total);
}


fn parse_input(input: &'static str) -> Network {
    input.parse().unwrap()
}