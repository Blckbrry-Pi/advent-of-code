use std::time::Instant;

fn main() {
    part1();
    part2();
}

#[allow(dead_code)]
const TEST: &str = include_str!("../../../data/2019/day01/test.txt");
const INPUT: &str = include_str!("../../../data/2019/day01/input.txt");

fn part1() {
    let start = Instant::now();
    let lines = parse_input(INPUT);

    let sum: usize = lines.into_iter().map(|mass| mass / 3 - 2).sum::<usize>();

    println!("Part 2: {sum} {:?}", start.elapsed());
}

fn part2() {
    let start = Instant::now();
    let lines = parse_input(INPUT);

    let sum: usize = lines.into_iter().map(calc_fuel).sum::<usize>();

    println!("Part 2: {sum} {:?}", start.elapsed());
}


fn parse_input(input: &'static str) -> Vec<usize> {
    input.lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn calc_fuel(v: usize) -> usize {
    if v == 0 { return 0 }
    let fuel = (v / 3).saturating_sub(2);
    fuel + calc_fuel(fuel)
}
