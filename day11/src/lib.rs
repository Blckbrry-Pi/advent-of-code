aoc_tools::aoc_sol!(day11: part1, part2);

pub fn part1(input: &str) -> u64 {
    let mut rocks = parse_input(input);
    for _ in 0..25 { rocks = Rock::blink_all(&rocks); }
    Rock::size(&rocks)
}

pub fn part2(input: &str) -> u64 {
    let mut rocks = parse_input(input);
    for _ in 0..75 { rocks = Rock::blink_all(&rocks); }
    Rock::size(&rocks)
}

fn parse_input(input: &str) -> HashMap<Rock, u64> {
    let rock_iter = input.split(' ')
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.trim().parse::<u64>().unwrap())
        .map(Rock);

    let mut rocks = HashMap::new();
    for rock in rock_iter {
        *rocks.entry(rock).or_default() += 1;
    }

    rocks
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rock(u64);

impl Rock {
    pub fn digits(&self) -> (u64, u64) {
        match self.0 {
            ..10 => (1, 1),
            ..100 => (2, 10),
            ..1_000 => (3, 10),
            ..10_000 => (4, 100),
            ..100_000 => (5, 100),
            ..1_000_000 => (6, 1_000),
            ..10_000_000 => (7, 1_000),
            ..100_000_000 => (8, 10_000),
            ..1_000_000_000 => (9, 10_000),
            ..10_000_000_000 => (10, 100_000),
            ..100_000_000_000 => (11, 100_000),
            ..1_000_000_000_000 => (12, 1_000_000),
            ..10_000_000_000_000 => (13, 1_000_000),
            ..100_000_000_000_000 => (14, 10_000_000),
            ..1_000_000_000_000_000 => (15, 10_000_000),
            ..10_000_000_000_000_000 => (16, 100_000_000),
            ..100_000_000_000_000_000 => (17, 100_000_000),
            ..1_000_000_000_000_000_000 => (18, 1_000_000_000),
            ..10_000_000_000_000_000_000 => (19, 1_000_000_000),
            _ => (20, 10_000_000_000),
        }
    }
    pub fn blink(&self) -> (Self, Option<Self>) {
        if self.0 == 0 {
            (Rock(1), None)
        } else {
            let (digits, split) = self.digits();
            if digits & 1 == 1 {
                (Rock(self.0 * 2024), None)
            } else {
                let rock_l = self.0 / split;
                let rock_r = self.0 % split;
                (Rock(rock_l), Some(Rock(rock_r)))
            }
        }
    }
    pub fn blink_add(&self, rocks: &mut HashMap<Rock, u64>, multiples: u64) {
        let (l, r) = self.blink();
        *rocks.entry(l).or_default() += multiples;
        if let Some(r) = r {
            *rocks.entry(r).or_default() += multiples;
        }
    }

    pub fn blink_all(rocks: &HashMap<Rock, u64>) -> HashMap<Rock, u64> {
        let mut new_rocks = HashMap::with_capacity(rocks.len() * 2);
        for (&rock, &count) in rocks.iter() {
            rock.blink_add(&mut new_rocks, count);
        }
        new_rocks
    }

    pub fn size(rocks: &HashMap<Rock, u64>) -> u64 {
        rocks.values().copied().sum()
    }
}
