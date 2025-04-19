use std::{collections::BTreeSet, fmt::Debug, hash::Hash, ops::Index};

use crate::network::{Network, Valve, ValveSet};

// enum Action { Open(Valve), MoveTo(Valve) }
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Agents { None, One(Valve), Two(Valve, Valve) }
impl Agents {
    pub fn iter(&self) -> impl Iterator<Item = Valve> {
        match *self {
            Self::None => [None, None],
            Self::One(v) => [Some(v), None],
            Self::Two(a, b) => [Some(a), Some(b)]
        }.into_iter().filter_map(|v| v)
    }
    pub fn push(&mut self, v: Valve) {
        *self = match *self {
            Self::None => Self::One(v),
            Self::One(a) => Self::Two(a, v),
            Self::Two(a, b) => Self::Two(a.min(b), a.max(b))
        };
    }
    pub fn len(&self) -> usize {
        match self {
            Self::None => 0,
            Self::One(_) => 1,
            Self::Two(_, _) => 2,
        }
    }
}
impl Index<usize> for Agents {
    type Output = Valve;
    fn index(&self, index: usize) -> &Self::Output {
        match (self, index) {
            (Self::None, n) => panic!("Index {n} out of bounds for empty agent set"),
            (Self::One(v), 0) => v,
            (Self::One(_), n) => panic!("Index {n} out of bounds for 1 agent"),
            (Self::Two(a, _), 0) => a,
            (Self::Two(_, b), 1) => b,
            (Self::Two(_, _), n) => panic!("Index {n} out of bounds for 2 agents"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct State {
    agents: Agents,
    opened: ValveSet,
    curr_flow: u32,
    flowed: u32,
    time_left: u8,
    upper_bound: u32,
    prev: Option<Agents>,
}
impl State {
    pub fn part1(n: &Network) -> Self {
        let start = n.to_valve('A', 'A');
        let mut output = Self {
            agents: Agents::One(start),
            opened: ValveSet::empty(),
            curr_flow: 0,
            flowed: 0,
            time_left: 30,
            upper_bound: 0,
            prev: None,
        };
        output._set_upper_bound(n);
        output
    }
    pub fn part2(n: &Network) -> Self {
        let start = n.to_valve('A', 'A');
        let mut output = Self {
            agents: Agents::Two(start, start),
            opened: ValveSet::empty(),
            curr_flow: 0,
            flowed: 0,
            time_left: 26,
            upper_bound: 0,
            prev: None,
        };
        output._set_upper_bound(n);
        output
    }
    #[inline(never)]
    pub fn advance(self, n: &Network) -> impl Iterator<Item = State> + '_ {
        let mut new_states = if self.time_left == 0 {
            vec![]
        } else {
            vec![Self {
                agents: Agents::None,
                opened: self.opened,
                curr_flow: self.curr_flow,
                flowed: self.flowed + self.curr_flow,
                time_left: self.time_left - 1,
                upper_bound: 0,
                prev: Some(self.agents),
            }]
        };
        for (idx, agent) in self.agents.iter().enumerate() {
            let node = n.get(agent).unwrap();
            let per_state = node.connections().count();
            let mut augmented_new_states = Vec::with_capacity(per_state * new_states.len());
            for new_state in new_states {
                for new_agent in node.connections() {
                    // Agent can't go back to where it was before without doing
                    // anything (will never be the ONLY optimal solution)
                    if let Some(prev_agent) = self.prev {
                        if prev_agent[idx] == new_agent { continue }
                    }
                    let mut new_state = new_state;
                    new_state.agents.push(new_agent);
                    augmented_new_states.push(new_state);
                }
                if node.flow_rate() > 0 && !new_state.opened.contains(agent) {
                    let mut new_state = new_state;
                    new_state.agents.push(agent);
                    new_state.curr_flow += node.flow_rate();
                    new_state.opened.insert(agent);
                    augmented_new_states.push(new_state);
                }
            }
            new_states = augmented_new_states;
        }
        // for state in new_states.iter_mut() {
            // state._set_upper_bound(n);
        // }
        new_states.into_iter()
    }
    pub fn lower_bound(&self) -> u32 {
        self.flowed + self.curr_flow * self.time_left as u32
    }
    #[inline(never)]
    pub fn _set_upper_bound(&mut self, n: &Network) {
        let mut remaining = n.opening_chain.as_slice();
        let mut time_left = self.time_left;
        let mut extra_flow = 0;
        let mut total_extra = 0;
        while !remaining.is_empty() {
            if time_left > 0 {
                total_extra += extra_flow;
                for _ in 0..self.agents.len() {
                    loop {
                        if remaining.is_empty() { break }
                        let (v, new_flow) = remaining[0];
                        remaining = &remaining[1..];
                        if self.opened.contains(v) { continue }
                        extra_flow += new_flow;
                        break
                    }
                }
                time_left -= 1;
            } else { break }
            if time_left > 0 {
                total_extra += extra_flow;
                time_left -= 1;
            } else { break }
        }
        total_extra += extra_flow * time_left as u32;
        self.upper_bound = total_extra + self.lower_bound();
    }
    pub fn upper_bound(&self) -> u32 {
        self.upper_bound
    }

    pub fn debug_with<'a, 'b>(&'a self, n: &'b Network) -> StateDebugger<'a, 'b> {
        StateDebugger(self, n)
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.opened == other.opened &&
        self.agents == other.agents &&
        self.flowed == other.flowed &&
        self.time_left == other.time_left
    }
}
impl Eq for State {}
impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.opened.hash(state);
        self.agents.hash(state);
        self.flowed.hash(state);
        self.time_left.hash(state);
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
            .then_with(|| self.upper_bound.cmp(&other.upper_bound))
            .then_with(|| self.lower_bound().cmp(&other.lower_bound()))
            .then_with(|| self.time_left.cmp(&other.time_left))
            .then_with(|| self.curr_flow.cmp(&other.curr_flow))
            .then_with(|| self.opened.cmp(&other.opened))
            .then_with(|| self.agents.iter().cmp(other.agents.iter()))
    }
}

pub struct StateDebugger<'a, 'b>(&'a State, &'b Network);
impl Debug for StateDebugger<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct AgentIter<'a>(Agents, &'a Network);
        impl Debug for AgentIter<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[")?;
                match self.0 {
                    Agents::None => {},
                    Agents::One(a) => {
                        let ident = self.1.to_ident(a);
                        write!(f, "{}{}", ident[0], ident[1])?;
                    }
                    Agents::Two(a, b) => {
                        let ident = self.1.to_ident(a);
                        write!(f, "{}{}", ident[0], ident[1])?;
                        let ident = self.1.to_ident(b);
                        write!(f, ", {}{}", ident[0], ident[1])?;
                    }
                }
                write!(f, "]")
            }
        }
        f.debug_struct("State")
            .field("agents", &AgentIter(self.0.agents, &self.1))
            .field("opened", &self.0.opened.debug_with(self.1))
            .field("curr_flow", &self.0.curr_flow)
            .field("flowed", &self.0.flowed)
            .field("time_left", &self.0.time_left)
            .finish()
    }
}


