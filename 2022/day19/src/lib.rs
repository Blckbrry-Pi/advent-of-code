use blueprints::Blueprint;
use state::State;

mod blueprints;
mod state;

aoc_tools::aoc_sol!(day19 2022: part1, part2);

pub fn part1(input: &str) -> i64 {
    let blueprints = parse_input(input);
    let mut sum = 0;
    for blueprint in blueprints {
        let quality = State::part1().recurse(&blueprint, &mut HashMap::new(), &mut 0);
        sum += quality as i64 * blueprint.id as i64;
    }
    sum
}

pub fn part2(input: &str) -> i64 {
    let blueprints = parse_input(input);
    let mut prod = 1;
    for blueprint in blueprints.into_iter().take(3) {
        let quality = State::part2().recurse(&blueprint, &mut HashMap::new(), &mut 0);
        prod *= quality as i64;
    }
    prod
}


fn parse_input(input: &str) -> Vec<Blueprint> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}
