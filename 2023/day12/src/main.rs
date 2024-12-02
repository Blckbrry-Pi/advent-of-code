#![feature(iter_intersperse)]

use record::Record;

mod record;

fn main() {
    part1();
    part2();
}

const TEST: &str = include_str!("../../../data/2023/day12/test.txt");
const PART_1: &str = include_str!("../../../data/2023/day12/p1.txt");
const PART_2: &str = include_str!("../../../data/2023/day12/p2.txt");

fn part1() {
    let records = parse_input(PART_1);
    // let records = parse_input(TEST);
    // let records = vec![parse_input(TEST).into_iter().next().unwrap()];

    let mut total = 0;
    for record in records {
        let possibilities = record.count_valid_positions_base();
        total += possibilities;
    }

    println!("Part 1: {}", total);
}

fn part2() {
    let records: Vec<_> = parse_input(PART_2).iter().map(Record::unfold).collect();
    // let records: Vec<_> = parse_input(TEST).iter().map(Record::unfold).collect();
    // let records: Vec<_> = parse_input(TEST).iter().map(Record::unfold).take(2).collect();

    let mut total = 0;
    for record in records {
        let possibilities = record.count_valid_positions_base();
        // println!("{possibilities:6} -> {record:?}");
        total += possibilities;
    }

    println!("Part 2: {}", total);
}


fn parse_input(input: &'static str) -> Vec<Record> {
    input.split('\n')
        .map(|line| line.parse().unwrap())
        .collect()
}
