use std::collections::HashSet;
use std::fmt::Debug;
use std::ops::{Add, Sub};
use std::str::FromStr;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position { x: isize, y: isize }

impl Position {
    const UP: Position = Position { x: 0, y: -1 };
    const DOWN: Position = Position { x: 0, y: 1 };
    const LEFT: Position = Position { x: -1, y: 0 };
    const RIGHT: Position = Position { x: 1, y: 0 };

    pub fn x(&self) -> isize { self.x }
    pub fn y(&self) -> isize { self.y }

    pub fn manhattan_distance(&self, other: Position) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Possibilities {
    possibilities: HashSet<Position>,
}

impl Possibilities {
    pub fn new(position: Position) -> Self {
        let mut possibilities = HashSet::new();
        possibilities.insert(position);
        Self { possibilities }
    }
    pub fn next(&self, plot: &Plot) -> Self {
        let Possibilities { possibilities } = self;

        let mut next_possibilities = HashSet::new();

        for position in possibilities {
            for direction in &[Position::UP, Position::DOWN, Position::LEFT, Position::RIGHT] {
                let next_position = *position + *direction;
                if matches!(plot.get(next_position), Some(Cell::Garden | Cell::Start)) {
                    next_possibilities.insert(next_position);
                }
            }
        }

        Self { possibilities: next_possibilities }
    }

    pub fn size(&self) -> usize {
        self.possibilities.len()
    }
}

#[derive(Clone)]
pub struct Plot {
    rows: Vec<Vec<Cell>>,
}
impl Plot {
    pub fn is_valid_position(&self, position: Position) -> bool {
        if position.y < 0 || position.y >= self.rows.len() as isize {
            return false;
        }
        if position.x < 0 || position.x >= self.rows[0].len() as isize {
            return false;
        }
        true
    }
    pub fn get(&self, position: Position) -> Option<Cell> {
        if !self.is_valid_position(position) {
            return None;
        }
        Some(self.rows[position.y as usize][position.x as usize])
    }
    pub fn start(&self) -> Position {
        for i in 0..self.rows.len() {
            for j in 0..self.rows[i].len() {
                if self.rows[i][j] == Cell::Start {
                    return Position { x: j as isize, y: i as isize };
                }
            }
        }
        unreachable!("No start position found");
    }

    pub fn calc_history(&self, start: Position) -> PlotHistory {
        PlotHistory::new(self, start)
    }

    pub fn width(&self) -> usize {
        self.rows[0].len()
    }
    pub fn height(&self) -> usize {
        self.rows.len()
    }
}
impl Debug for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for cell in row {
                write!(f, "{:?}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl FromStr for Plot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.split('\n')
            .map(|line| line.chars().map(|c| c.to_string().parse().unwrap()).collect())
            .collect();
        Ok(Self { rows })
    }
}

pub struct PlotHistory {
    history: Vec<Possibilities>,
}
impl PlotHistory {
    pub fn new(plot: &Plot, start: Position) -> Self {
        let mut history = vec![Possibilities::new(start)];
        let mut current = history.last().unwrap().next(&plot);
        while history.len() < 2 || &current != &history[history.len() - 2] {
            let next = current.next(&plot);
            history.push(current);
            current = next;
        }
        Self { history }
    }

    pub fn get(&self, idx: usize) -> &Possibilities {
        if idx >= self.history.len() {
            let diff = idx - self.history.len() + 1;
            let parity = diff % 2;
            return &self.history[self.history.len() - parity - 1];
        } else {
            &self.history[idx]
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    Start,
    Garden,
    Rock,
}
impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Start => write!(f, "S"),
            Cell::Garden => write!(f, "."),
            Cell::Rock => write!(f, "#"),
        }
    }
}
impl FromStr for Cell {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Cell::Rock),
            "." => Ok(Cell::Garden),
            "S" => Ok(Cell::Start),
            _ => Err(()),
        }
    }
}
