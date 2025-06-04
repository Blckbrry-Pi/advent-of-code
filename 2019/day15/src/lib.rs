use intcode_2019::Machine;

type Scalar = i16;

aoc_tools::aoc_sol!(day15 2019: part1, part2);
aoc_tools::pos!(Scalar; +y=>D);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile { Free, Wall, Goal }

#[derive(Clone)]
struct Map {
    known: HashMap<Pos, Tile>,
}
impl Map {
    pub fn dist_from(&self, a: Pos, b: Pos) -> Result<i16, i16> {
        let mut dist = 0;
        let mut seen = HashSet::new();
        let mut to_explore: HashSet<_> = [a].into_iter().collect();
        loop {
            let mut new_to_explore = HashSet::new();
            for pos in to_explore {
                if !seen.insert(pos) { continue }
                if pos == b { return Ok(dist); }
                for offset in [Pos::N, Pos::S, Pos::E, Pos::W] {
                    let new_pos = pos.add(offset);
                    if self.known.get(&new_pos) == Some(&Tile::Wall) { continue }
                    if !seen.contains(&new_pos) {
                        new_to_explore.insert(new_pos);
                    }
                }
            }
            to_explore = new_to_explore;
            if to_explore.len() == 0 { return Err(dist); }
            dist += 1;
        }
    }
    pub fn target(&self) -> Pos {
        for (&pos, &tile) in &self.known {
            if tile == Tile::Goal { return pos }
        }
        panic!("No target found")
    }
}
impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut x_range = (0, 0);
        let mut y_range = (0, 0);
        for pos in self.known.keys() {
            x_range.0 = x_range.0.min(pos.x);
            x_range.1 = x_range.1.max(pos.x);
            y_range.0 = y_range.0.min(pos.y);
            y_range.1 = y_range.1.max(pos.y);
        }

        for y in y_range.0..=y_range.1 {
            for x in x_range.0..=x_range.1 {
                let pos = Pos { x, y };
                match self.known.get(&pos) {
                    Some(Tile::Free) if pos == Pos::ZERO => write!(f, "R"),
                    Some(Tile::Free) => write!(f, "."),
                    Some(Tile::Wall) => write!(f, "#"),
                    Some(Tile::Goal) => write!(f, "O"),
                    None => write!(f, " "),
                }?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Robot {
    machine: Machine,
    data: Vec<isize>,
    pos: Pos
}
impl Robot {
    pub fn new(data: Vec<isize>) -> Self {
        Self {
            machine: Machine::new(vec![]),
            data,
            pos: Pos::ZERO,
        }
    }
    pub fn step(&mut self, direction: Pos) -> Result<Option<bool>, String> {
        let input =
            if direction == Pos::N { 1 }
            else if direction == Pos::S { 2 }
            else if direction == Pos::W { 3 }
            else if direction == Pos::E { 4 }
            else { panic!("Invalid direction") };
        self.machine.input.0.push(input);
        while self.machine.output.is_empty() {
            if self.machine.step(&mut self.data).is_err() {
                return Err("Machine halted".to_string());
            }
        }
        match self.machine.output.pop().unwrap() {
            0 => Ok(None),
            1 => Ok(Some(false)),
            2 => Ok(Some(true)),
            v => panic!("Invalid status code {v}"),
        }
    }
    pub fn generate_map(&self) -> Map {
        let mut known = Map { known: HashMap::new() };
        let mut to_explore: Vec<(Pos, Self)> = vec![(self.pos, self.clone())];
        let mut seen = HashSet::new();
        while let Some((pos, mut state)) = to_explore.pop() {
            seen.insert(pos);
            for offset in [Pos::N, Pos::S, Pos::W, Pos::E] {
                let new_pos = pos.add(offset);
                if known.known.contains_key(&new_pos) { continue }
                match state.step(offset).unwrap() {
                    None => {
                        known.known.insert(new_pos, Tile::Wall);
                        continue;
                    },
                    Some(false) => known.known.insert(new_pos, Tile::Free),
                    Some(true)  => known.known.insert(new_pos, Tile::Goal),
                };

                if seen.contains(&new_pos) { continue }
                to_explore.push((new_pos, state.clone()));

                state.step(offset.neg()).unwrap();
            }
        }

        known
    }
}

pub fn part1(input: &str) -> Scalar {
    let data = parse_input(input);
    let robot = Robot::new(data);
    let map = robot.generate_map();
    map.dist_from(map.target(), Pos::ZERO).unwrap()
}

pub fn part2(input: &str) -> i16 {
    let data = parse_input(input);
    let robot = Robot::new(data);
    let map = robot.generate_map();
    map.dist_from(map.target(), Pos { x: Scalar::MIN, y: Scalar::MIN }).unwrap_err()
}

fn parse_input(input: &str) -> Vec<isize> {
    intcode_2019::parse_program(input, 0)
}
