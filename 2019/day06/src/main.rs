use std::fmt::Debug;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    part1();
    part2();
}

#[allow(dead_code)]
const TEST: &str = include_str!("../../../data/2019/day06/test.txt");
const INPUT: &str = include_str!("../../../data/2019/day06/input.txt");

fn part1() {
    let start = Instant::now();
    let system = parse_input(INPUT);

    let out = system.orbits();

    println!("Part 1: {out:?} {:?}", start.elapsed());
}

fn part2() {
    let start = Instant::now();
    let system = parse_input(INPUT);

    let from = system.orbit_center_for(Body::YOU).unwrap();
    let to = system.orbit_center_for(Body::SAN).unwrap();
    let out = system.bfs_len(from, to);

    println!("Part 2: {out:?} {:?}", start.elapsed());
}


fn parse_input(input: &'static str) -> System {
    let mut system = System::new();
    input.lines()
        .map(|orbit| orbit.split_once(')').unwrap())
        .map(|(center, orbiting_body)| (Body::from_str(center), Body::from_str(orbiting_body)))
        .for_each(|(center, orbiting_body)| system.add_orbit(center, orbiting_body));

    system
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Body(char, char, char);
impl Body {
    const COM: Self = Self('C', 'O', 'M');
    const SAN: Self = Self('S', 'A', 'N');
    const YOU: Self = Self('Y', 'O', 'U');

    pub fn from_str(str: &str) -> Self {
        let c0 = str.chars().next().unwrap_or('_');
        let c1 = str.chars().nth(1).unwrap_or('_');
        let c2 = str.chars().nth(2).unwrap_or('_');

        Self(c0, c1, c2)
    }
}
impl Debug for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}

#[derive(Debug, Clone)]
struct System {
    bodies: HashMap<Body, Option<Body>>,
    bodies_rev: HashMap<Body, Vec<Body>>,
}

impl System {
    pub fn new() -> Self {
        Self {
            bodies: [(Body::COM, None)].into_iter().collect(),
            bodies_rev: [(Body::COM, vec![])].into_iter().collect(),
        }
    }
    pub fn add_orbit(&mut self, center: Body, orbiting_body: Body) {
        self.bodies.entry(center).or_default();
        *self.bodies.entry(orbiting_body).or_default() = Some(center);

        self.bodies_rev.entry(center).or_default().push(orbiting_body);
        self.bodies_rev.entry(orbiting_body).or_default();
    }

    pub fn orbits_for(&self, body: Body) -> usize {
        let mut orbits = 0;
        let mut curr = body;

        while let Some(Some(center)) = self.bodies.get(&curr) {
            curr = *center;
            orbits += 1;
        }

        orbits
    }
    pub fn orbits(&self) -> usize {
        let mut orbits = 0;
        for &body in self.bodies.keys() {
            orbits += self.orbits_for(body);
        }
        orbits
    }

    pub fn bfs_len(&self, from: Body, to: Body) -> usize {
        let mut curr_bodies: HashSet<_> = [from].into_iter().collect();
        let mut new_bodies = HashSet::new();
        let mut len = 0;

        while !curr_bodies.contains(&to) {
            for body in curr_bodies {
                if let Some(&Some(orbiting_around)) = self.bodies.get(&body) {
                    new_bodies.insert(orbiting_around);
                }
                if let Some(sub_bodies) = self.bodies_rev.get(&body) {
                    new_bodies.extend(sub_bodies.iter().copied());
                }
            }

            curr_bodies = new_bodies;
            new_bodies = HashSet::new();
            len += 1;
        }

        len
    }

    pub fn orbit_center_for(&self, body: Body) -> Option<Body> {
        *self.bodies.get(&body).unwrap()
    }
}
