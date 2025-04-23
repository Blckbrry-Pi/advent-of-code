aoc_tools::aoc_sol!(day08 2024: part1, part2);
type Scalar = i16;
aoc_tools::pos!(Scalar);

pub fn part1(input: &str) -> usize {
    let map = parse_input(input);

    map.all_antinodes_p1().len()
}

pub fn part2(input: &str) -> usize {
    let map = parse_input(input);

    map.all_antinodes_p2().len()
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
                        .insert(Pos { x: x as Scalar, y: y as Scalar });
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
        0 <= p.x && p.x < self.width as Scalar &&
        0 <= p.y && p.y < self.height as Scalar
    }

    pub fn antinodes_for_p1(&self, a: Pos, b: Pos) -> impl Iterator<Item = Pos> {
        let test_a = a.sub(b).add(a);
        let test_b = b.sub(a).add(b);
        let mut positions = [None, None];
        if self.has_pos(test_a) { positions[0] = Some(test_a) }
        if self.has_pos(test_b) { positions[1] = Some(test_b) }
        
        positions.into_iter().flatten()
    }
    pub fn antinodes_for_p2<'a>(&'a self, a: Pos, b: Pos) -> impl Iterator<Item = Pos> + 'a {
        let step_a = a.sub(b).simplify();
        let step_b = b.sub(a).simplify();        

        let step_a_iter = (0..)
            .map(move |s| a.add(step_a.mul(s)))
            .map_while(|pos| self.has_pos(pos).then_some(pos));
        let step_b_iter = (0..)
            .map(move |s| b.add(step_b.mul(s)))
            .map_while(|pos| self.has_pos(pos).then_some(pos));

        step_a_iter.chain(step_b_iter)
    }

    pub fn antinodes_for<'a, O: Iterator<Item = Pos> + 'a>(
        &'a self,
        f: Freq,
        gen_possible_antinodes: &'a impl Fn(&'a Self, Pos, Pos) -> O,
    ) -> impl Iterator<Item = Pos> + 'a {
        let freq_matches = self.antennae.get(&f).unwrap();
        freq_matches.iter()
            .flat_map(move |&a| freq_matches.iter().filter(move |&&b| a != b).map(move |&b| (a, b)))
            .flat_map(move |(a, b)| {
                gen_possible_antinodes(self, a, b)
            })
    }

    fn all_antinodes<'a, O: Iterator<Item = Pos> + 'a>(
        &'a self,
        gen_possible_antinodes: &'a impl Fn(&'a Self, Pos, Pos) -> O,
    ) -> impl Iterator<Item = Pos> + 'a {
        self.antennae.keys()
            .flat_map(move |&k| self.antinodes_for(k, gen_possible_antinodes))
    }

    pub fn all_antinodes_p1(&self) -> HashSet<Pos> {
        self.all_antinodes(&Self::antinodes_for_p1).collect()
    }
    pub fn all_antinodes_p2(&self) -> HashSet<Pos> {
        self.all_antinodes(&Self::antinodes_for_p2).collect()
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.height {
            for c in 0..self.width {
                let pos = Pos { x: c as Scalar, y: r as Scalar };
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

impl Pos {
    pub fn simplify(&self) -> Self {
        fn gcd(a: Scalar, b: Scalar) -> Scalar {
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
