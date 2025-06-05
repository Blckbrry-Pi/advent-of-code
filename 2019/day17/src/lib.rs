use intcode_2019::Machine;

type Scalar = i16;

aoc_tools::aoc_sol!(day17 2019: part1, part2);
aoc_tools::map_struct!(Map of Tile, pos Scalar; +y=>D);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Scaffolding,
    Robot(Pos),
    DriftingRobot,
}
impl Tile {
    pub fn parse(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Scaffolding,
            'X' => Self::DriftingRobot,
            '^' => Self::Robot(Pos::U),
            'v' => Self::Robot(Pos::D),
            '<' => Self::Robot(Pos::L),
            '>' => Self::Robot(Pos::R),
            v => panic!("Invalid tile {v}"),
        }
    }
}
impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            Self::Empty => '.',
            Self::Scaffolding => '#',
            Self::DriftingRobot => 'X',
            Self::Robot(pos) => {
                if pos == Pos::U { '^' }
                else if pos == Pos::D { 'v' }
                else if pos == Pos::L { '<' }
                else if pos == Pos::R { '>' }
                else { '?' }
            }
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    TurnL,
    TurnR,
    Move(u8),
}
impl Move {
    pub fn get_abc(moves: &[Move]) -> (&[Move], &[Move], &[Move]) {
        // 1- 10 moves per function
        for a_len in 1..moves.len().min(10) {
            let Some(a) = moves.get(0..a_len) else { continue };

            let mut main_routine_len = 1;
            let mut moves = &moves[a_len..];
            loop {
                let Some(new_moves) = moves.strip_prefix(a) else { break };
                moves = new_moves;
                main_routine_len += 1;
            }
            if main_routine_len > 10 { continue }
            for b_len in 1..moves.len().min(10) {
                let Some(b) = moves.get(0..b_len) else { continue };

                let mut main_routine_len = main_routine_len + 1;
                let mut moves = &moves[b_len..];

                loop {
                    let Some(new_moves) = moves.strip_prefix(a).or(moves.strip_prefix(b)) else { break };
                    moves = new_moves;
                    main_routine_len += 1;
                }
                if main_routine_len > 10 { continue }
                for c_len in 1..moves.len().min(10) {
                    let Some(c) = moves.get(0..c_len) else { continue };

                    let mut main_routine_len = main_routine_len + 1;
                    let mut moves = &moves[c_len..];

                    loop {
                        let Some(new_moves) = moves.strip_prefix(a).or(moves.strip_prefix(b)).or(moves.strip_prefix(c)) else { break };
                        moves = new_moves;
                        main_routine_len += 1;
                    }
                    if main_routine_len > 10 { continue }
                    if moves.is_empty() { return (a, b, c) }
                }
            }
        }
        panic!("No valid abc found")
    }
    pub fn gen_routine(mut moves: &[Move], functions: (&[Move], &[Move], &[Move])) -> String {
        let mut output = String::new();
        while !moves.is_empty() {
            if moves.is_empty() {
                break
            } else if let Some(new_moves) = moves.strip_prefix(functions.0) {
                moves = new_moves;
                output.push('A');
            } else if let Some(new_moves) = moves.strip_prefix(functions.1) {
                moves = new_moves;
                output.push('B');
            } else if let Some(new_moves) = moves.strip_prefix(functions.2) {
                moves = new_moves;
                output.push('C');
            } else {
                panic!("Invalid functions for routine");
            }
            output.push(',');
        }
        output.pop();
        output
    }
    pub fn gen_fn(moves: &[Move]) -> String {
        use std::fmt::Write;
        let mut output = String::new();
        for i in 0..moves.len() {
            if i != 0 { output.push(','); }
            write!(&mut output, "{}", moves[i]).unwrap();
        }
        output
    }
}
impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TurnL => write!(f, "L"),
            Self::TurnR => write!(f, "R"),
            Self::Move(d) => write!(f, "{d}"),
        }
    }
}

