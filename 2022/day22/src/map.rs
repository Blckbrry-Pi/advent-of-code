use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use crate::{Pos, Scalar};
use crate::instructions::Instruction;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {
    pos: Pos,
    facing: Pos,
}
impl State {
    pub fn naive_advance(&self) -> Self {
        Self {
            pos: self.pos.add(self.facing),
            facing: self.facing,
        }
    }
    pub fn advance(&self, map: &Map) -> Self {
        let new = self.naive_advance();
        let new = map.redirects.get(&new).copied().unwrap_or(new);
        if map.rows[new.pos.y as usize][new.pos.x as usize] == Tile::Open {
            new
        } else {
            *self
        }
    }
    pub fn advance_n(&self, map: &Map, n: usize) -> Self {
        let mut curr = *self;
        for _ in 0..n {
            let new = curr.advance(map);
            if curr == new { break }
            curr = new;
        }
        curr
    }
    pub fn turn_l(&self) -> Self {
        State {
            pos: self.pos,
            facing: self.facing.turn_l(),
        }
    }
    pub fn turn_r(&self) -> Self {
        State {
            pos: self.pos,
            facing: self.facing.turn_r(),
        }
    }
    pub fn handle(&self, map: &Map, instruction: Instruction) -> Self {
        match instruction {
            Instruction::TurnL => self.turn_l(),
            Instruction::TurnR => self.turn_r(),
            Instruction::Advance(n) => self.advance_n(map, n)
        }
    }
    pub fn password(&self) -> Scalar {
        let facing =
            if self.facing == Pos::E { 0 }
            else if self.facing == Pos::S { 1 }
            else if self.facing == Pos::W { 2 }
            else { 3 };
        (self.pos.y + 1) * 1000 + (self.pos.x) * 4 + facing
    }
}
impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) facing ", self.pos.x, self.pos.y)?;
        if self.facing == Pos::N {
            write!(f, "N")
        } else if self.facing == Pos::S {
            write!(f, "S")
        } else if self.facing == Pos::E {
            write!(f, "E")
        } else {
            write!(f, "W")
        }
    }
}

#[derive(Clone)]
pub struct Map {
    rows: Vec<Vec<Tile>>,
    redirects: HashMap<State, State>,
}
impl Map {
    pub fn start(&self) -> State {
        let x = (0..).find(|i| self.rows[0][*i] != Tile::OutOfBounds).unwrap();
        State {
            pos: Pos { x: x as Scalar, y: 0 },
            facing: Pos::E,
        }
    }
    pub fn setup_part1_redirects(&mut self) {
        for y in 0..self.rows.len() {
            let mut first_not_oob = 0;
            while self.rows[y][first_not_oob as usize] == Tile::OutOfBounds {
                first_not_oob += 1;
            }
            let mut last_not_oob = self.rows[y].len() as Scalar - 1;
            while self.rows[y][last_not_oob as usize] == Tile::OutOfBounds {
                last_not_oob -= 1;
            }

            let y = y as Scalar;
            self.redirects.insert(
                State {
                    pos: Pos { x: first_not_oob - 1, y },
                    facing: Pos::W,
                },
                State {
                    pos: Pos { x: last_not_oob, y },
                    facing: Pos::W,
                }
            );
            self.redirects.insert(
                State {
                    pos: Pos { x: last_not_oob + 1, y },
                    facing: Pos::E,
                },
                State {
                    pos: Pos { x: first_not_oob, y },
                    facing: Pos::E,
                }
            );
        }
        for x in 0..self.rows[0].len() {
            let mut first_not_oob = 0;
            while self.rows[first_not_oob as usize][x] == Tile::OutOfBounds {
                first_not_oob += 1;
            }
            let mut last_not_oob = self.rows.len() as Scalar - 1;
            while self.rows[last_not_oob as usize][x] == Tile::OutOfBounds {
                last_not_oob -= 1;
            }

            let x = x as Scalar;
            self.redirects.insert(
                State {
                    pos: Pos { x, y: first_not_oob - 1 },
                    facing: Pos::N,
                },
                State {
                    pos: Pos { x, y: last_not_oob },
                    facing: Pos::N,
                }
            );
            self.redirects.insert(
                State {
                    pos: Pos { x, y: last_not_oob + 1 },
                    facing: Pos::S,
                },
                State {
                    pos: Pos { x, y: first_not_oob },
                    facing: Pos::S,
                }
            );
        }
    }
}
impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().map(|line| line.len()).max().unwrap();
        let rows = s.lines()
            .map(|line| (0..width).map(|i| if i < line.len() {
                &line[i..i+1]
            } else {
                " "
            }))
            .map(|tiles| tiles.map(|s| s.parse()).collect())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            rows,
            redirects: HashMap::new(),
        })
    }
}
impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.rows.len() {
            for x in 0..self.rows[0].len() {
                write!(f, "{:?}", self.rows[y][x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    OutOfBounds,
    Open,
    Wall,
}
impl FromStr for Tile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            " " => Ok(Tile::OutOfBounds),
            "." => Ok(Tile::Open),
            "#" => Ok(Tile::Wall),
            _ => Err(format!("Unknown tile: {}", s)),
        }
    }
}
impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBounds => write!(f, " "),
            Self::Open => write!(f, "."),
            Self::Wall => write!(f, "#"),
        }
    }
}
