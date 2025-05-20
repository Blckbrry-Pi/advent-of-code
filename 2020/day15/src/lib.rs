use std::num::NonZeroU32;

aoc_tools::aoc_sol!(day15 2020: part1, part2);

#[derive(Debug, Clone, Copy)]
enum NumHistory {
    SpokenOnce(NonZeroU32),
    SpokenMulti(NonZeroU32, NonZeroU32),
}

#[derive(Debug)]
struct State {
    lookups: HashMap<u32, NumHistory>,
    next_step_num: NonZeroU32,
    prev: u32,
}
impl State {
    pub fn start() -> Self {
        Self {
            lookups: HashMap::new(),
            next_step_num: NonZeroU32::new(1).unwrap(),
            prev: 0,
        }
    }
    pub fn get_next(&self) -> u32 {
        match self.lookups.get(&self.prev) {
            Some(NumHistory::SpokenOnce(_)) | None => 0,
            Some(NumHistory::SpokenMulti(prev, new)) => new.get() - prev.get(),
        }
    }
    pub fn add(&mut self, speak: u32) {
        if let Some(history) = self.lookups.get_mut(&speak) {
            match history {
                NumHistory::SpokenOnce(prev) => *history = NumHistory::SpokenMulti(*prev, self.next_step_num),
                NumHistory::SpokenMulti(_, prev) => *history = NumHistory::SpokenMulti(*prev, self.next_step_num),
            }
        } else {
            self.lookups.insert(speak, NumHistory::SpokenOnce(self.next_step_num));
        }
        self.next_step_num = self.next_step_num.checked_add(1).unwrap();
        self.prev = speak;
    }
    pub fn numbers_said(&self) -> u32 {
        self.next_step_num.get() - 1
    }
}

pub fn part1(input: &str) -> u32 {
    let starting_numbers = parse_input(input);
    let mut histories = State::start();
    for number in starting_numbers {
        histories.add(number);
    }
    while histories.numbers_said() < 2020 {
        histories.add(histories.get_next());
    }
    histories.prev
}

pub fn part2(input: &str) -> u32 {
    let starting_numbers = parse_input(input);
    let mut histories = State::start();
    for number in starting_numbers {
        histories.add(number);
    }
    while histories.numbers_said() < 30_000_000 {
        histories.add(histories.get_next());
    }
    histories.prev
}

fn parse_input(input: &str) -> Vec<u32> {
    input.trim().split(',').map(|v| v.parse().unwrap()).collect()
}
