#![feature(vec_push_within_capacity)]

aoc_tools::aoc_sol!(day15 2024: part1, part2);
type Scalar = i16;
aoc_tools::pos!(Scalar; +y => D);


pub fn part1(input: &str) -> i32 {
    let (mut warehouse, moves) = parse_input(input, false);

    for push in moves {
        warehouse.push(push);
    }

    warehouse.calc_gps()
}

pub fn part2(input: &str) -> i32 {
    let (mut warehouse, moves) = parse_input(input, true);

    for push in moves {
        warehouse.push_p2(push);
    }

    warehouse.calc_gps()
}

#[inline(never)]
fn parse_input(input: &str, expand: bool) -> (Warehouse, Vec<Pos>) {
    let (warehouse, moves_str) = input.split_once("\n\n").unwrap();

    let mut moves = Vec::with_capacity(moves_str.len());
    moves_str.trim().chars().filter_map(|c| match c {
        '^' => Some(Pos::U),
        'v' => Some(Pos::D),
        '<' => Some(Pos::L),
        '>' => Some(Pos::R),
        _ => None,
    }).for_each(|m| moves.push(m));
    

    let mut map = FastSet::new();
    let mut robot = Pos { x: 0, y: 0 };


    let mut x = 0;
    let mut y = 0;
    let mut i = 0;
    while i < warehouse.as_bytes().len() {
        let byte = *unsafe { warehouse.as_bytes().get_unchecked(i) };
        if byte == b'\n' {
            x = 0;
            y += 1;
            i += 1;
            continue;
        }

        let pos = Pos { x, y };
        let pos_exp = Pos { x: x * 2, y };

        let pos = if expand { pos_exp } else { pos };

        match byte {
            b'#' => {
                map.set(pos, Cell::Wall);
                if expand {
                    map.set(pos.add(Pos::R), Cell::Wall);
                }
            },
            b'O' => { map.set(pos, Cell::Box); },
            b'@' => robot = pos,
            _ => (),
        };

        x += 1;
        i += 1;
    }
    let width = x * if expand { 2 } else { 1 };
    let height = y + 1;

    (
        Warehouse { map, robot, width, height },
        moves
    )
}

#[derive(Clone)]
struct Warehouse {
    map: FastSet<Pos>,
    robot: Pos,
    width: Scalar,
    height: Scalar,
}

impl Warehouse {
    pub fn push(&mut self, direction: Pos) -> bool {
        if self.resolve_space(self.robot.add(direction), direction) {
            self.robot = self.robot.add(direction);
            true
        } else {
            false
        }
    }

    fn resolve_space(&mut self, pos: Pos, push: Pos) -> bool {
        match self.map.get(&pos) {
            Cell::Box => {
                if self.resolve_space(pos.add(push), push) {
                    self.map.set(pos, Cell::Clear);
                    self.map.set(pos.add(push), Cell::Box);
                    true
                } else {
                    false
                }
            },
            Cell::Wall => false,
            Cell::Clear => true,
        }
    }

    pub fn walls_in<const T: usize>(&self, pos: Pos, offsets: [Pos; T]) -> bool {
        for offset in offsets {
            if self.map.wall_at(&pos.add(offset)) {
                return true
            }
        }
        false
    }

    pub fn push_p2(&mut self, push: Pos) -> bool {
        if self.map.wall_at(&self.robot.add(push)) { return false }

        let hori_box_offs = if push.x < 0 { [push.mul(2)] } else { [push] };
        let vert_box_offs = [push, push.sub(push.swap().abs())];
        let box_offsets = if push.y == 0 { hori_box_offs.as_slice() } else { vert_box_offs.as_slice() };

        let can_push = box_offsets.iter().all(|&offset| self.resolve_space_p2(self.robot.add(offset), push));
        if can_push {
            box_offsets.iter().for_each(|&offset| self.move_boxes_p2(self.robot.add(offset), push));
            self.robot = self.robot.add(push);
            true
        } else {
            false
        }
    }

    fn resolve_space_p2(&self, pos: Pos, push: Pos) -> bool {
        if self.map.box_at(&pos) {
            let would_encounter_wall = if push.y == 0 {
                let offset = if push.x > 0 {
                    push.mul(2)
                } else {
                    push
                };
                self.walls_in(pos, [offset])
            } else {
                self.walls_in(pos, [push, push.add(push.swap().abs())])
            };
            if would_encounter_wall { return false }
            
            let hori_box_offs = [push.mul(2)];
            let vert_box_offs = [push.add(push.swap()), push, push.sub(push.swap())];
            let box_offsets = if push.y == 0 { hori_box_offs.as_slice() } else { vert_box_offs.as_slice() };

            box_offsets.iter().all(|&offset| self.resolve_space_p2(pos.add(offset), push))
        } else {
            true
        }
    }

    fn move_boxes_p2(&mut self, pos: Pos, push: Pos) {
        if !self.map.box_at(&pos) { return }
        let hori_box_offs = [push.mul(2)];
        let vert_box_offs = [push.add(push.swap()), push, push.sub(push.swap())];
        let box_offsets = if push.y == 0 { hori_box_offs.as_slice() } else { vert_box_offs.as_slice() };
        box_offsets.iter().for_each(|&offset| self.move_boxes_p2(pos.add(offset), push));

        self.map.set(pos, Cell::Clear);
        self.map.set(pos.add(push), Cell::Box);
    }

    #[inline(never)]
    fn calc_gps(&self) -> i32 {
        self.map.iter_boxes().map(Pos::gps).sum()
    }
}

impl Debug for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos { x, y };
                let ch = match (self.map.get(&pos), self.map.get(&pos.add(Pos::L)), f.alternate()) {
                    (Cell::Box, _, true) => '[',
                    (_, Cell::Box, true) => ']',
                    (Cell::Box, _, false) => 'O',
                    (Cell::Wall, _, _) => '#',
                    (Cell::Clear, _, _) => '.',
                };
                write!(f, "{ch}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Pos {
    pub fn gps(self) -> i32 {
        self.x as i32 + self.y as i32 * 100
    }
}

#[derive(Clone)]
struct FastSet<T>(Vec<Cell>, std::marker::PhantomData<T>);

impl FastSet<Pos> {
    fn new() -> Self {
        Self(vec![Cell::Clear; 8192], std::marker::PhantomData)
    }

    fn idx(pos: &Pos) -> u16 {
        let x = pos.x as u16 & 0x7F;
        let y = pos.y as u16 & 0x3F;
        (x << 6) | y
    }

    fn pos(idx: u16) -> Pos {
        let x = (idx >> 6) & 0x7F;
        let y = idx & 0x3F;
        Pos { x: x as i16, y: y as i16 }
    }

    fn set(&mut self, pos: Pos, cell: Cell) -> Cell {
        let idx = Self::idx(&pos);
        let prev = unsafe { *self.0.get_unchecked(idx as usize) };
        unsafe { *self.0.get_unchecked_mut(idx as usize) = cell; }
        prev
    }

    fn get(&self, pos: &Pos) -> Cell {
        let idx = Self::idx(pos);
        unsafe { *self.0.get_unchecked(idx as usize) }
    }
    fn box_at(&self, pos: &Pos) -> bool {
        self.get(pos) == Cell::Box
    }
    fn wall_at(&self, pos: &Pos) -> bool {
        self.get(pos) == Cell::Wall
    }

    fn iter_boxes(&self) -> impl Iterator<Item = Pos> + '_ {
        self.0.iter()
            .enumerate()
            .filter_map(|(i, &b)| (b == Cell::Box).then_some(Self::pos(i as u16)))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Clear,
    Box,
    Wall,
}

