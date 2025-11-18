use std::collections::BinaryHeap;


const EROSION_LEVEL_MOD: Num = 20183;
const X_COEFF: Num = 16807;
const Y_COEFF: Num = 48271;

aoc_tools::aoc_sol!(day22 2018: part1, part2);
aoc_tools::map_struct!(Map of Cell { depth: Num, target: Pos }, pos Num; +y => D);
type Num = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ToolState {
    Torch,
    Neither,
    Climbing,
}
impl ToolState {
    pub fn is_compatible_with(&self, terrain: Terrain) -> bool {
        match self {
            Self::Torch    => terrain != Terrain::Wet,
            Self::Neither  => terrain != Terrain::Rocky,
            Self::Climbing => terrain != Terrain::Narrow,
        }
    }
    pub fn all_compatible_with(terrain: Terrain) -> [Self; 2] {
        match terrain {
            Terrain::Rocky  => [Self::Torch, Self::Climbing],
            Terrain::Wet    => [Self::Neither, Self::Climbing],
            Terrain::Narrow => [Self::Torch, Self::Neither]
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Terrain {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}
impl Terrain {
    pub fn from_erosion_level(level: u32) -> Self {
        match level % 3 {
            0 => Self::Rocky,
            1 => Self::Wet,
            2 => Self::Narrow,
            _ => unreachable!(),
        }
    }
}
impl Debug for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rocky  => write!(f, "."),
            Self::Wet    => write!(f, "="),
            Self::Narrow => write!(f, "|"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PathingState(Num, ToolState, Pos);
impl PartialOrd for PathingState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PathingState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
            .then_with(|| self.2.y.cmp(&other.2.y))
            .then_with(|| self.2.x.cmp(&other.2.x))
            .then_with(|| self.1.cmp(&other.1).reverse())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Cell(Num);
impl Cell {
    pub fn new(n: Num) -> Self {
        Self(n % EROSION_LEVEL_MOD)
    }
    pub fn terrain(&self) -> Terrain {
        Terrain::from_erosion_level(self.0)
    }
}
impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "|{:5} {:?}|", self.0, self.terrain())
        } else {
            write!(f, "{:?}", self.terrain())
        }
    }
}

impl Map {
    pub fn add_row(&mut self, len: Num) {
        let mut new_row = Vec::with_capacity(len as usize);
        let y = self.rows.len() as Num;
        for x in 0..len {
            if x == 0 {
                new_row.push(Cell::new(y * Y_COEFF + self.depth));
                continue
            } else if y == 0 {
                new_row.push(Cell::new(x * X_COEFF + self.depth));
                continue;
            } else if (Pos { x, y }) == self.target {
                new_row.push(Cell::new(self.depth));
                continue;
            }
            let x_min_1 = *new_row.last().unwrap();
            let y_min_1 = self.rows.last().unwrap()[x as usize];
            new_row.push(Cell::new(x_min_1.0 * y_min_1.0 + self.depth));
        }
        self.rows.push(new_row);
    }
    #[allow(unused)]
    pub fn add_col(&mut self) {
        let mut new_col = Vec::with_capacity(self.height());
        let x = self.rows[0].len() as Num;
        for y in 0..self.rows.len() as Num {
            if x == 0 {
                new_col.push(Cell::new(y * Y_COEFF + self.depth));
                continue
            } else if y == 0 {
                new_col.push(Cell::new(x * X_COEFF + self.depth));
                continue;
            } else if (Pos { x, y }) == self.target {
                new_col.push(Cell::new(self.depth));
                continue;
            }
            let y_min_1 = *new_col.last().unwrap();
            let x_min_1 = *self.rows[y as usize].last().unwrap();
            new_col.push(Cell::new(x_min_1.0 * y_min_1.0 + self.depth));
        }
        for (y, new_val) in new_col.into_iter().enumerate() {
            self.rows[y].push(new_val);
        }
    }
    pub fn risk_total(&self, x: impl IntoIterator<Item = Num> + Clone, y: impl IntoIterator<Item = Num>) -> Num {
        let mut total = 0;
        for y in y {
            for x in x.clone() {
                let pos = Pos { x, y };
                total += self.get_raw(pos).unwrap().terrain() as Num;
            }
        }
        total
    }

    pub fn shortest_path(&mut self) -> Num {
        let mut visited = HashSet::<(ToolState, Pos)>::new();
        let mut queue = BinaryHeap::<PathingState>::new();
        queue.push(PathingState(0, ToolState::Torch, Pos { x: 0, y: 0 }));
        while let Some(PathingState(dist, tool, pos)) = queue.pop() {
            if pos == self.target && tool == ToolState::Torch { return dist; }
            if visited.contains(&(tool, pos)) { continue; }
            for offset in [Pos::S, Pos::E, Pos::N, Pos::W] {
                let neighbor = pos.add(offset);

                // if neighbor.y != 0 && neighbor.y as usize >= self.height() {
                //     self.add_row(self.width() as Num);
                // }
                // if neighbor.x != 0 && neighbor.x as usize >= self.height() {
                //     self.add_col();
                // }

                let Some(cell) = self.get_raw(neighbor) else { continue; };
                if tool.is_compatible_with(cell.terrain()) {
                    let new_dist = dist + 1;
                    queue.push(PathingState(new_dist, tool, neighbor));
                } else {
                    for new_tool in ToolState::all_compatible_with(cell.terrain()) {
                        if new_tool.is_compatible_with(self.get_raw(pos).unwrap().terrain()) {
                            queue.push(PathingState(dist + 8, new_tool, neighbor));
                        }
                    }
                }
            }
            visited.insert((tool, pos));
        }
        panic!("No path found to target");
    }
}

pub fn part1(input: &str) -> Num {
    let (depth, target) = parse_input(input);
    let mut map = Map { rows: vec![], depth, target };
    for _ in 0..=target.y {
        map.add_row(target.x + 1);
    }
    map.risk_total(0..=target.x, 0..=target.y)
}

pub fn part2(input: &str) -> u32 {
    let (depth, target) = parse_input(input);
    let mut map = Map { rows: vec![], depth, target };
    for _ in 0..=target.y + 20 {
        map.add_row(target.x + 20);
    }
    map.shortest_path()
}

fn parse_input(input: &str) -> (Num, Pos) {
    let (depth, target) = input.trim().split_once('\n').unwrap();
    let depth = depth.strip_prefix("depth: ").unwrap();
    let depth = depth.parse::<Num>().unwrap();
    
    let target = target.strip_prefix("target: ").unwrap();
    let (x, y) = target.split_once(',').unwrap();
    let x = x.parse::<Num>().unwrap();
    let y = y.parse::<Num>().unwrap();

    (depth, Pos { x, y })
}
