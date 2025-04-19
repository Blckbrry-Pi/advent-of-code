use std::collections::HashMap;
use std::fmt::Debug;
use std::num::NonZeroU64;
use std::str::FromStr;

pub struct Network {
    pub nodes: HashMap<Valve, Node>,
    pub lookup: HashMap<[char; 2], Valve>,
    pub reverse_lookup: HashMap<Valve, [char; 2]>,
    pub opening_chain: Vec<(Valve, u32)>,
}
impl Network {
    pub fn get(&self, valve: Valve) -> Option<&Node> {
        self.nodes.get(&valve)
    }
    pub fn to_valve(&self, a: char, b: char) -> Valve {
        *self.lookup.get(&[a, b]).unwrap()
    }
    pub fn to_ident(&self, v: Valve) -> [char; 2] {
        *self.reverse_lookup.get(&v).unwrap()
    }
}
impl Debug for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Network ")?;
        let mut network = f.debug_map();
        for (&valve, &node) in &self.nodes {
            let ident = self.to_ident(valve);
            struct IdentFormatter([char; 2]);
            impl Debug for IdentFormatter {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}{}", self.0[0], self.0[1])
                }
            }
            network.entry(&IdentFormatter(ident), &NodeDebugger(node, &self.lookup));
        }
        network.finish()
    }
}
impl FromStr for Network {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lookup = HashMap::new();
        let nodes: Vec<_> = s.split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|line| Node::from_str(line, &mut lookup))
            .collect::<Result<_, _>>()?;

        let nodes: HashMap<_, _> = nodes.into_iter()
            .map(|(ident, node)| {
                if let Some(valve) = lookup.get(&ident) {
                    (*valve, node)
                } else {
                    for valve in ValveSet(u64::MAX) {
                        'a: {
                            for taken_valve in lookup.values() {
                                if taken_valve == &valve {
                                    break 'a;
                                }
                            }
                            return (valve, node);
                        }
                    }
                    unreachable!("Too many valves");
                }
            })
            .collect();

        let reverse_lookup = lookup
            .iter()
            .map(|(a, b)| (*b, *a))
            .collect();

        let mut opening_chain: Vec<_> = nodes.iter()
            .map(|(valve, node)| (*valve, node.flow_rate))
            .collect();
        opening_chain.sort_by_key(|k| k.1);
        opening_chain.reverse();

        Ok(Self {
            nodes,
            lookup,
            reverse_lookup,
            opening_chain,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Valve(NonZeroU64);


pub struct ValveIter {
    conns: u64,
    i: u64,
}
impl Iterator for ValveIter {
    type Item = Valve;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            None
        } else if self.conns & self.i != 0 {
            let i = NonZeroU64::new(self.i).unwrap();
            let v = Valve(i);
            self.i = self.i.wrapping_shl(1);
            Some(v)
        } else {
            self.i = self.i.wrapping_shl(1);
            self.next()
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValveSet(u64);
impl ValveSet {
    pub fn empty() -> Self {
        Self(0)
    }
    pub fn contains(&self, valve: Valve) -> bool {
        (self.0 & valve.0.get()) != 0
    }
    pub fn insert(&mut self, valve: Valve) {
        self.0 |= valve.0.get();
    }
    pub fn debug_with<'a>(&self, n: &'a Network) -> ValveSetDebugger<'a> {
        ValveSetDebugger(*self, n)
    }
}
impl IntoIterator for ValveSet {
    type IntoIter = ValveIter;
    type Item = Valve;
    fn into_iter(self) -> Self::IntoIter {
        ValveIter { conns: self.0, i: 1 }
    }
}

pub struct ValveSetDebugger<'a>(ValveSet, &'a Network);
impl Debug for ValveSetDebugger<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut set = f.debug_set();
        for valve in self.0 {
            let ident = self.1.to_ident(valve);
            set.entry_with(|f| {
                write!(f, "{}", ident[0])?;
                write!(f, "{}", ident[1])
            });
        }
        set.finish()
    }
}

#[derive(Clone, Copy)]
pub struct Node {
    flow_rate: u32,
    connections: ValveSet,
}
impl Node {
    pub fn flow_rate(&self) -> u32 {
        self.flow_rate
    }
    pub fn connections(&self) -> impl Iterator<Item = Valve> {
        self.connections.into_iter()
    }
}
impl Node {
    fn from_str(s: &str, lookup: &mut HashMap<[char; 2], Valve>) -> Result<([char; 2], Self), String> {
        let s = s.strip_prefix("Valve ").ok_or("missing Valve".to_string())?;
        let Some((ident, s)) = s.split_once(" has flow rate=") else {
            return Err("missing flow rate".to_string());
        };

        let ident = [ident.as_bytes()[0] as char, ident.as_bytes()[1] as char];

        let (flow_rate, s) = s.split_once("; tunnels lead to valves ")
            .or(s.split_once("; tunnel leads to valve "))
            .ok_or("missing connections".to_string())?;

        let flow_rate = flow_rate.parse::<u32>().map_err(|e| e.to_string())?;

        let connections = s.split(", ")
            .map(|ident| [ident.as_bytes()[0] as char, ident.as_bytes()[1] as char])
            .map(|ident| {
                if let Some(valve) = lookup.get(&ident) {
                    *valve
                } else {
                    for valve in ValveSet(u64::MAX) {
                        'a: {
                            for taken_valve in lookup.values() {
                                if taken_valve == &valve {
                                    break 'a;
                                }
                            }
                            lookup.insert(ident, valve);
                            return valve;
                        }
                    }
                    unreachable!("Too many valves");
                }
            })
            .fold(ValveSet(0), |set, new_valve| ValveSet(set.0 | new_valve.0.get()));

        Ok((ident, Self {
            flow_rate,
            connections,
        }))
    }
}

struct NodeDebugger<'a>(Node, &'a HashMap<[char; 2], Valve>);
impl Debug for NodeDebugger<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "flow {}; ->", self.0.flow_rate)?;
        for valve in self.0.connections {
            let ident = 'a: {
                for (ident, v) in self.1 {
                    if &valve == v {
                        break 'a *ident;
                    }
                }
                unreachable!("Bad thingy :(");
            };
            write!(f, " {}{}", ident[0], ident[1])?;
        }
        Ok(())
    }
}
