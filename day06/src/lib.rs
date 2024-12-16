#![feature(map_try_insert)]

aoc_tools::aoc_sol!(day06: part1, part2);
aoc_tools::pos!(Scalar);

type Scalar = isize;

pub fn part1(input: &str) -> usize {
    let mut map = parse_input(input);

    while map.guard_in_bounds() {
        map.step(false);
    }

    map.explored_in_bounds().len()
}

pub fn part2(input: &str) -> usize {
    // let start = std::time::Instant::now();
    let template_map = parse_input(input);
    // let post_parse = std::time::Instant::now();
    
    let pos_to_prev_guard = {
        let mut map = template_map.clone();

        while map.guard_in_bounds() {
            map.step(true);
        }
        map.prev_state
    };
    // let post_initial_traverse = std::time::Instant::now();

    let mut loops = Vec::with_capacity(pos_to_prev_guard.len());
    let mut map = template_map.clone().ensure_guard_states_capacity();
    for (pos, guard) in pos_to_prev_guard.into_iter() {
        if template_map.guard.loc == pos { continue }
        if pos.y < 0 || pos.y >= map.rows() as isize { continue }
        if pos.x < 0 || pos.x >= map.cols() as isize { continue }

        // let mut map = template_map.clone().ensure_guard_states_capacity();
        map.guard_states.clear();
        map.guard = guard;

        map.cells[pos.y as usize][pos.x as usize] = Cell::NewObstructed;

        while map.guard_in_bounds() {
            if map.step(false) {
                if map.guard_state_seen_before() {
                    loops.push(pos);
                    break
                } else {
                    map.log_guard_state();
                }
            }
        }
        map.cells[pos.y as usize][pos.x as usize] = Cell::Clear;
    }

    // println!(
    //     "parse: {:?}, initial traverse: {:?}, counting: {:?}",
    //     post_parse - start,
    //     post_initial_traverse - post_parse,
    //     post_initial_traverse.elapsed(),
    // );

    loops.len()
}

// #[inline(never)]
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
                guard.loc = Pos { x: x as Scalar, y: y as Scalar };
            }
        }
    }

    Map {
        prev_state: HashMap::with_capacity(cells.len() * cells[0].len() / 128),
        cells,
        guard,
        guard_states: [guard].into_iter().collect(),
    }.ensure_guard_states_capacity()
}

#[derive(Clone, PartialEq, Eq)]
struct Map {
    cells: Vec<Vec<Cell>>,
    guard: Guard,
    guard_states: HashSet<Guard>,
    prev_state: HashMap<Pos, Guard>,
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

    pub fn step(&mut self, track_prev_state: bool) -> bool {
        if self.get(self.guard.forward().loc) != Cell::Clear {
            self.guard = self.guard.turn();
            true
        } else {
            if track_prev_state {
                let new_guard = self.guard.forward();
                let _ = self.prev_state.try_insert(new_guard.loc, self.guard);
                self.guard = new_guard;
            } else {
                while self.guard_in_bounds() {
                    let new_guard = self.guard.forward();
                    if self.get(new_guard.loc) != Cell::Clear { break }
                    self.guard = new_guard;
                }
                // self.guard = self.guard.forward();
            }
            false
        }
    }

    pub fn guard_state_seen_before(&self) -> bool {
        self.guard_states.contains(&self.guard)
    }
    pub fn log_guard_state(&mut self) {
        self.guard_states.insert(self.guard);
    }

    pub fn guard_in_bounds(&self) -> bool {
        let in_bounds_y = 0 <= self.guard.loc.y && self.guard.loc.y < self.rows() as Scalar;
        let in_bounds_x = 0 <= self.guard.loc.x && self.guard.loc.x < self.cols() as Scalar;

        in_bounds_y && in_bounds_x
    }

    pub fn explored_in_bounds(&self) -> HashSet<Pos> {
        self.guard_states.iter()
            .copied()
            .map(|Guard { loc, .. }| loc)
            .filter(|k| 0 <= k.y && k.y < self.rows() as Scalar)
            .filter(|k| 0 <= k.x && k.x < self.cols() as Scalar)
            .collect()
    }
}
impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.rows() as Scalar {
            for x in 0..self.cols() as Scalar {
                let pos = Pos { x, y };
                let guard_states: Vec<_> = [
                    Guard { loc: pos, dir: Direction::U },
                    Guard { loc: pos, dir: Direction::D },
                    Guard { loc: pos, dir: Direction::L },
                    Guard { loc: pos, dir: Direction::R },
                ].iter().filter_map(|g| self.guard_states.contains(g).then_some(g.dir)).collect();
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
enum Direction { U, D, L, R }
impl Direction {
    pub fn turn_r(&self) -> Self {
        match self {
            Self::U => Self::R,
            Self::R => Self::D,
            Self::D => Self::L,
            Self::L => Self::U,
        }
    }

    pub fn dx(&self) -> Scalar {
        match self {
            Self::L => -1,
            Self::U | Self::D => 0,
            Self::R => 1,
        }
    }
    pub fn dy(&self) -> Scalar {
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

// impl Pos {
//     fn into_u16(&self) -> u16 {
//         self.x as u8 * 150 + self.y
//         u16::from_be_bytes([self.x as u8, self.y as u8])
//     }
// }
