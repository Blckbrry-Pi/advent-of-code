use std::{collections::{BTreeMap, BTreeSet}, fmt::Debug, hash::Hash};

use crate::network::{Network, PackedValve, Valve, ValveSet};

// enum Action { Open(Valve), MoveTo(Valve) }
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Agents { None, One(PackedValve), Two(PackedValve, PackedValve) }
impl Agents {
    pub fn iter(&self) -> impl Iterator<Item = Valve> {
        match *self {
            Self::None => [None, None],
            Self::One(v) => [Some(v), None],
            Self::Two(a, b) => [Some(a), Some(b)]
        }.into_iter().filter_map(|v| v).map(|v| v.unpacked())
    }
    pub fn push(&mut self, v: Valve) {
        *self = match *self {
            Self::None => Self::One(v.packed()),
            Self::One(a) => Self::Two(a, v.packed()),
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

    pub fn get(&self, idx: usize) -> Valve {
        match (self, idx) {
            (Self::None, n) => panic!("Index {n} out of bounds for empty agent set"),
            (Self::One(v), 0) => v,
            (Self::One(_), n) => panic!("Index {n} out of bounds for 1 agent"),
            (Self::Two(a, _), 0) => a,
            (Self::Two(_, b), 1) => b,
            (Self::Two(_, _), n) => panic!("Index {n} out of bounds for 2 agents"),
        }.unpacked()
    }
}

#[derive(Clone, Copy)]
pub struct State {
    opened: ValveSet,
    curr_flow: u16,

    flowed: u16,
    time_left: u8,
    upper_bound: u16,

    agents: Agents,
    prev: Agents,
}
impl State {
    pub fn part1(n: &Network) -> Self {
        let start = n.to_valve('A', 'A');
        let mut output = Self {
            agents: Agents::One(start.packed()),
            opened: ValveSet::empty(),
            curr_flow: 0,
            flowed: 0,
            time_left: 30,
            upper_bound: 0,
            prev: Agents::None,
        };
        output._set_upper_bound(n);
        output
    }
    pub fn part2(n: &Network) -> Self {
        let start = n.to_valve('A', 'A');
        let mut output = Self {
            agents: Agents::Two(start.packed(), start.packed()),
            opened: ValveSet::empty(),
            curr_flow: 0,
            flowed: 0,
            time_left: 26,
            upper_bound: u16::MAX,
            prev: Agents::None,
        };
        output._set_upper_bound(n);
        output
    }
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
                upper_bound: u16::MAX,
                prev: self.agents,
            }]
        };
        for (idx, agent) in self.agents.iter().enumerate() {
            let node = n.get(agent).unwrap();
            let per_state = node.connectivity() as usize;
            let mut augmented_new_states = Vec::with_capacity(per_state * new_states.len());
            for new_state in new_states {
                for new_agent in node.connections() {
                    // Agent can't go back to where it was before without doing
                    // anything (will never be the ONLY optimal solution)
                    if self.prev != Agents::None {
                        if self.prev.get(idx) == new_agent { continue }
                    }
                    // let new_node = n.get(new_agent).unwrap();
                    // if new_node.connectivity() == 1 {
                    //     let already_opened = new_state.opened.contains(new_agent);
                    //     let is_zero = new_node.flow_rate() == 0;
                    //     if already_opened || is_zero {
                    //         continue;
                    //     }
                    // }
                    let mut new_state = new_state;
                    new_state.agents.push(new_agent);
                    augmented_new_states.push(new_state);
                }

                // It's never the only optimal solution to open a valve with a
                // flow rate of 0
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
        new_states.into_iter()
    }
    pub fn lower_bound(&self) -> u16 {
        self.flowed + self.curr_flow * self.time_left as u16
    }
    pub fn _set_upper_bound(&mut self, n: &Network) {
        if self.upper_bound != u16::MAX { return }
        let mut remaining = n.opening_chain.as_slice();
        let mut time_left = self.time_left;
        let mut extra_flow = 0;
        let mut total_extra = 0;

        // If an agent is at a zero-gain cell, (i.e. at a cell where spending
        // time to open it will not gain benefit,) they must move before opening
        // something, making them a "second agent" rather than a "first agent"
        let (agents_first, agents_second) = {
            let mut agents_first = 0;
            let mut agents_second = 0;
            for agent in self.agents.iter() {
                if self.opened.contains(agent) {
                    agents_second += 1;
                    continue
                }
                if n.get(agent).unwrap().flow_rate() > 0 {
                    agents_first += 1;
                } else {
                    agents_second += 1;
                }
            }
            (agents_first, agents_second)
        };

        let mut is_first = true;
        while !remaining.is_empty() && time_left > 0 {
            total_extra += extra_flow;
            let agent_count = if is_first { agents_first } else { agents_second };
            for _ in 0..agent_count {
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
            is_first = !is_first;
        }
        total_extra += extra_flow * time_left as u16;
        self.upper_bound = total_extra + self.lower_bound();
    }
    pub fn upper_bound(&self) -> u16 { self.upper_bound }

    pub fn debug_with<'a, 'b>(&'a self, n: &'b Network) -> StateDebugger<'a, 'b> {
        StateDebugger(self, n)
    }

    pub fn ordering_key(&self) -> u128 {
        let mut output = 0;
        output <<= 16;
        output |= self.upper_bound as u128;
        output <<= 16;
        output |= self.lower_bound() as u128;
        output <<= 8;
        output |= self.time_left as u128;
        output <<= 16;
        output |= self.curr_flow as u128;
        output <<= 60;
        output |= self.opened.inner() as u128;
        output <<= 6;
        if self.agents.len() > 0 { output |= self.agents.get(0).packed().inner() as u128; }
        output <<= 6;
        if self.agents.len() > 1 { output |= self.agents.get(1).packed().inner() as u128; }
        output
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
        match self.upper_bound.cmp(&other.upper_bound) {
            std::cmp::Ordering::Equal => (),
            o => return o,
        }
        match self.lower_bound().cmp(&other.lower_bound()) {
            std::cmp::Ordering::Equal => (),
            o => return o,
        }
        match self.time_left.cmp(&other.time_left) {
            std::cmp::Ordering::Equal => (),
            o => return o,
        }
        match self.curr_flow.cmp(&other.curr_flow) {
            std::cmp::Ordering::Equal => (),
            o => return o,
        }
        match self.opened.cmp(&other.opened) {
            std::cmp::Ordering::Equal => (),
            o => return o,
        }
        match self.agents.iter().cmp(other.agents.iter()) {
            std::cmp::Ordering::Equal => (),
            o => return o,
        }
        std::cmp::Ordering::Equal
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
                        let ident = self.1.to_ident(a.unpacked());
                        write!(f, "{}{}", ident[0], ident[1])?;
                    }
                    Agents::Two(a, b) => {
                        let ident = self.1.to_ident(a.unpacked());
                        write!(f, "{}{}", ident[0], ident[1])?;
                        let ident = self.1.to_ident(b.unpacked());
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
    pub queue: BTreeMap<u128, State>,
    pub best_lower_bound: u16,
}
impl Solver {
    pub fn part1(n: &Network) -> Self {
        Self {
            queue: [State::part1(n)].into_iter().map(|s| (s.ordering_key(), s)).collect(),
            best_lower_bound: 0,
        }
    }
    pub fn part2(n: &Network) -> Self {
        Self {
            queue: [State::part2(n)].into_iter().map(|s| (s.ordering_key(), s)).collect(),
            best_lower_bound: 0,
        }
    }
    pub fn advance(&mut self, n: &Network) -> bool {
        let Some((_, state)) = self.queue.pop_last() else { return false };

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
            self.queue.insert(next_state.ordering_key(), next_state);
        }

        true
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
                    .entries(self.0.queue.iter().map(|(_, s)| s.debug_with(self.1)))
                    .finish()
            })
            .field("best_lower_bound", &self.0.best_lower_bound)
            .finish()
    }
}