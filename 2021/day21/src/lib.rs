use std::str::FromStr;
aoc_tools::aoc_sol!(day21 2021: part1, part2);

trait Die<const OPTIONS: usize, const OPTIONS_3: usize> {
    fn roll(&mut self) -> [u16; OPTIONS];
    fn roll_3(&mut self) -> [u16; OPTIONS_3] {
        let a = self.roll();
        let b = self.roll();
        let c = self.roll();
        std::array::from_fn(|i| {
            let a_i = i / OPTIONS / OPTIONS;
            let b_i = (i / OPTIONS) % OPTIONS;
            let c_i = i % OPTIONS;
            a[a_i] + b[b_i] + c[c_i]
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DeterministicDie(usize);
impl DeterministicDie {
    pub fn new() -> Self {
        Self(0)
    }
}
impl Die<1, 1> for DeterministicDie {
    fn roll(&mut self) -> [u16; 1] {
        self.0 += 1;
        let res = (self.0 % 100) as u16;
        [res]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DiracDie;
impl Die<3, 27> for DiracDie {
    fn roll(&mut self) -> [u16; 3] {
        [1, 2, 3]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct PlayerState {
    pos: u16,
    score: u16,
}
impl PlayerState {
    pub fn advance_one(mut self, by: u16) -> Self {
        self.pos += by % 10;
        if self.pos > 10 {
            self.pos -= 10;
        }
        self.score += self.pos;
        self
    }
    pub fn advance<const N: usize>(self, advance_by: [u16; N]) -> [Self; N] {
        advance_by.map(|by| self.advance_one(by))
    }
}
impl PartialOrd for PlayerState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
impl Ord for PlayerState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).then(self.pos.cmp(&other.pos))
    }
}

#[derive(Clone)]
struct GameState {
    p1_states: HashMap<PlayerState, u64>,
    p1_state_count: u64,
    p2_states: HashMap<PlayerState, u64>,
    p2_state_count: u64,
    is_p1_turn: bool,
}
impl GameState {
    pub fn take_turn<const O: usize, const O3: usize>(&mut self, die: &mut impl Die<O, O3>) {
        let rolls = die.roll_3();
        let (state_count, old_states) = if self.is_p1_turn {
            // std::mem::take(&mut self.p1_states)
            (self.p1_states.len(), self.p1_states.drain())
        } else {
            // std::mem::take(&mut self.p2_states)
            (self.p2_states.len(), self.p2_states.drain())
        };
        let mut new_state_count = 0;
        let mut new_states = HashMap::with_capacity(state_count * rolls.len().isqrt());
        for (state, count) in old_states {
            for new_state in state.advance(rolls) {
                *new_states.entry(new_state).or_default() += count;
                new_state_count += count;
            }
        }
        if self.is_p1_turn {
            self.p1_states = new_states;
            self.p1_state_count = new_state_count;
        } else {
            self.p2_states = new_states;
            self.p2_state_count = new_state_count;
        }
        self.is_p1_turn = !self.is_p1_turn;
    }
    pub fn remove_won(&mut self, threshold: u16) -> impl Iterator<Item = (bool, u64)> + '_ {
        let Self {
            p1_state_count,
            p2_state_count,
            p1_states,
            p2_states,
            is_p1_turn,
        } = self;

        let cmp_threshold = move |s: &PlayerState, _: &mut u64| s.score >= threshold;
        fn dec_by_count(to_dec: &mut u64) -> impl FnMut(&(PlayerState, u64)) + '_ {
            move |&(_, count): &(PlayerState, u64)| *to_dec -= count
        }
        let multiplier = |is_p1, multiplier| move |(_, count): (PlayerState, u64)| (is_p1, count * multiplier);

        if !*is_p1_turn {
            p1_states
                .extract_if(cmp_threshold)
                .inspect(dec_by_count(p1_state_count))
                .map(multiplier(true, *p2_state_count))
        } else {
            p2_states
                .extract_if(cmp_threshold)
                .inspect(dec_by_count(p2_state_count))
                .map(multiplier(false, *p1_state_count))
        }
    }
    pub fn get_score(&self) -> u16 {
        u16::MAX
            .min(self.p1_states.iter().next().map(|(state, _)| state.score).unwrap_or(u16::MAX))
            .min(self.p2_states.iter().next().map(|(state, _)| state.score).unwrap_or(u16::MAX))
    }
}
impl FromStr for GameState {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((p1, p2)) = s.split_once('\n') else {
            return Err("Expected 2 lines with the player starting positions on them".to_string());
        };
        let Some(p1) = p1.strip_prefix("Player 1 starting position: ") else {
            return Err("Invalid player 1 start spec format".to_string());
        };
        let Some(p2) = p2.strip_prefix("Player 2 starting position: ") else {
            return Err("Invalid player 2 start spec format".to_string());
        };
        let p2 = p2.trim();
        let p1 = p1.parse::<u16>().map_err(|e| e.to_string())?;
        let p2 = p2.parse::<u16>().map_err(|e| e.to_string())?;
        Ok(GameState {
            p1_states: [(PlayerState { pos: p1, score: 0 }, 1)].into_iter().collect(),
            p1_state_count: 1,
            p2_states: [(PlayerState { pos: p2, score: 0 }, 1)].into_iter().collect(),
            p2_state_count: 1,
            is_p1_turn: true,
        })
    }
}

pub fn part1(input: &str) -> u32 {
    let mut state = parse_input(input);
    let mut die = DeterministicDie::new();
    while state.remove_won(1000).next().is_none() {
        state.take_turn(&mut die);
    }
    die.0 as u32 * state.get_score() as u32
}

pub fn part2(input: &str) -> u64 {
    let mut state = parse_input(input);
    let mut die = DiracDie;
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    while !state.p1_states.is_empty() && !state.p2_states.is_empty() {
        state.take_turn(&mut die);
        for (is_p1, count) in state.remove_won(21) {
            if is_p1 {
                p1_wins += count;
            } else {
                p2_wins += count;
            }
        }
    }
    p1_wins.max(p2_wins)
}

fn parse_input(input: &str) -> GameState {
    input.parse().unwrap()
}
