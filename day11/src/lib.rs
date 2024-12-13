aoc_tools::aoc_sol!(day11: part1, part2);

fn part1(input: &str) -> u64 {
    let mut rocks = parse_input(input);
    for _ in 0..25 { rocks = Rock::blink_all(&rocks); }

    Rock::size(&rocks)
}

fn part2(input: &str) -> u64 {
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
    pub fn digits(&self) -> u64 {
        let mut curr = self.0;
        let mut digits = 0;
        while curr > 0 {
            curr /= 10;
            digits += 1;
        }
        digits
    }
    pub fn blink(&self) -> (Self, Option<Self>) {
        let digits = self.digits();
        if self.0 == 0 {
            (Rock(1), None)
        } else if digits % 2 == 1 {
            (Rock(self.0 * 2024), None)
        } else {
            let split = 10_u64.pow(digits as u32 / 2);
            let rock_l = self.0 / split;
            let rock_r = self.0 % split;
            (Rock(rock_l), Some(Rock(rock_r)))
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
