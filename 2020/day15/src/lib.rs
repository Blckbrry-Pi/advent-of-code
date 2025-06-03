aoc_tools::aoc_sol!(day15 2020: part1, part2);

struct State {
    lookups: Vec<u32>,
    next_step_num: u32,
}
impl State {
    pub fn start(max_number: usize) -> Self {
        let lookups = vec![u32::MAX; max_number + 2];
        Self {
            lookups,
            next_step_num: 1,
        }
    }
    pub fn add(&mut self, speak: u32) -> u32 {
        assert!((speak as usize) < self.lookups.len(), "{speak} is not less than {} @ step {}", self.lookups.len(), self.next_step_num);
        
        let old_time = self.lookups[speak as usize];
        self.lookups[speak as usize] = self.next_step_num;
        
        let output =
            if old_time >= self.next_step_num { 0 }
            else { self.next_step_num - old_time };

        self.next_step_num += 1;
        output
    }
}
impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("History")
            .field("next_step_num", &self.next_step_num)
            .field("lookups", &&self.lookups[0..20 + self.next_step_num as usize])
            .finish()
    }
}

fn history_after_n(n: usize, starting: &[u32]) -> State {
    let mut history = State::start(n);
    let mut next = 0;
    for &number in starting {
        next = history.add(number);
    }
    while history.next_step_num <= n as u32 {
        next = history.add(next);
    }
    history
}

pub fn part1(input: &str) -> u32 {
    let starting_numbers = parse_input(input);
    history_after_n(2020, &starting_numbers).prev
}

pub fn part2(input: &str) -> u32 {
    let starting_numbers = parse_input(input);
    history_after_n(30_000_000, &starting_numbers).prev
}

fn parse_input(input: &str) -> Vec<u32> {
    input.trim().split(',').map(|v| v.parse().unwrap()).collect()
}
