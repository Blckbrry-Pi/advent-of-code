#![feature(iter_intersperse)]

use record::Record;

mod record;

fn main() {
    part1();
    part2();
}

const TEST: &str = include_str!("../../../data/2023/day12/test.txt");
const INPUT: &str = include_str!("../../../data/2023/day12/input.txt");

fn part1() {
    let records = parse_input(INPUT);

    let mut total = 0;
    for record in records {
        let possibilities = record.count_valid_positions_base();
        total += possibilities;
    }

    println!("Part 1: {}", total);
}

fn part2() {
    let records: Vec<_> = parse_input(INPUT).iter().map(Record::unfold).collect();

    let mut total = 0;
    for record in records {
        let possibilities = record.count_valid_positions_base();
        total += possibilities;
    }

    println!("Part 2: {}", total);
}


fn parse_input(input: &'static str) -> Vec<Record> {
    input.split('\n')
        .map(|line| line.parse().unwrap())
        .collect()
}
