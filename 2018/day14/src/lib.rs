aoc_tools::aoc_sol!(day14 2018: part1, part2);

struct State(Vec<u8>, [usize; 2]);
impl State {
    pub fn extend_list(&mut self) {
        let mut sum = self.0[self.1[0]] + self.0[self.1[1]];
        if sum >= 10 {
            self.0.push(1);
            sum -= 10;
        }
        self.0.push(sum);
    }
    pub fn choose_next_recipes(&mut self) {
        for i in 0..self.1.len() {
            self.1[i] += 1 + self.0[self.1[i]] as usize;
            self.1[i] %= self.0.len();
        }
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
    let mut state = State(vec![3, 7], [0, 1]);
    loop {
        state.extend_list();
        if state.0.len() >= recipes + 10 { break }
        state.choose_next_recipes();
    }
    let value = state.0[recipes..].iter().take(10).fold(0, |v, &r| v * 10 + r as u64);
    format!("{value:010}")
}

pub fn part2(input: &str) -> usize {
    let (_, target) = parse_input(input);
    let mut state = State(vec![3, 7], [0, 1]);
    loop {
        state.extend_list();
        if let Some(idx) = state.index_of_last_2(&target) {
            return idx;
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
