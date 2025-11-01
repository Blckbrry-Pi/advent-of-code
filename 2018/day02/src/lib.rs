aoc_tools::aoc_sol!(day02 2018: part1, part2);

pub fn has_exactly_n_repeats(id: &[u8], n: usize) -> bool {
    let to_check: HashSet<_> = id.iter().copied().collect();
    for byte in to_check {
        if id.iter().filter(|b| **b == byte).count() == n { return true }
    }
    false
}

pub fn part1(input: &str) -> usize {
    let ids = parse_input(input);
    let twices = ids.iter()
        .filter(|id| has_exactly_n_repeats(id, 2))
        .count();
    let thrices = ids.iter()
        .filter(|id| has_exactly_n_repeats(id, 3))
        .count();
    twices * thrices
}

pub fn part2(input: &str) -> String {
    let ids = parse_input(input);
    let mut seen = HashMap::new();
    for i in 0..ids.len() {
        for removed in 0..ids[i].len() {
            let mut modified = ids[i].clone();
            modified.remove(removed);

            if let Some(p) = seen.insert(modified, i) {
                if p != i {
                    let mut modified = ids[i].clone();
                    modified.remove(removed);
                    return String::from_utf8(modified).unwrap();
                }
            }
        }
    }
    panic!("I'm sad now :(")
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect()
}
