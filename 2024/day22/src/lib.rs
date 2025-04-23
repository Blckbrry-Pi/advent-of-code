#![feature(map_try_insert)]

aoc_tools::aoc_sol!(day22 2024: part1, part2);
aoc_tools::fast_hash!();

type DiffScalar = i8;
type P2Scalar = u16;
const MAX_ID: u32 = 10000;
const STEPS: u32 = 2000;

pub fn part1(input: &str) -> usize {
    let secrets = parse_input(input);

    secrets.iter()
        .copied()
        .map(|s| s.step_n(STEPS))
        .map(|s| s.val as usize)
        .sum()
}

pub fn part2(input: &str) -> P2Scalar {
    // let start = std::time::Instant::now();
    let secrets = parse_input(input);
    // let parsed = start.elapsed();

    let max_sequences = secrets.len() * STEPS as usize;

    let mut matches = new_fastmap_with_capacity(max_sequences / 1000);
    // let allocated = start.elapsed();

    secrets.iter().copied().for_each(|s| s.increment_matches(STEPS, &mut matches));
    // let incremented = start.elapsed();

    let max = matches.into_values().max_by_key(|&(a, _)| a);
    // let maxed = start.elapsed();

    // println!("parsing: {:?}", parsed);
    // println!("alloced: {:?}", allocated - parsed);
    // println!("incrmtd: {:?}", incremented - allocated);
    // println!("maxxedd: {:?}", maxed - incremented);

    max.unwrap().0
    // 0
}

fn parse_input(input: &str) -> Vec<Secret> {
    input.lines()
        .filter(|l| !l.is_empty())
        .map(|v| v.parse::<u32>())
        .map(|n| n.unwrap())
        .enumerate()
        .map(|(id, val)| Secret { id: id as u32, val })
        .collect()
}

// From 1995 to 2673

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Secret { id: u32, val: u32 }

impl Secret {
    pub fn step_unpruned(self) -> Self {
        let secret = self;
        let secret = secret.mixed(secret.val << 6).pruned();
        let secret = secret.mixed(secret.val >> 5);
        // let secret = secret.mixed(secret.val << 11).pruned();
        let secret = secret.mixed(secret.val << 11);
        secret
    }
    pub fn step_n(self, n: u32) -> Self {
        let mut curr_secret = self;
        for _ in 0..n { curr_secret = curr_secret.step_unpruned(); }
        curr_secret.pruned()
    }
    pub fn increment_matches(self, n: u32, matches: &mut FastMap<u32, (P2Scalar, u32)>) {
        let mut curr_secret = self;
        let mut curr_offer = 0;
        let mut curr_changes = Sequence([0; 4]);
        // let mut curr_changes = Sequence(0);
        for step in 0..n {
            let new_secret = curr_secret.step_unpruned().pruned();
            let new_offer = new_secret.offer();
            let change = new_offer - curr_offer;
            curr_changes = curr_changes.shift_in(change);
            if step >= 3 {
                let index = (MAX_ID - self.id as u32) * n + step;
                let curr_entry = matches.entry(curr_changes.idx())
                    .or_insert((0, u32::MAX));
                if index < curr_entry.1 {
                    curr_entry.0 += new_secret.offer() as P2Scalar;
                    // curr_entry.0 += new_offer as P2Scalar;
                    curr_entry.1 = index;
                }
            }

            curr_secret = new_secret;
            curr_offer = new_offer;
        }
    }
    pub fn mixed(self, n: u32) -> Self {
        Secret { id: self.id, val: self.val ^ n }
    }
    pub fn pruned(self) -> Self {
        Secret { id: self.id, val: self.val & 0xFFFFFF }
    }

    pub fn offer(self) -> DiffScalar {
        (self.val % 10) as DiffScalar
    }
}
impl Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S<{:03x}:{}>", self.id, self.val)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Sequence([DiffScalar; 4]);
// struct Sequence(u32);
impl Sequence {
    pub fn shift_in(self, new: DiffScalar) -> Self {
        Self([self.0[1], self.0[2], self.0[3], new])
    }
    // pub fn shift_in(&mut self, new: DiffScalar) {
    //     self.0 <<= 8;
    //     self.0 |= new as u32;
    //     // Self((self.0 << 8) | new as u32)
    // }
    pub fn idx(self) -> u32 {
        // self.0
        u32::from_ne_bytes(self.0.map(|v| v as u8))
    }
}
