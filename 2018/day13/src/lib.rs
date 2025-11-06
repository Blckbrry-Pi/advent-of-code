aoc_tools::aoc_sol!(day13 2018: part1, part2);
aoc_tools::map_struct!(Map of Cell, pos Scalar; +y => D);
type Scalar = i16;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Hori,
    Vert,
    Curve { n: bool, e: bool, s: bool, w: bool, },
    Inter,
}
impl Cell {
    pub fn update_cart(&self, cart: &mut Cart) {
        match *self {
            Self::Empty => unreachable!("Cart should not be able to drive onto an empty track"),
            | Self::Hori | Self::Vert => (),
            Self::Inter => {
                cart.vel = cart.state.apply(cart.vel);
                cart.state.advance();
            },
            Self::Curve { e, w, .. } if cart.vel == Pos::N || cart.vel == Pos::S => {
                if e {
                    cart.vel = Pos::E;
                } else if w {
                    cart.vel = Pos::W;
                } else {
                    unreachable!();
                }
            },
            Self::Curve { n, s, .. } if cart.vel == Pos::E || cart.vel == Pos::W => {
                if n {
                    cart.vel = Pos::N;
                } else if s {
                    cart.vel = Pos::S;
                } else {
                    unreachable!();
                }
            },
            Self::Curve { .. } => unreachable!("Invalid cart velocity"),
        }
    }
}
impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => ' ',
            Self::Hori => '─',
            Self::Vert => '│',
            Self::Curve { n: true, e: true, .. } => '╰',
            Self::Curve { e: true, s: true, .. } => '╭',
            Self::Curve { s: true, w: true, .. } => '╮',
            Self::Curve { w: true, n: true, .. } => '╯',
            Self::Curve { .. } => '╳',
            Self::Inter => '┼',
        };
        write!(f, "{c}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Cart {
    pos: Pos,
    vel: Pos,
    state: TurnState,
    crashed: bool,
}
impl Cart {
    pub fn step(&mut self, map: &Map) {
        if self.crashed { return }
        self.pos = self.pos.add(self.vel);
        let new_cell = map.get_raw(self.pos).unwrap();
        new_cell.update_cart(self);
    }
}
impl Debug for Cart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.crashed { return write!(f, "#"); }

        let c = if self.vel == Pos::N { '↑' }
        else if self.vel == Pos::E { '→' }
        else if self.vel == Pos::S { '↓' }
        else if self.vel == Pos::W { '←' }
        else { '*' };

        let color_code = match self.state {
            TurnState::L => 31,
            TurnState::F => 32,
            TurnState::R => 34,
        };

        write!(f, "\x1b[1;{color_code}m{c}\x1b[0m")
    }
}
impl Ord for Cart {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.crashed.cmp(&other.crashed)
            .then(self.pos.y.cmp(&other.pos.y))
            .then(self.pos.x.cmp(&other.pos.x))
    }
}
impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TurnState { L, F, R }
impl TurnState {
    pub fn apply(&self, dir: Pos) -> Pos {
        match self {
            Self::L => dir.turn_l(),
            Self::F => dir,
            Self::R => dir.turn_r(),
        }
    }
    pub fn advance(&mut self) {
        match self {
            Self::L => *self = Self::F,
            Self::F => *self = Self::R,
            Self::R => *self = Self::L,
        }
    }
}

#[allow(dead_code)]
fn draw_map(map: &Map, carts: &[Cart]) {
    for (y, row) in map.rows.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let pos = Pos { x: x as Scalar, y: y as Scalar };
            if let Some(cart) = carts.iter().find(|c| c.pos == pos) {
                print!("{cart:?}");
            } else {
                print!("{cell:?}");
            }
        }
        println!();
    }
}

fn check_crashes(carts: &mut [Cart], i: usize) -> Option<Pos> {
    let (before, carts) = carts.split_at_mut(i);
    let (curr, after) = carts.split_at_mut(1);
    let curr = &mut curr[0];
    for other in before.iter_mut().chain(after) {
        if curr.pos == other.pos && !other.crashed {
            curr.crashed = true;
            other.crashed = true;
            return Some(other.pos);
        }
    }
    None
}

pub fn part1(input: &str) -> String {
    let (map, mut carts) = parse_input(input);
    loop {
        carts.sort_unstable();
        for i in 0..carts.len() {
            if carts[i].crashed { continue }
            carts[i].step(&map);
            if let Some(pos) = check_crashes(&mut carts, i) {
                return format!("{},{}", pos.x, pos.y);
            }
        }
    }
}

pub fn part2(input: &str) -> String {
    let (map, mut carts) = parse_input(input);
    loop {
        carts.sort_unstable();

        // Carts that are not crashed are always less than carts that are
        // crashed, so all crashed carts will be at the end of the list
        if let Some((crashed_idx, _)) = carts.iter().enumerate().find(|(_, c)| c.crashed) {
            carts.drain(crashed_idx..);
        }
        if carts.len() == 1 { return format!("{},{}", carts[0].pos.x, carts[0].pos.y) }

        for i in 0..carts.len() {
            if carts[i].crashed { continue }
            carts[i].step(&map);
            check_crashes(&mut carts, i);
        }
    }
}

fn parse_input(input: &str) -> (Map, Vec<Cart>) {
    let mut carts = vec![];
    let mut map = Map { rows: vec![] };
    for (y, line) in input.lines().enumerate() {
        if line.trim().is_empty() { continue }
        map.rows.push(Vec::with_capacity(map.rows.last().map(|r| r.len()).unwrap_or(0)));
        for (x, c) in line.chars().enumerate() {
            match c {
                ' ' => map.rows.last_mut().unwrap().push(Cell::Empty),
                '^' | 'v' | '|' => map.rows.last_mut().unwrap().push(Cell::Vert),
                '>' | '<' | '-' => map.rows.last_mut().unwrap().push(Cell::Hori),
                '+' => map.rows.last_mut().unwrap().push(Cell::Inter),
                '/' | '\\' => {
                    let cell_to_left = *map.rows.last().unwrap().last().unwrap_or(&Cell::Empty);
                    let has_to_left = cell_to_left == Cell::Hori || cell_to_left == Cell::Inter || matches!(cell_to_left, Cell::Curve { e: true, .. });
                    let cell = match (c == '/', has_to_left) {
                        (false, false) => Cell::Curve { n:  true, e:  true, s: false, w: false },
                        ( true, false) => Cell::Curve { n: false, e:  true, s:  true, w: false },
                        (false,  true) => Cell::Curve { n: false, e: false, s:  true, w:  true },
                        ( true,  true) => Cell::Curve { n:  true, e: false, s: false, w:  true },
                    };
                    map.rows.last_mut().unwrap().push(cell);
                },
                _ => panic!("Invalid map character {c:?}"),
            }
            let pos = Pos { x: x as Scalar, y: y as Scalar };
            match c {
                '^' => carts.push(Cart { pos, vel: Pos::N, state: TurnState::L, crashed: false }),
                '>' => carts.push(Cart { pos, vel: Pos::E, state: TurnState::L, crashed: false }),
                'v' => carts.push(Cart { pos, vel: Pos::S, state: TurnState::L, crashed: false }),
                '<' => carts.push(Cart { pos, vel: Pos::W, state: TurnState::L, crashed: false }),
                _ => (),
            }
        }
    }
    (map, carts)
}
