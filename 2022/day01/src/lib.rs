aoc_tools::aoc_sol!(day01 2022: part1, part2);

pub fn part1(input: &str) -> usize {
    let lines = parse_input(input);

    let mut calorie_counts: Vec<usize> = lines.iter()
        .map(|elf| elf.iter().copied().sum())
        .collect();

    calorie_counts.sort();

    calorie_counts.last().copied().unwrap()
}

pub fn part2(input: &str) -> usize {
    let lines = parse_input(input);

    let mut calorie_counts: Vec<usize> = lines.iter()
        .map(|elf| elf.iter().sum())
        .collect();

    calorie_counts.sort();

    calorie_counts.iter().rev().take(3).sum::<usize>()
}


fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}
