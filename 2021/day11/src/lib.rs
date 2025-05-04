use std::str::FromStr;

aoc_tools::aoc_sol!(day11 2021: part1, part2);
aoc_tools::pos!(i16; +y => D);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Octopus(u8);
impl Octopus {
    pub fn increase(&mut self) -> bool {
        self.0 += 1;
        self.0 > 9
    }
}

#[derive(Clone)]
struct Map {
    rows: Vec<Vec<Octopus>>,
    flashed: HashSet<Pos>,
}
impl Map {
    pub fn count(&self) -> usize { self.rows.len() * self.rows[0].len() }
    pub fn get_mut(&mut self, pos: Pos) -> Option<&mut Octopus> {
        if !(0..self.rows.len()).contains(&(pos.y as usize)) { return None }
        let row = self.rows[pos.y as usize].as_mut_slice();
        if !(0..row.len()).contains(&(pos.x as usize)) { return None }
        Some(&mut row[pos.x as usize])
    }
    pub fn neighbors(pos: Pos) -> [Pos; 8] {
        [
            pos.add(Pos::W),
            pos.add(Pos::N).add(Pos::W),
            pos.add(Pos::N),
            pos.add(Pos::N).add(Pos::E),
            pos.add(Pos::E),
            pos.add(Pos::S).add(Pos::E),
            pos.add(Pos::S),
            pos.add(Pos::S).add(Pos::W),
        ]
    }
    pub fn flash_at(&mut self, pos: Pos) {
        if !self.flashed.insert(pos) { return }
        for neighbor_pos in Self::neighbors(pos) {
            let Some(neighbor) = self.get_mut(neighbor_pos) else { continue };
            if neighbor.increase() {
                self.flash_at(neighbor_pos);
            }
        }
    }
    fn do_step(&mut self) -> u32 {
        let h = self.rows.len();
        let w = self.rows[0].len();
        for y in 0..h {
            for x in 0..w {
                let pos = Pos {
                    x: x as i16,
                    y: y as i16,
                };
                let val = self.get_mut(pos).unwrap();
                if val.increase() {
                    self.flash_at(pos);
                }
            }
        }
        let output = self.flashed.len();
        for flashed in std::mem::take(&mut self.flashed) {
            self.get_mut(flashed).unwrap().0 = 0;
        }
        output as u32
    }
}
impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for col in row {
                write!(f, "{}", col.0)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl FromStr for Map {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<Octopus>> = s.trim()
            .lines()
            .map(
                |line| line.chars().map(|cell| {
                    Octopus(cell as u8 - b'0')
                }).collect(),
            )
            .collect();
        Ok(Self { rows, flashed: HashSet::new() })
    }
}

pub fn part1(input: &str) -> u32 {
    let mut map = parse_input(input);
    let mut total = 0;
    for _ in 0..100 {
        total += map.do_step();
    }
    total
}

pub fn part2(input: &str) -> u32 {
    let mut map = parse_input(input);
    let mut i = 1;
    while map.do_step() != map.count() as u32 {
        i += 1;
    }
    i
}

fn parse_input(input: &str) -> Map {
    input.parse().unwrap()
}
