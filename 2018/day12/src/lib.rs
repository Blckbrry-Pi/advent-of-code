use std::collections::VecDeque;

aoc_tools::aoc_sol!(day12 2018: part1, part2);

#[derive(Clone, Copy, PartialEq, Eq)]
struct Rules(u32);
impl Rules {
    pub fn from_indices(indices: &[u8]) -> Self {
        Self(indices.iter().fold(0, |v, idx| {
            v | (1 << *idx)
        }))
    }
    pub fn get(&self, value: u8) -> bool {
        (self.0 >> (value as u32)) & 0b1 == 1
    }
    pub fn advance_state(&self, state: &mut (VecDeque<bool>, isize)) {
        if self.get(0) == true { panic!("Aaaaaa :("); }
        let mut state_window = 0b00000;
        state.1 -= 2;
        state.0.push_back(false);
        state.0.push_back(false);
        state.0.push_front(false);
        state.0.push_front(false);
        for idx in 0..state.0.len() {
            state_window <<= 1;
            state_window |= *state.0.get(idx+2).unwrap_or(&false) as u8;
            state_window &= 0b11111;

            state.0[idx] = self.get(state_window)
        }

        while state.0.front() == Some(&false) {
            state.0.pop_front();
            state.1 += 1;
        }
        while state.0.back() == Some(&false) {
            state.0.pop_back();
        }
    }
}
impl Debug for Rules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..32 {
            for b in 0..5 {
                if (i >> (4 - b)) & 0b1 == 1 {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, " => ")?;
            if (self.0 >> i) & 0b1 == 1 {
                writeln!(f, "#")?;
            } else {
                writeln!(f, ".")?;
            }
        }
        Ok(())
    }
}

pub fn display_state(state: &(VecDeque<bool>, isize), indices: impl IntoIterator<Item = isize>) {
    for i in indices {
        let has_plant = *state.0.get((i - state.1) as usize).unwrap_or(&false);
        if has_plant {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();
}

pub fn state_score(state: &(VecDeque<bool>, isize)) -> isize {
    let mut sum = 0;
    for (i, &has_plant) in state.0.iter().enumerate() {
        if !has_plant { continue }
        sum += i as isize + state.1 as isize;
    }
    sum
}

pub fn part1(input: &str) -> isize {
    let (initial, rules) = parse_input(input);
    let rules = Rules::from_indices(&rules);
    let mut state = (initial, 0);
    for _ in 0..20 {
        rules.advance_state(&mut state);
    }

    state_score(&state)
}

pub fn part2(input: &str) -> isize {
    const STEPS: isize = 50_000_000_000;

    let (initial, rules) = parse_input(input);
    let rules = Rules::from_indices(&rules);
    let mut state = (initial, 0);
    let mut prev = state.clone();
    let mut i = 0;
    while i < STEPS {
        rules.advance_state(&mut state);
        i += 1;
        if state.0 == prev.0 { break; }
        prev = state.clone();
    }

    let offset = state.1 - prev.1;
    let plant_count = state.0.iter().filter(|v| **v).count() as isize;
    let total_correction = (STEPS - i) * offset * plant_count;
    state_score(&state) + total_correction
}

fn parse_input(input: &str) -> (VecDeque<bool>, Vec<u8>) {
    let (initial, rules) = input.split_once("\n\n").unwrap();
    let initial = initial.strip_prefix("initial state: ").unwrap();
    let initial: VecDeque<_> = initial.chars().map(|c| c == '#').collect();

    let rules: Vec<_> = rules.trim()
        .lines()
        .filter_map(|rule| {
            let (pattern, output) = rule.split_once(" => ").unwrap();
            if output != "#" { return None }
            let pattern = pattern.chars().fold(0, |v, c| v * 2 + (c == '#') as u8);
            Some(pattern)
        })
        .collect();

    (initial, rules)
}
