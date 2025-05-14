aoc_tools::aoc_sol!(day01 2020: part1, part2);

const TARGET: u64 = 2020;

pub fn part1(input: &str) -> u64 {
    let list = parse_input(input);
    for a in 0..list.len() {
        if list[a] > TARGET { continue }
        for b in a+1..list.len() {
            if list[a] + list[b] == TARGET {
                return list[a] * list[b];
            }
        }
    }
    unreachable!("No solution found")
}

pub fn part2(input: &str) -> u64 {
    let list = parse_input(input);
    for a in 0..list.len() {
        if list[a] > TARGET { continue }
        for b in a+1..list.len() {
            if list[a] + list[b] > TARGET { continue }
            for c in b+1..list.len() {
                if list[a] + list[b] + list[c] == TARGET {
                    return list[a] * list[b] * list[c];
                }
            }
        }
    }
    unreachable!("No solution found")
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}