#[derive(Clone)]
pub struct Solver {
    pub queue: BTreeSet<State>,
    pub best_lower_bound: u32,
}
impl Solver {
    pub fn part1(n: &Network) -> Self {
        Self {
            queue: [State::part1(n)].into_iter().collect(),
            best_lower_bound: 0,
        }
    }
    pub fn part2(n: &Network) -> Self {
        Self {
            queue: [State::part2(n)].into_iter().collect(),
            best_lower_bound: 0,
        }
    }
    pub fn advance(&mut self, n: &Network) -> bool {
        if let Some(state) = self.queue.pop_last() {
            // If the upper bound is less than the best seen lower bound, we
            // know this can't be the best path.
            // Additionally, because of how `State`'s Ord impl is defined, we
            // know that the last item MUST have the highest upper bound, so we
            // can stop processing
            if state.upper_bound() < self.best_lower_bound {
                self.queue.clear();
                return false;
            }
            for mut next_state in state.advance(n) {
                let lower_bound = next_state.lower_bound();
                if lower_bound > self.best_lower_bound {
                    self.best_lower_bound = lower_bound;
                }
                // If it's at the end, don't push to handling queue
                if next_state.time_left == 0 { continue }

                // Calculate the upper bound
                next_state._set_upper_bound(n);
                let upper_bound = next_state.upper_bound();

                // If lower bound is same as upper bound, skip because it can't
                // get better
                if lower_bound == upper_bound { continue }

                // If the upper bound is less than the best lower bound, we know
                // this can't be the best path
                if upper_bound < self.best_lower_bound { continue }

                // Push this to the handling queue
                self.queue.insert(next_state);
            }
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn debug_with<'a, 'b>(&'a self, n: &'b Network) -> SolverDebugger<'a, 'b> {
        SolverDebugger(self, n)
    }
}

pub struct SolverDebugger<'a, 'b>(&'a Solver, &'b Network);
impl Debug for SolverDebugger<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Solver")
            .field_with("states", |f| {
                f.debug_list()
                    .entries(self.0.queue.iter().map(|s| s.debug_with(self.1)))
                    .finish()
            })
            .field("best_lower_bound", &self.0.best_lower_bound)
            .finish()
    }
}