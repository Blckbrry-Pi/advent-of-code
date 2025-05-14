use std::str::FromStr;
aoc_tools::aoc_sol!(day25 2021: part1);

type Scalar = i16;
aoc_tools::pos!(Scalar; +y => D);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    CukeE,
    CukeS,
}
impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::CukeE => write!(f, ">"),
            Self::CukeS => write!(f, "v"),
        }
    }
}
impl FromStr for Tile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 { return Err("Tiles are 1 char long".to_string()) }        
        match s.as_bytes()[0] as char {
            '.' => Ok(Self::Empty),
            '>' => Ok(Self::CukeE),
            'v' => Ok(Self::CukeS),
            c => Err(format!("Invalid tile char: {c:?}"))
        }
    }
}

#[derive(Clone)]
struct Map {
    rows: Vec<Vec<Tile>>,
}
impl Map {
    pub fn step_e(&mut self) -> usize {
        let mut moves = 0;
        for row in self.rows.iter_mut() {
            let start_is_free = row[0] == Tile::Empty;
            let mut x = 0;
            while x < row.len()-1 {
                if row[x] == Tile::CukeE && row[x+1] == Tile::Empty {
                    row[x] = Tile::Empty;
                    row[x+1] = Tile::CukeE;
                    moves += 1;
                    x += 1;
                }
                x += 1;
            }

            if start_is_free && row.last().unwrap() == &Tile::CukeE && x < row.len() {
                *row.last_mut().unwrap() = Tile::Empty;
                row[0] = Tile::CukeE;
                moves += 1;
            }
        }
        moves
    }
    pub fn step_s(&mut self) -> usize {
        let mut moves = 0;
        for x in 0..self.rows[0].len() {
            let start_is_free = self.rows[0][x] == Tile::Empty;
            let mut y = 0;
            while y < self.rows.len()-1 {
                if self.rows[y][x] == Tile::CukeS && self.rows[y+1][x] == Tile::Empty {
                    self.rows[y][x] = Tile::Empty;
                    self.rows[y+1][x] = Tile::CukeS;
                    moves += 1;
                    y += 1;
                }
                y += 1;
            }
            if start_is_free && self.rows.last().unwrap()[x] == Tile::CukeS && y < self.rows.len() {
                self.rows.last_mut().unwrap()[x] = Tile::Empty;
                self.rows[0][x] = Tile::CukeS;
                moves += 1;
            }
        }
        moves
    }
    pub fn step(&mut self) -> usize {
        self.step_e() + self.step_s()
    }
}
impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for col in row {
                write!(f, "{col:?}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(
                |l| (0..l.len())
                    .map(|i| &l[i..i+1])
                    .map(Tile::from_str)
                    .collect::<Result<Vec<_>, _>>(),
            )
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { rows })
    }
}

pub fn part1(input: &str) -> u16 {
    let mut map = parse_input(input);
    let mut i = 1;
    while map.step() > 0 {
        i += 1;
    }
    i
}

pub fn part2(input: &str) -> usize { input.len() }

fn parse_input(input: &str) -> Map {
    input.parse().unwrap()
}
