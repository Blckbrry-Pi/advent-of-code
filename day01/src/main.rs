mod pair;
mod count;

use count::Count;
use pair::Pair;

aoc_tools::aoc_sol!(day01: part1, part2);

fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);

    left.sort();
    right.sort();

    zipped(left, right).map(diffed).sum()
}

fn part2(input: &str) -> i32 {
    let (left, right) = parse_input(input);
    let right_counts = Count::from_list(&right);

    left.into_iter().map(|v| v * right_counts.count(v)).sum()
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

pub fn diffed((l, r): (i32, i32)) -> i32 { (l - r).abs() }
