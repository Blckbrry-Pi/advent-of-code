use intcode_2019::Machine;

type Scalar = i16;

aoc_tools::aoc_sol!(day13 2019: part1, part2);
aoc_tools::map_struct!(Map of Tile, pos Scalar; +y=>D);

impl Map {
    pub fn ball_pos(&self) -> Pos {
        for y in 0..self.height() as Scalar {
            for x in 0..self.width() as Scalar {
                if *self.get_raw(Pos { x, y }).unwrap() == Tile::Ball {
                    return Pos { x, y };
                }
            }
        }
        panic!("No balls")
    }
    pub fn paddle_pos(&self) -> Pos {
        for y in 0..self.height() as Scalar {
            for x in 0..self.width() as Scalar {
                if *self.get_raw(Pos { x, y }).unwrap() == Tile::Paddle {
                    return Pos { x, y };
                }
            }
        }
        panic!("No paddle")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}
impl Tile {
    fn from_int(n: u8) -> Self {
        match n {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            v => panic!("Invalid tile {v}"),
        }
    }
}
impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty  => write!(f, " "),
            Self::Wall   => write!(f, "█"),
            Self::Block  => write!(f, "▣"),
            Self::Paddle => write!(f, "—"),
            Self::Ball   => write!(f, "·"),
        }
    }
}

fn do_step(machine: &mut Machine, data: &mut [isize], map: &mut Map, first_frame: bool) -> Result<(Option<isize>, bool), String> {
    while machine.output.len() < 3 {
        machine.step(data).map_err(|_| "Invalid instruction".to_string())?;
    }
    let x = machine.output[0];
    let y = machine.output[1];
    let tile = machine.output[2];
    machine.output.clear();

    if (x, y) == (-1, 0) {
        Ok((Some(tile), false))
    } else {
        let pos = Pos { x: x as Scalar, y: y as Scalar };
        *map.get_mut_raw(pos).unwrap() = Tile::from_int(tile as u8);
        Ok((None, if first_frame { pos == BOTTOM_RIGHT } else { tile == 4 }))
    }
}
fn do_frame(machine: &mut Machine, data: &mut [isize], map: &mut Map, first_frame: bool) -> Result<Option<isize>, (Option<isize>, String)> {
    let mut score = None;
    loop {
        let (new_score, is_frame_end) = do_step(machine, data, map, first_frame).map_err(|e| (score, e))?;
        if new_score.is_some() {
            score = new_score;
        }
        if is_frame_end { break }
    }
    Ok(score)
}

const WIDTH: usize = 44;
const HEIGHT: usize = 24;
const BOTTOM_RIGHT: Pos = Pos { x: WIDTH as Scalar - 1, y: HEIGHT as Scalar - 1 };

pub fn part1(input: &str) -> usize {
    let mut data = parse_input(input);
    let mut machine = Machine::new(vec![]);
    let mut map = Map { rows: vec![vec![Tile::Empty; WIDTH]; HEIGHT] };

    loop {
        while !machine.halt && machine.output.len() < 3 {
            if machine.step(&mut data).is_err() { break }
        }
        if machine.output.len() < 3 { break }
        let x = machine.output[0];
        let y = machine.output[1];
        let tile = machine.output[2];
        machine.output.clear();

        let pos = Pos { x: x as Scalar, y: y as Scalar };
        *map.get_mut_raw(pos).unwrap() = Tile::from_int(tile as u8);
    }

    map.count_matching(|&t| t == Tile::Block)
}

pub fn part2(input: &str) -> isize {
    let mut data = parse_input(input);
    data[0] = 2;
    let mut machine = Machine::new(vec![]);
    let mut map = Map { rows: vec![vec![Tile::Empty; WIDTH]; HEIGHT] };

    do_frame(&mut machine, &mut data, &mut map, true).unwrap();

    'score: loop {
        let mut total_moves = 0;
        let target = {
            let mut temp_data = data.clone();
            let mut temp_machine = machine.clone();
            let mut temp_map = map.clone();
            temp_machine.input.0.push(0);
            loop {
                match do_frame(&mut temp_machine, &mut temp_data, &mut temp_map, false) {
                    Ok(_) => (),
                    Err((Some(final_score), _)) => break 'score final_score,
                    Err((None, err)) => panic!("Error when processing frame: {err}"),
                }

                let ball = temp_map.ball_pos();
                if ball.y == 21 { break ball.x }

                total_moves += 1;
                temp_machine.input.0.push(0);
            }
        };
        let paddle = map.paddle_pos();
        machine.input.0.clear();
        machine.input.1 = 0;
        for i in 0..total_moves + 11 {
            if i < (paddle.x - target).abs() {
                machine.input.0.push((target - paddle.x).signum() as isize);
            } else {
                machine.input.0.push(0);
            }
        }

        loop {
            do_frame(&mut machine, &mut data, &mut map, false).unwrap();
            if map.ball_pos().y == 20 { break }
            // println!("{map:?}");
            // std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }
}

fn parse_input(input: &str) -> Vec<isize> {
    intcode_2019::parse_program(input, 4096)
}
