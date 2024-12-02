use std::collections::{HashMap, HashSet, VecDeque};

use crate::{component::Component, wire::Wire};

#[derive(Debug, Clone)]
pub struct System {
    components: HashSet<Component>,
    wires: HashSet<Wire>,
    wire_map: HashMap<Component, HashSet<Wire>>,
}

impl System {
    pub fn new() -> Self {
        System {
            components: HashSet::new(),
            wires: HashSet::new(),
            wire_map: HashMap::new(),
        }
    }
    pub fn add_wire(&mut self, wire: Wire) {
        let a = Component::new(wire.a());
        let b = Component::new(wire.b());

        self.components.insert(a);
        self.components.insert(b);

        self.wires.insert(wire);

        self.wire_map
            .entry(a)
            .or_insert_with(HashSet::new)
            .insert(wire.clone());
        self.wire_map
            .entry(b)
            .or_insert_with(HashSet::new)
            .insert(wire);
    }

    pub fn partitions(&self, ignore_wires: Vec<Wire>) -> Option<[Vec<Component>; 2]> {
        let mut visited = HashSet::new();
        let Some(&first) = self.components.iter().next() else { return None; };
        let mut stack: HashSet<_> = [first].into_iter().collect();

        while let Some(&component) = stack.iter().next() {
            stack.remove(&component);

            // println!("Checking component {:?}", component);
            if visited.contains(&component) {
                continue;
            }
            // println!("Component {:?} not visited", component);
            visited.insert(component);

            let wires = self.wire_map.get(&component).unwrap();
            for wire in wires {
                if ignore_wires.contains(&wire) {
                    continue;
                }

                let other = wire.other(component.name()).unwrap();
                stack.insert(Component::new(other));
            }
        }

        if visited.len() == self.components.len() {
            None
        } else {
            Some([
                self.components.difference(&visited).copied().collect(),
                visited.into_iter().collect(),
            ])
        }

    }

    pub fn path_from_bfs(&self, from: Component, to: Component, ignore: HashSet<Wire>) -> Option<Vec<Wire>> {
        let mut visited = HashSet::new();
        let mut stack: VecDeque<_> = [(vec![], from)].into_iter().collect();

        loop {
            let Some((history, next)) = stack.pop_front() else { return None; };

            if visited.contains(&next) {
                continue;
            }
            let new_history: Vec<_> = history.into_iter().chain([next]).collect();

            if next == to {
                let wires = new_history
                    .iter()
                    .zip(new_history.iter().skip(1))
                    .map(|(a, b)| Wire::new(a.name(), b.name())).collect();
                return Some(wires);
            }

            visited.insert(next);

            let wires = self.wire_map.get(&next).unwrap();

            for wire in wires {
                if ignore.contains(wire) {
                    continue;
                }

                let other = wire.other(next.name()).unwrap();
                let other = Component::new(other);

                let new_entry = (new_history.clone(), other);
                stack.push_back(new_entry);
            }
        }
    }

    pub fn wire_disconnects_needed(&self, from: Component, to: Component) -> (i32, HashSet<Wire>) {
        let mut ignored = HashSet::new();
        let mut path_removals = 0;
        while let Some(path) = self.path_from_bfs(from, to, ignored.clone()) {
            for wire in path {
                ignored.insert(wire);
            }
            path_removals += 1;
        }
        (path_removals, ignored)
    }

    pub fn wires(&self) -> impl Iterator<Item = Wire> + '_ {
        self.wires.iter().copied()
    }

    pub fn components(&self) -> impl Iterator<Item = Component> + '_ {
        self.components.iter().copied()
    }

    pub fn has_wire(&self, wire: &Wire) -> bool {
        self.wires.contains(wire)
    }
}
