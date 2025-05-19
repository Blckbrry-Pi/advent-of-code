aoc_tools::aoc_sol!(day13 2020: part1, part2);

type Scalar = u64;

fn gcd(a: Scalar, b: Scalar) -> Scalar {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn lcm(a: Scalar, b: Scalar) -> Scalar {
    a / gcd(a, b) * b
}

pub fn part1(input: &str) -> Scalar {
    let (start, ids) = parse_input(input);
    let mut best_start = (Scalar::MAX, Scalar::MAX);
    for id in ids {
        let Some(id) = id else { continue };
        let need_to_wait = (id - (start % id)) % id;
        if need_to_wait < best_start.1 {
            best_start = (id, need_to_wait);
        }
    }
    best_start.0 * best_start.1
}

pub fn part2(input: &str) -> Scalar {
    let (_start, ids) = parse_input(input);
    let mut skip = 1;
    let mut t = 0;
    for i in 0..ids.len() {
        let Some(id) = ids[i] else { continue };
        while (t + i as Scalar) % id != 0 {
            t += skip;
        }
        skip = lcm(skip, id);
    }
    t
}

fn parse_input(input: &str) -> (Scalar, Vec<Option<Scalar>>) {
    let (start, ids) = input.split_once('\n').unwrap();
    (
        start.parse().unwrap(),
        ids.split(',').map(|id| id.trim().parse().ok()).collect(),
    )
}
