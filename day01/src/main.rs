mod pair;
mod count;

use count::Count;
use pair::Pair;

fn main() {
    part1();
    part2();
}

#[allow(dead_code)]
const TEST: &str = include_str!("../../data/day01/test.txt");
const INPUT: &str = include_str!("../../data/day01/input.txt");

fn part1() {
    let (mut left, mut right) = parse_input(INPUT);

    left.sort();
    right.sort();

    let sum: i32 = zipped(left, right).map(diffed).sum();

    println!("Part 1: {}", sum);
}

fn part2() {
    let (left, right) = parse_input(INPUT);
    let right_counts = Count::from_list(&right);

    let sum: i32 = left.into_iter().map(|v| v * right_counts.count(v)).sum();

    println!("Part 2: {}", sum);
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines: Vec<Pair> = input.split('\n')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    Pair::unzip_lists(&lines)
}


pub fn zipped<L, R>(left: impl IntoIterator<Item = L>, right: impl IntoIterator<Item = R>) -> impl Iterator<Item = (L, R)> {
    left.into_iter().zip(right)
}

pub fn diffed((l, r): (i32, i32)) -> i32 { l - r }
