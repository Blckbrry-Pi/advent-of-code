use intcode_2019::Machine;

type Scalar = i32;

aoc_tools::aoc_sol!(day11 2019: part1, part2);
aoc_tools::pos!(Scalar; +y => D);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color { B = 0, W = 1 }

struct PaintingRobot {
    data: Vec<isize>,
    machine: Machine,
    pos: Pos,
    dir: Pos,
}
impl PaintingRobot {
    pub fn new(data: Vec<isize>) -> Self {
        Self {
            data,
            machine: Machine::new(vec![]),
            pos: Pos::ZERO,
            dir: Pos::U,
        }
    }
    pub fn do_step(&mut self, map: &mut Map) -> bool {
        self.machine.input.0.push(map.get(self.pos) as isize);
        while self.machine.output.len() < 2 {
            if self.machine.step(&mut self.data).is_err() {
                return false;
            }
        }
        let paint = if self.machine.output[0] == 1 { Color::W } else { Color::B };
        map.set(self.pos, paint);

        self.dir = if self.machine.output[1] != 0 { self.dir.turn_r() } else { self.dir.turn_l() };
        self.machine.output.clear();

        self.pos = self.pos.add(self.dir);
        true
    }
}

#[derive(Clone)]
struct Map {
    tiles: HashMap<Pos, Color>,
}
impl Map {
    pub fn new(starting_panel: Color) -> Self {
        Self {
            tiles: [(Pos::ZERO, starting_panel)].into_iter().collect(),
        }
    }
    pub fn get(&self, pos: Pos) -> Color {
        self.tiles.get(&pos).copied().unwrap_or(Color::B)
    }
    pub fn set(&mut self, pos: Pos, color: Color) {
        self.tiles.insert(pos, color);
    }
}
impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut x = (Scalar::MAX, Scalar::MIN);
        let mut y = (Scalar::MAX, Scalar::MIN);
        for &pos in self.tiles.keys() {
            x.0 = x.0.min(pos.x);
            x.1 = x.1.max(pos.x);
            y.0 = y.0.min(pos.y);
            y.1 = y.1.max(pos.y);
        }
        for y in y.0..=y.1 {
            for x in x.0..=x.1 {
                let pos = Pos { x, y };
                if self.get(pos) == Color::W {
                // if self.tiles.get(&pos).is_some() {
                    write!(f, "█")
                } else {
                    write!(f, " ")
                }?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part1(input: &str) -> usize {
    let data = parse_input(input);
    let mut map = Map::new(Color::B);
    let mut robot = PaintingRobot::new(data);
    while robot.do_step(&mut map) {}
    map.tiles.len()
}

pub fn part2(input: &str) -> usize {
    let data = parse_input(input);
    let mut map = Map::new(Color::W);
    let mut robot = PaintingRobot::new(data.clone());
    while robot.do_step(&mut map) {}

    // TODO: Decode it into a `String`
    println!("{map:?}");
    0
}

fn parse_input(input: &str) -> Vec<isize> {
    intcode_2019::parse_program(input, 512)
}
