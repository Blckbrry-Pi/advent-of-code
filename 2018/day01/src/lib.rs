aoc_tools::aoc_sol!(day01 2018: part1, part2);

pub fn part1(input: &str) -> i32 {
    let offsets = parse_input(input);
    offsets.iter().sum()
}

pub fn part2(input: &str) -> i32 {
    let offsets = parse_input(input);
    let mut curr = 0;
    let mut seen: HashSet<_> = [curr].into_iter().collect();
    for &offset in std::iter::repeat(&offsets).flatten() {
        curr += offset;
        if !seen.insert(curr) { return curr }
    }
    panic!("No frequency reached twice")
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}
