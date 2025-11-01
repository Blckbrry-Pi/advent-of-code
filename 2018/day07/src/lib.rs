#[derive(Clone, Copy, PartialEq, Eq, Default)]
struct StepSet(u32);
#[allow(dead_code)]
impl StepSet {
    pub fn new() -> Self { Self(0) }

    /// Returns whether the step was already in the set
    pub fn insert(&mut self, step: u8) -> bool {
        let value = 1_u32 << step;
        let had = self.0 & value != 0;
        self.0 |= value;
        had
    }
    /// Returns whether the step was in the set
    pub fn remove(&mut self, step: u8) -> bool {
        let value = 1_u32 << step;
        let had = self.0 & value != 0;
        self.0 &= !value;
        had
    }

    pub fn has(&self, step: u8) -> bool {
        (self.0 >> step) & 0b1 != 0
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn into_iter(self) -> impl Iterator<Item = u8> {
        (0..32).filter_map(move |s| self.has(s).then_some(s))
    }

    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
    pub fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
    pub fn minus(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }
    pub fn complement(self) -> Self {
        Self(!self.0)
    }
}
impl Debug for StepSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut set = f.debug_set();
        set.entries(self.into_iter().map(|s| (b'A' + s) as char));
        set.finish()
    }
}

aoc_tools::aoc_sol!(day07 2018: part1, part2);

pub fn part1(input: &str) -> String {
    let mut reliance_list = parse_input(input);
    let mut done = StepSet::new();
    let mut output = String::with_capacity(reliance_list.len());
    loop {
        let mut updated = false;
        for i in 0..reliance_list.len() {
            let Some(reliances) = reliance_list[i].as_mut() else { continue };
            *reliances = reliances.minus(done);
            if reliances.is_empty() {
                reliance_list[i] = None;
                done.insert(i as u8);
                output.push((b'A' + i as u8) as char);
                updated = true;
                break;
            }
        }
        if !updated { break }
    }
    output
}

pub fn part2(input: &str) -> u32 {
    const TIME_OFFSET: u8 = 60;
    const ELF_COUNT: usize = 5;

    let mut reliance_list = parse_input(input);
    let mut done = StepSet::new();
    let mut elves = [None; ELF_COUNT];
    for secs in 0.. {
        for s in 0..reliance_list.len() {
            let Some(reliances) = reliance_list[s].as_mut() else { continue };
            *reliances = reliances.minus(done);
            if !reliances.is_empty() { continue }
            for i in 0..elves.len() {
                if elves[i] != None { continue }
                elves[i] = Some((s as u8 + 1 + TIME_OFFSET, s));
                reliance_list[s] = None;
                break;
            }
        }
        if elves == [None; ELF_COUNT] {
            return secs
        }

        for e in &mut elves {
            let Some((time, idx)) = e else { continue };
            *time -= 1;
            if *time == 0 {
                done.insert(*idx as u8);
                *e = None;
            }
        }

    }
    panic!();
}

fn parse_input(input: &str) -> Vec<Option<StepSet>> {
    let mut reliances = HashMap::<u8, StepSet>::new();
    for line in input.lines() {
        if line.trim().is_empty() { continue }
        let line = line.strip_prefix("Step ").unwrap();
        let (preceding, rest) = line.split_once(" must be finished before step ").unwrap();
        let relyer = rest.strip_suffix(" can begin.").unwrap();
        let relyer = relyer.as_bytes()[0] - b'A';
        let preceding = preceding.as_bytes()[0] - b'A';
        reliances.entry(relyer).or_default().insert(preceding);
        reliances.entry(preceding).or_default();
    }
    let max_key = *reliances.keys().max().unwrap();
    let mut output = vec![None; max_key as usize + 1];
    for (relyer, reliances) in reliances {
        output[relyer as usize] = Some(reliances);
    }
    output
}
