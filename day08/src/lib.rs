aoc_tools::aoc_sol!(day08: part1, part2);

pub fn part1(input: &str) -> usize {
    let map = parse_input(input);

    let all_antinodes: HashSet<_> = map.all_antinodes_p1().values().flatten().copied().collect();
    all_antinodes.len()
}

pub fn part2(input: &str) -> usize {
    let map = parse_input(input);

    let all_antinodes: HashSet<_> = map.all_antinodes_p2().values().flatten().copied().collect();
    all_antinodes.len()
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

    pub fn antinodes_for_p1(&self, a: Pos, b: Pos) -> impl Iterator<Item = Pos> {
        let test_a = a.sub(b).add(a);
        let test_b = b.sub(a).add(b);
        let mut positions = [None, None];
        if self.has_pos(test_a) { positions[0] = Some(test_a) }
        if self.has_pos(test_b) { positions[1] = Some(test_b) }
        
        positions.into_iter().flatten()
    }
    pub fn antinodes_for_p2(&self, a: Pos, b: Pos) -> impl Iterator<Item = Pos> {
        let step_a = a.sub(b).simplify();
        let step_b = b.sub(a).simplify();        

        let mut positions = HashSet::new();
        let mut test_a = a;
        while self.has_pos(test_a) {
            positions.insert(test_a);
            test_a = test_a.add(step_a);
        }

        let mut test_b = b;
        while self.has_pos(test_b) {
            positions.insert(test_b);
            test_b = test_b.add(step_b);
        }

        positions.into_iter()
    }

    pub fn antinodes_for<O: Iterator<Item = Pos>>(
        &self,
        f: Freq,
        gen_possible_antinodes: impl Fn(&Self, Pos, Pos) -> O,
    ) -> HashSet<Pos> {
        let mut positions = HashSet::new();
        let Some(freq_matches) = self.antennae.get(&f) else {
            return positions;
        };
        for &pos_a in freq_matches {
            for &pos_b in freq_matches {
                if pos_a == pos_b {
                    continue;
                }

                for antinode in gen_possible_antinodes(self, pos_a, pos_b) {
                    positions.insert(antinode);
                }
            }
        }

        positions
    }

    fn all_antinodes<O: Iterator<Item = Pos>>(
        &self,
        gen_possible_antinodes: impl Fn(&Self, Pos, Pos) -> O + Clone + Copy,
    ) -> HashMap<Freq, HashSet<Pos>> {
        self.antennae.keys()
            .map(|&k| (
                k,
                self.antinodes_for(k, gen_possible_antinodes),
            ))
            .collect()
    }

    pub fn all_antinodes_p1(&self) -> HashMap<Freq, HashSet<Pos>> {
        self.all_antinodes(Self::antinodes_for_p1)
    }
    pub fn all_antinodes_p2(&self) -> HashMap<Freq, HashSet<Pos>> {
        self.all_antinodes(Self::antinodes_for_p2)
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

aoc_tools::pos!(isize);

impl Pos {
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
