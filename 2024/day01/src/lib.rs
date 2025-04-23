mod count;

use count::Count;

aoc_tools::aoc_sol!(day01 2024: part1, part2);

pub fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);

    left.sort_unstable();
    right.sort_unstable();

    let mut sum = 0;
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }
    
    sum
}

pub fn part2(input: &str) -> i32 {
    let (left, right) = parse_input(input);
    let right_counts = Count::from_list(&right);

    left.into_iter().map(|v| v * right_counts.count(v)).sum()
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    const INPUT_LINE_LEN: usize = 14;
    const NUM_WIDTH: usize = 5;

    let mut l_list = Vec::with_capacity(input.len() / INPUT_LINE_LEN + 5);
    let mut r_list = Vec::with_capacity(input.len() / INPUT_LINE_LEN + 5);

    let mut i = 0;
    while i < input.len() {
        let l = i..i+NUM_WIDTH;
        let r = i+NUM_WIDTH+3..i+NUM_WIDTH+3+NUM_WIDTH;

        let l = parse_n_5(&input.as_bytes()[l]);
        let r = parse_n_5(&input.as_bytes()[r]);

        l_list.push(l);
        r_list.push(r);

        i += INPUT_LINE_LEN;
    }

    (l_list, r_list)
}

aoc_tools::parse_unsigned!(parse_n_5<i32>(= 5 digits));
