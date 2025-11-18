use std::str::FromStr;

aoc_tools::aoc_sol!(day18 2018: part1, part2);
aoc_tools::map_struct!(Map of Cell { trees: Vec<Vec<u8>>, yards: Vec<Vec<u8>> }, pos Scalar; +y => D);
type Scalar = u8;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Trees,
    Lumberyard,
}
impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, " "),
            Self::Trees => write!(f, "|"),
            Self::Lumberyard => write!(f, "#"),
        }
    }
}

impl Map {
    pub fn update_counts(&mut self) {
        // Clear existing
        for y in 0..self.trees.len() {
            for x in 0..self.trees[y].len() {
                self.trees[y][x] = 0;
                self.yards[y][x] = 0;
            }
        }

        for y in 0..self.rows.len() {
            for dy in -1..=1 {
                if y == 0 && dy < 0 { continue }
                let affected_y = (y as isize + dy) as usize;

                let Some(trees_row) = self.trees.get_mut(affected_y) else { continue };
                let Some(yards_row) = self.yards.get_mut(affected_y) else { continue };

                for x in 0..self.rows[y].len() {
                    let trees_inc = if matches!(self.rows[y][x], Cell::Trees) { 1 } else { 0 };
                    let yards_inc = if matches!(self.rows[y][x], Cell::Lumberyard) { 1 } else { 0 };
                    for dx in -1..=1 {
                        if x == 0 && dx < 0 { continue }
                        if dy == 0 && dx == 0 { continue }

                        let affected_x = (x as isize + dx) as usize;
                        if affected_x >= trees_row.len() || affected_x >= yards_row.len() { continue }

                        trees_row[affected_x] += trees_inc;
                        yards_row[affected_x] += yards_inc;
                    }
                }
            }
        }
    }

    pub fn do_step(&mut self) {
        for y in 0..self.rows.len() {
            for x in 0..self.rows[y].len() {
                match self.rows[y][x] {
                    Cell::Empty => if self.trees[y][x] >= 3 {
                        self.rows[y][x] = Cell::Trees;
                    },
                    Cell::Trees => if self.yards[y][x] >= 3 {
                        self.rows[y][x] = Cell::Lumberyard;
                    },
                    Cell::Lumberyard => if self.trees[y][x] == 0 || self.yards[y][x] == 0 {
                        self.rows[y][x] = Cell::Empty;
                    }
                }
            }
        }
    }

    pub fn resource_value(&self) -> u64 {
        let mut trees = 0;
        let mut yards = 0;
        for row in &self.rows {
            for cell in row {
                match cell {
                    Cell::Empty => (),
                    Cell::Trees => trees += 1,
                    Cell::Lumberyard => yards += 1,
                }
            }
        }
        trees * yards
    }
}
impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Self { rows: vec![], trees: vec![], yards: vec![] };
        for line in s.lines() {
            if line.trim().is_empty() { continue }

            let reserve = map.rows.last().map(|v| v.len()).unwrap_or_default();
            map.rows.push(Vec::with_capacity(reserve));

            for c in line.chars() {
                let cell = match c {
                    '.' => Cell::Empty,
                    '|' => Cell::Trees,
                    '#' => Cell::Lumberyard,
                    _ => return Err(format!("Invalid cell {c:?}")),
                };
                map.rows.last_mut().unwrap().push(cell);
            }

            if map.rows.last().unwrap().len() != map.rows.first().unwrap().len() {
                return Err("Jagged map found!".to_string());
            }
        }
        map.trees = (0..map.rows.len()).map(|_| (0..map.rows[0].len()).map(|_| 0).collect()).collect();
        map.yards = (0..map.rows.len()).map(|_| (0..map.rows[0].len()).map(|_| 0).collect()).collect();
        Ok(map)
    }
}

pub fn part1(input: &str) -> u64 {
    const STEPS: usize = 10;

    let mut map = parse_input(input);
    for _ in 0..STEPS {
        map.update_counts();
        map.do_step();
    }
    map.resource_value()
}

pub fn part2(input: &str) -> u64 {
    const STEPS: usize = 1_000_000_000;

    let mut seen = HashMap::new();
    let mut map = parse_input(input);
    let mut remaining = 0;
    for i in 0..STEPS {
        map.update_counts();
        map.do_step();
        if let Some(prev) = seen.insert(map.rows.clone(), i) {
            remaining = (STEPS - i - 1) % (i - prev);
            break;
        }
    }
    for _ in 0..remaining {
        map.update_counts();
        map.do_step();
    }
    map.resource_value()
}

fn parse_input(input: &str) -> Map {
    input.parse().unwrap()
}
