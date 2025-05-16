aoc_tools::aoc_sol!(day10 2020: part1, part2);

pub fn part1(input: &str) -> u64 {
    let joltages = parse_input(input);
    let mut diff_1 = 0;
    let mut diff_3 = 0;
    for i in 1..joltages.len() {
        let prev_adapter = joltages[i-1];
        let new_adapter = joltages[i];
        match new_adapter - prev_adapter {
            1 => diff_1 += 1,
            3 => diff_3 += 1,
            _ => (),
        }
    }
    diff_1 * diff_3
}

pub fn part2(input: &str) -> u64 {
    let joltages = parse_input(input);
    let mut counts_starting_at = HashMap::new();
    counts_starting_at.insert(*joltages.last().unwrap(), 1);
    for i in (0..joltages.len()).rev() {
        let curr_val = joltages[i as usize];

        let mut opts = 0;
        for next_opt in i+1..joltages.len() {
            let next_val = joltages[next_opt];
            if next_val - curr_val <= 3 {
                opts += counts_starting_at.get(&next_val).unwrap();
            }
        }
        counts_starting_at.insert(curr_val, opts.max(1));
    }
    *counts_starting_at.get(&0).unwrap()
}

fn parse_input(input: &str) -> Vec<u64> {
    let mut joltages: Vec<_> = input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();
    joltages.push(0);
    joltages.sort_unstable();
    joltages.push(*joltages.last().unwrap() + 3);
    joltages
}
