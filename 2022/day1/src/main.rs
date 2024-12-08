fn main() {
    part1();
    part2();
}

const TEST: &str = include_str!("../../../data/2022/day1/test.txt");
const INPUT: &str = include_str!("../../../data/2022/day1/input.txt");

fn part1() {
    let lines = parse_input(INPUT);

    let mut calorie_counts: Vec<usize> = lines.iter()
        .map(|elf| elf.iter().copied().sum())
        .collect();

    calorie_counts.sort();

    println!("Part 1: {}", calorie_counts.last().unwrap());
}

fn part2() {
    let lines = parse_input(INPUT);

    let mut calorie_counts: Vec<usize> = lines.iter()
        .map(|elf| elf.iter().sum())
        .collect();

    calorie_counts.sort();

    println!("Part 2: {}", calorie_counts.iter().rev().take(3).sum::<usize>());
}


fn parse_input(input: &'static str) -> Vec<Vec<usize>> {
    input.split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}