impl Map {
    pub fn find_intersections(&self) -> impl Iterator<Item = Pos> + '_ {
        let height = self.height() as Scalar;
        let width = self.width() as Scalar;
        (0..height)
            .flat_map(move |y| (0..width).map(move |x| Pos { x, y }))
            .filter(|pos| {
                let to_check = [
                    *pos,
                    pos.add(Pos::U),
                    pos.add(Pos::D),
                    pos.add(Pos::L),
                    pos.add(Pos::R),
                ];
                to_check.into_iter().all(|neighbor| matches!(
                    self.get_raw(neighbor),
                    Some(Tile::Robot(_) | Tile::Scaffolding),
                ))
            })
    }
    pub fn get_full_directions(&self) -> Vec<Move> {
        let mut pos = self.find(|t| matches!(t, Tile::Robot(_))).unwrap();
        let &Tile::Robot(mut direction) = self.get_raw(pos).unwrap() else { panic!("Robot is not a robot") };
        let mut moves = vec![];
        let mut curr_movement_run = 0;
        loop {
            match self.get_raw(pos.add(direction)) {
                Some(Tile::Scaffolding) => {
                    curr_movement_run += 1;
                    pos = pos.add(direction);
                },
                _ => {
                    if curr_movement_run > 0 {
                        moves.push(Move::Move(curr_movement_run));
                        curr_movement_run = 0;
                    }
                    if matches!(self.get_raw(pos.add(direction.turn_r())), Some(Tile::Scaffolding)) {
                        direction = direction.turn_r();
                        moves.push(Move::TurnR);
                    } else if matches!(self.get_raw(pos.add(direction.turn_l())), Some(Tile::Scaffolding)) {
                        direction = direction.turn_l();
                        moves.push(Move::TurnL);
                    } else {
                        break;
                    }
                }
            }
        }
        if curr_movement_run > 0 {
            moves.push(Move::Move(curr_movement_run));
        }
        moves
    }
}

fn get_map(machine: &mut Machine, data: &mut [isize]) -> Map {
    let mut map = String::new();
    while machine.step(data).is_ok() {
        let Some(byte) = machine.output.pop() else { continue };
        map.push(byte as u8 as char);
    }
    Map { rows: aoc_tools::parse_map(&map, Tile::parse) }
}

pub fn part1(input: &str) -> Scalar {
    let mut data = parse_input(input);
    let mut machine = Machine::new(vec![]);
    let map = get_map(&mut machine, &mut data);
    let mut sum = 0;
    for intersection in map.find_intersections() {
        sum += intersection.x * intersection.y;
    }
    sum
}

pub fn part2(input: &str) -> isize {
    let mut data = parse_input(input);
    let mut machine = Machine::new(vec![]);
    let map = get_map(&mut machine.clone(), &mut data.clone());

    data[0] = 2;

    let directions = map.get_full_directions();
    let functions = Move::get_abc(&directions);
    let main_routine = Move::gen_routine(&directions, functions);
    let functions = (Move::gen_fn(functions.0), Move::gen_fn(functions.1), Move::gen_fn(functions.2));
    
    for &byte in main_routine.as_bytes() {
        machine.input.0.push(byte as isize);
    }
    machine.input.0.push(b'\n' as isize);

    for &byte in functions.0.as_bytes() {
        machine.input.0.push(byte as isize);
    }
    machine.input.0.push(b'\n' as isize);

    for &byte in functions.1.as_bytes() {
        machine.input.0.push(byte as isize);
    }
    machine.input.0.push(b'\n' as isize);

    for &byte in functions.2.as_bytes() {
        machine.input.0.push(byte as isize);
    }
    machine.input.0.push(b'\n' as isize);
    machine.input.0.push(b'n' as isize);
    machine.input.0.push(b'\n' as isize);

    while machine.step(&mut data).is_ok() {}

    machine.output.pop().unwrap()
}

fn parse_input(input: &str) -> Vec<isize> {
    intcode_2019::parse_program(input, 4096)
}
