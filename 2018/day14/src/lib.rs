#![feature(cold_path)]

aoc_tools::aoc_sol!(day14 2018: part1, part2);

struct State(Vec<u8>, [usize; 2], [u8; 2]);
impl State {
    pub fn extend_list(&mut self, target_last: u8) -> bool {
        let mut sum = self.2[0] + self.2[1];
        let must_check = if sum >= 10 {
            self.0.push(1);
            sum -= 10;
            target_last == 1
        } else { false };

        self.0.push(sum);
        must_check || target_last == sum
    }

    pub fn choose_next_recipes(&mut self) {
        self.1[0] += 1 + self.2[0] as usize;
        self.1[1] += 1 + self.2[1] as usize;

        if self.0.len() >= 10 {
            if self.1[0] >= self.0.len() {
                self.1[0] -= self.0.len();
            }
            if self.1[1] >= self.0.len() {
                self.1[1] -= self.0.len();
            }
        } else {
            std::hint::cold_path();
            self.1[0] %= self.0.len();
            self.1[1] %= self.0.len();
        }

        self.2[0] = self.0[self.1[0]];
        self.2[1] = self.0[self.1[1]];
    }

    pub fn index_of_last_2(&self, l: &[u8]) -> Option<usize> {
        if self.0.ends_with(l) {
            Some(self.0.len() - l.len())
        } else if self.0[..self.0.len()-1].ends_with(l) {
            Some(self.0.len() - l.len() - 1)
        } else {
            None
        }
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.0.len() {
            let (o, c) = if self.1[0] == i {
                ('(', ')')
            } else if self.1[1] == i {
                ('[', ']')
            } else {
                (' ', ' ')
            };
            write!(f, "{o}{}{c}", self.0[i])?;
        }
        Ok(())
    }
}

pub fn part1(input: &str) -> String {
    let (recipes, _) = parse_input(input);
    let mut state = State(vec![3, 7], [0, 1], [3, 7]);
    loop {
        state.extend_list(0);
        if state.0.len() >= recipes + 10 { break }
        state.choose_next_recipes();
    }
    let value = state.0[recipes..].iter().take(10).fold(0, |v, &r| v * 10 + r as u64);
    format!("{value:010}")
}

pub fn part2(input: &str) -> usize {
    let (_, target) = parse_input(input);
    let target_last = *target.last().unwrap();
    let mut state = State(vec![3, 7], [0, 1], [3, 7]);
    loop {
        if state.extend_list(target_last) {
            if let Some(idx) = state.index_of_last_2(&target) {
                return idx;
            }
        }
        state.choose_next_recipes();
    }
}

fn parse_input(input: &str) -> (usize, Vec<u8>) {
    (
        input.trim().parse::<usize>().unwrap(),
        input.trim().as_bytes().iter().map(|c| c - b'0').collect(),
    )
}
