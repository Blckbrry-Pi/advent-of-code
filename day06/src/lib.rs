aoc_tools::aoc_sol!(day06: part1, part2);
aoc_tools::fast_hash!();

fn part1(input: &str) -> usize {
    let mut map = parse_input(input);

    while map.guard_in_bounds() {
        map.step();
        map.log_guard_state();
    }

    map.explored_in_bounds().len()
}

fn part2(input: &str) -> usize {
    let template_map = parse_input(input);

    let explored_in_bounds = {
        let mut map = template_map.clone().ensure_guard_states_capacity();

        while map.guard_in_bounds() {
            map.step();
            map.log_guard_state();
        }
        map.explored_in_bounds()
    };
    let explored_in_bounds = {
        let mut list: Vec<_> = explored_in_bounds.into_iter().collect();
        list.sort();
        list
    };

    let mut loops = vec![];
    let mut map = template_map.clone();
    for pos in explored_in_bounds.iter().copied() {
        if template_map.guard.loc == pos { continue }

        // let mut map = template_map.clone().ensure_guard_states_capacity();
        map.guard_states.clear();
        map.guard = template_map.guard;
        map.cells[pos.y as usize][pos.x as usize] = Cell::NewObstructed;

        while map.guard_in_bounds() {
            if map.step() {
                if map.guard_state_seen_before() > 0 {
                    loops.push(pos);
                    break
                } else {
                    map.log_guard_state();
                }
            }
        }
        map.cells[pos.y as usize][pos.x as usize] = Cell::Clear;
    }

    loops.len()
}

fn parse_input(input: &str) -> Map {
    let mut guard = Guard {
        loc: Pos { x: -1, y: -1 },
        dir: Direction::U,
    };
    let mut cells = vec![];

    for (y, line) in input.lines().enumerate() {
        cells.push(vec![]);
        for (x, ch) in line.chars().enumerate() {
            let cell_type = match ch {
                '#' => Cell::Obstructed,
                '.' | '^' => Cell::Clear,
                _ => unimplemented!("Unknown cell type: {ch}"),
            };
            cells[y].push(cell_type);
            if ch == '^' {
                guard.loc = Pos { x: x as isize, y: y as isize };
            }
        }
    }

    Map {
        cells,
        guard,
        guard_states: [(guard, 1)].into_iter().collect(),
    }.ensure_guard_states_capacity()
}

#[derive(Clone, PartialEq, Eq)]
pub struct Map {
    cells: Vec<Vec<Cell>>,
    guard: Guard,
    guard_states: FastMap<Guard, usize>,
}
impl Map {
    pub fn ensure_guard_states_capacity(mut self) -> Self {
        let assumed_max = self.cols() * self.rows() / 128;
        self.guard_states.reserve(assumed_max.saturating_sub(self.guard_states.len()));
        self
    }
    pub fn rows(&self) -> usize {
        self.cells.len()
    }
    pub fn cols(&self) -> usize {
        self.cells[0].len()
    }

    pub fn get(&self, Pos { x, y }: Pos) -> Cell {
        let (x, y) = (x as usize, y as usize);
        if !(0..self.rows()).contains(&y) || !(0..self.cols()).contains(&x) {
            return Cell::Clear;
        }
        self.cells[y][x]
    }

    pub fn step(&mut self) -> bool {
        if self.get(self.guard.forward().loc) != Cell::Clear {
            self.guard = self.guard.turn();
            true
        } else {
            self.guard = self.guard.forward();
            false
        }
    }

    pub fn guard_state_seen_before(&self) -> usize {
        self.guard_states.get(&self.guard).copied().unwrap_or(0)
    }
    pub fn log_guard_state(&mut self) {
        *self.guard_states.entry(self.guard)
            .or_insert(0) += 1;
    }

    pub fn guard_in_bounds(&self) -> bool {
        let in_bounds_y = 0 <= self.guard.loc.y && self.guard.loc.y < self.rows() as isize;
        let in_bounds_x = 0 <= self.guard.loc.x && self.guard.loc.x < self.cols() as isize;

        in_bounds_y && in_bounds_x
    }

    pub fn explored_in_bounds(&self) -> HashSet<Pos> {
        self.guard_states.keys()
            .copied()
            .map(|Guard { loc, .. }| loc)
            .filter(|k| 0 <= k.y && k.y < self.rows() as isize)
            .filter(|k| 0 <= k.x && k.x < self.cols() as isize)
            .collect()
    }
}
impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.rows() as isize {
            for x in 0..self.cols() as isize {
                let pos = Pos { x, y };
                let guard_states: Vec<_> = [
                    Guard { loc: pos, dir: Direction::U },
                    Guard { loc: pos, dir: Direction::D },
                    Guard { loc: pos, dir: Direction::L },
                    Guard { loc: pos, dir: Direction::R },
                ].iter().filter_map(|g| self.guard_states.contains_key(g).then_some(g.dir)).collect();
                if self.guard.loc == pos {
                    write!(f, "\x1b[7m{:#?}\x1b[0m", self.guard)
                } else if !guard_states.is_empty() {
                    if guard_states.iter().all(|d| matches!(d, Direction::U | Direction::D)) {
                        write!(f, "|")
                    } else if guard_states.iter().all(|d| matches!(d, Direction::L | Direction::R)) {
                        write!(f, "—")
                    } else {
                        write!(f, "+")
                    }
                } else {
                    write!(f, "{:?}", self.get(pos))
                }?
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Clear,
    Obstructed,
    NewObstructed,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clear => write!(f, "."),
            Self::Obstructed => write!(f, "#"),
            Self::NewObstructed => write!(f, "O"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos { x: isize, y: isize }
impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Guard {
    loc: Pos,
    dir: Direction,
}
impl Guard {
    pub fn forward(&self) -> Self {
        Self { loc: self.dir.step(self.loc), ..*self }
    }
    pub fn turn(&self) -> Self {
        Self { dir: self.dir.turn_r(), ..*self }
    }
}
impl Debug for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.dir)?;
        if !f.alternate() {
            write!(f, "{:?}", self.loc)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction { U, D, L, R }
impl Direction {
    pub fn turn_r(&self) -> Self {
        match self {
            Self::U => Self::R,
            Self::R => Self::D,
            Self::D => Self::L,
            Self::L => Self::U,
        }
    }

    pub fn about(&self) -> Self { self.turn_r().turn_r() }
    pub fn turn_l(&self) -> Self { self.turn_r().turn_r().turn_r() }

    pub fn dx(&self) -> isize {
        match self {
            Self::L => -1,
            Self::U | Self::D => 0,
            Self::R => 1,
        }
    }
    pub fn dy(&self) -> isize {
        match self {
            Self::U => -1,
            Self::L | Self::R => 0,
            Self::D => 1,
        }
    }
    pub fn step(&self, pos: Pos) -> Pos {
        Pos {
            x: pos.x + self.dx(),
            y: pos.y + self.dy(),
        }
    }
}
impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U => write!(f, "^"),
            Self::D => write!(f, "v"),
            Self::L => write!(f, "<"),
            Self::R => write!(f, ">"),
        }
    }
}
