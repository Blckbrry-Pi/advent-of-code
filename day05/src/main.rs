use std::{ collections::HashMap, str::FromStr };

fn main() {
    part1();
    part2();
}

const TEST: &str = include_str!("../../data/day05/test.txt");
const INPUT: &str = include_str!("../../data/day05/input.txt");

fn part1() {
    let start = std::time::Instant::now();
    let (requirements, updates) = parse_input(INPUT);

    let mut sum = 0;
    for update in updates {
        if update.is_valid(&requirements) {
            sum += update.middle() as u16;
        }
    }

    println!("Part 1: {} ({:?})", sum, start.elapsed());
}

fn part2() {
    let start = std::time::Instant::now();
    let (requirements, updates) = parse_input(INPUT);
    let lut = Update::build_requirement_lut(&requirements);

    let mut sum = 0;
    for mut update in updates {
        if !update.is_valid(&requirements) {
            update.order_correctly(&lut);
            sum += update.middle() as u16;
        }
    }

    println!("Part 2: {} ({:?})", sum, start.elapsed());
}

fn parse_input(input: &str) -> (Vec<Requirement>, Vec<Update>) {
    let (requirements, updates) = input.split_once("\n\n").unwrap();
    let requirements = requirements.lines().map(Requirement::from_str).map(Result::unwrap).collect();
    let updates = updates.lines().map(Update::from_str).map(Result::unwrap).collect();

    (requirements, updates)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Requirement {
    before: u8,
    after: u8,
}

impl Requirement {
    fn min(&self) -> u8 {
        self.before.min(self.after)
    }

    fn max(&self) -> u8 {
        self.before.max(self.after)
    }

    fn key(&self) -> u16 {
        ((self.min() as u16) << 8) | self.max() as u16
    }

    fn key_for(a: u8, b: u8) -> u16 {
        ((a.min(b) as u16) << 8) | a.max(b) as u16
    }
}

impl FromStr for Requirement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('|').ok_or(())?;
        Ok(Self {
            before: left.trim().parse().map_err(|_| ())?,
            after: right.trim().parse().map_err(|_| ())?,
        })
    }
}

#[derive(Debug, Clone)]
struct Update {
    parts: Vec<u8>,
    indicies: HashMap<u8, usize>,
}

impl Update {
    fn is_valid(&self, requirements: &[Requirement]) -> bool {
        for req in requirements {
            let Some(before) = self.indicies.get(&req.before)  else { continue };
            let Some(after ) = self.indicies.get(&req.after ) else { continue };
            if before > after { return false; }
        }
        true
    }

    fn middle(&self) -> u8 {
        self.parts[self.parts.len() / 2]
    }

    fn build_requirement_lut(requirements: &[Requirement]) -> HashMap<u16, Requirement> {
        let mut lut = HashMap::new();
        for &req in requirements {
            lut.insert(req.key(), req);
        }
        lut
    }

    fn order_correctly(&mut self, requirements: &HashMap<u16, Requirement>) {
        self.parts.sort_by(|a, b| {
            let key = Requirement::key_for(*a, *b);
            // let Some(req) = requirements.get(&key) else {
            //     return std::cmp::Ordering::Equal;
            // };
            let req = requirements.get(&key).unwrap();

            if req.before == *a {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
    }

    fn rebuild_indicies(&mut self) {
        self.indicies.clear();
        for (i, part) in self.parts.iter().enumerate() {
            self.indicies.insert(part.clone(), i);
        }
    }
}

impl FromStr for Update {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Result<Vec<u8>, _> = s.split(',').map(|s| s.parse()).collect();
        let Ok(parts) = parts else { return Err(()) };

        let mut out = Self { parts, indicies: HashMap::new() };
        out.rebuild_indicies();

        Ok(out)
    }
}

