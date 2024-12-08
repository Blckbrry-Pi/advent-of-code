use std::{collections::{HashMap, HashSet}, fmt::Debug};

fn main() {
    part1();
    part2();
}

const TEST: &str = include_str!("../../data/day8/test.txt");
const INPUT: &str = include_str!("../../data/day8/input.txt");

fn part1() {
    let start = std::time::Instant::now();
    let map = parse_input(INPUT);

    let all_antinodes: HashSet<_> = map.all_antinodes_p1().values().flatten().copied().collect();
    let out = all_antinodes.len();

    println!("Part 1: {} ({:?})", out, start.elapsed());
}

fn part2() {
    let start = std::time::Instant::now();
    let map = parse_input(INPUT);

    let all_antinodes: HashSet<_> = map.all_antinodes_p2().values().flatten().copied().collect();
    let out = all_antinodes.len();

    println!("Part 2: {} ({:?})", out, start.elapsed());
}

fn parse_input(input: &str) -> Map {
    let mut width = 0;
    let mut antennae: HashMap<Freq, HashSet<Pos>> = HashMap::new();
    input.lines()
        .enumerate()
        .for_each(|(y, row)| row.chars().enumerate().for_each(
            |(x, cell)| {
                width = width.max(x + 1);
                if cell != '.' {
                    antennae.entry(Freq(cell))
                        .or_default()
                        .insert(Pos { x: x as isize, y: y as isize });
                }
            }
        ));

    let height = input.lines().count();
        
    Map {
        antennae,
        width,
        height
    }
}


#[derive(Clone)]
struct Map {
    antennae: HashMap<Freq, HashSet<Pos>>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn has_pos(&self, p: Pos) -> bool {
        0 <= p.x && p.x < self.width as isize &&
        0 <= p.y && p.y < self.height as isize
    }
    pub fn antinodes_for_p1(&self, f: Freq) -> Vec<Pos> {
        let mut positions = vec![];
        let Some(freq_matches) = self.antennae.get(&f) else {
            return vec![];
        };
        for &pos_a in freq_matches {
            for &pos_b in freq_matches {
                if pos_a == pos_b {
                    continue;
                }

                let test = pos_a.sub(pos_b).add(pos_a);

                if self.has_pos(test) {
                    positions.push(test);
                }
            }
        }

        positions
    }

    pub fn all_antinodes_p1(&self) -> HashMap<Freq, Vec<Pos>> {
        self.antennae.keys()
            .map(|&k| (k, self.antinodes_for_p1(k)))
            .collect()
    }

    pub fn antinodes_for_p2(&self, f: Freq) -> Vec<Pos> {
        let mut positions = HashSet::new();
        let Some(freq_matches) = self.antennae.get(&f) else {
            return vec![];
        };
        for &pos_a in freq_matches {
            for &pos_b in freq_matches {
                if pos_a == pos_b {
                    continue;
                }

                let step = pos_a.sub(pos_b).simplify();

                let mut curr = pos_a;
                while self.has_pos(curr) {
                    positions.insert(curr);
                    curr = curr.add(step);
                }
            }
        }

        positions.into_iter().collect()
    }

    pub fn all_antinodes_p2(&self) -> HashMap<Freq, Vec<Pos>> {
        self.antennae.keys()
            .map(|&k| (k, self.antinodes_for_p2(k)))
            .collect()
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.height {
            for c in 0..self.width {
                let pos = Pos { x: c as isize, y: r as isize };
                let mut antenna_found = false;
                for (freq, antennae_positions) in self.antennae.iter() {
                    if antennae_positions.contains(&pos) {
                        write!(f, "{freq:?}")?;
                        antenna_found = true;
                        break
                    }
                }
                if !antenna_found {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Freq(char);
impl Debug for Freq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos { x: isize, y: isize }

impl Pos {
    pub fn add(&self, o: Self) -> Self {
        Self {
            x: self.x + o.x,
            y: self.y + o.y
        }
    }

    pub fn neg(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y
        }
    }

    pub fn sub(&self, o: Self) -> Self {
        self.add(o.neg())
    }

    pub fn simplify(&self) -> Self {
        fn gcd(a: isize, b: isize) -> isize {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        if self.x == 0 {
            Self {
                x: 0,
                y: self.y.signum(),
            }
        } else if self.y == 0 {
            Self {
                x: self.x.signum(),
                y: 0,
            }
        } else {
            let div_by = gcd(self.x, self.y).abs();
            Self {
                x: self.x / div_by,
                y: self.y / div_by,
            }
        }
    }
}
