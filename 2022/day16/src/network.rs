use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug)]
pub struct Network {
    pub nodes: HashMap<Ident, Node>,
}
impl Network {
    pub fn get(&self, ident: Ident) -> &Node {
        self.nodes.get(&ident).unwrap()
    }

    pub fn start_state_p1(&self) -> State {
        State::new(vec![Ident(['A', 'A'])], 0)
    }

    pub fn start_state_p2(&self) -> State {
        State::new(vec![
            Ident(['A', 'A']),
            Ident(['A', 'A']),
        ], 4)
    }

    pub fn find_best(&self, start: State) -> i64 {
        if start.open.len() < 3 {
            println!("{start:?}");
        }
        if start.minutes_passed >= 30 {
            return start.pressure_escaped;
        }

        let mut curr_best = None;
        for new_state in start.advance(self) {
            let new = self.find_best(new_state);
            if let Some(best) = curr_best {
                if new > best {
                    curr_best = Some(new);
                }
            } else {
                curr_best = Some(new);
            }
        }

        curr_best.unwrap()
    }
}
impl FromStr for Network {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nodes = s.split('\n')
            .map(|line| line.parse::<Node>())
            .map(|node| node.map(|n| (n.ident(), n)))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            nodes,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident([char; 2]);
impl Ident {
    pub fn string(&self) -> String {
        self.0.iter().collect()
    }
}
impl Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string())
    }
}
impl FromStr for Ident {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<_>>();
        if chars.len() != 2 {
            Err("ident must be 2 characters".to_string())
        } else {
            Ok(Self([chars[0], chars[1]]))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    ident: Ident,
    flow_rate: i64,
    connections: Vec<Ident>,
}
impl Node {
    pub fn ident(&self) -> Ident {
        self.ident
    }
    pub fn flow_rate(&self) -> i64 {
        self.flow_rate
    }
    pub fn connections(&self) -> &[Ident] {
        &self.connections
    }
}
impl FromStr for Node {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Valve ").ok_or("missing Valve".to_string())?;
        let Some((ident, s)) = s.split_once(" has flow rate=") else {
            return Err("missing flow rate".to_string());
        };

        let ident = ident.parse()?;

        let (flow_rate, s) = s.split_once("; tunnels lead to valves ")
            .or(s.split_once("; tunnel leads to valve "))
            .ok_or("missing connections".to_string())?;

        let flow_rate = flow_rate.parse::<i64>().map_err(|e| e.to_string())?;

        let connections = s.split(", ")
            .map(|s| s.parse())
            .collect::<Result<Vec<Ident>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(Self {
            ident,
            flow_rate,
            connections,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    open: Vec<Ident>,
    actors: Vec<Ident>,
    pressure_escaped: i64,
    flow_rate: i64,
    minutes_passed: i64,
}
impl State {
    pub fn new(actors: Vec<Ident>, minutes_passed: i64) -> Self {
        Self {
            open: vec![],
            actors,
            pressure_escaped: 0,
            flow_rate: 0,
            minutes_passed,
        }
    }

    pub fn advance(&self, network: &Network) -> impl Iterator<Item = Self> + '_ {
        let mut new_states = vec![];

        let actors_move_to: Vec<Vec<Ident>> = self.actors.iter()
            .map(|&ident| network.get(ident).connections.clone())
            .collect();

        let possibilities: Vec<usize> = (0..actors_move_to.len())
            .map(|i| actors_move_to.iter().skip(i).map(|v| v.len()).product())
            .collect();

        for i in 0..possibilities[0] {
            let mut new_actors = self.actors.clone();
            for j in 0..actors_move_to.len() {
                // println!("{actors_move_to:?}");
                let possibilities = if j == possibilities.len() - 1 {
                    1
                } else {
                    possibilities[j]
                };

                let possibility_idx = (i / possibilities) % actors_move_to[j].len();
                let ident = actors_move_to[j][possibility_idx];
                new_actors[j] = ident;
            }

            let opened: HashSet<_> = self.open.iter()
                .chain(self.actors.iter())
                .copied()
                .collect();
            let mut opened: Vec<_> = opened.iter().copied().collect();
            opened.sort();
            
            let flow_rate = opened.iter()
                .map(|&ident| network.get(ident).flow_rate)
                .sum();
            
            let new_state = State {
                open: opened,
                actors: new_actors,
                pressure_escaped: self.pressure_escaped + self.flow_rate,
                flow_rate,
                minutes_passed: self.minutes_passed + 1,
            };

            new_states.push(new_state);
        }

        // println!("new: {new_states:?}");

        new_states.into_iter()
    }
}
