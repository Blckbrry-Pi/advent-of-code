use std::str::FromStr;

type Scalar = i32;

aoc_tools::aoc_sol!(day12 2020: part1, part2);
aoc_tools::pos!(Scalar; +y => U);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    N(Scalar),
    S(Scalar),
    E(Scalar),
    W(Scalar),
    L(i16),
    R(i16),
    F(Scalar),
}
impl FromStr for Action {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(value) = s.get(1..) else {
            return Err("All actions should be ascii".to_string())
        };
        let value = value.parse::<Scalar>().map_err(|e| e.to_string())?;
        let output = match s.as_bytes()[0] {
            b'N' => Self::N(value),
            b'S' => Self::S(value),
            b'E' => Self::E(value),
            b'W' => Self::W(value),
            b'F' => Self::F(value),
            b'L' => Self::L(value as i16),
            b'R' => Self::R(value as i16),
            c => return Err(format!("Invalid command prefix {:?}", c as char))
        };
        Ok(output)
    }
}

#[derive(Debug, Clone, Copy)]
struct Ship {
    pos: Pos,
    waypoint: Pos,
}
impl Ship {
    pub fn take_action_p1(&mut self, action: Action) {
        match action {
            Action::N(s) => self.pos = self.pos.add(Pos::N.mul(s)),
            Action::S(s) => self.pos = self.pos.add(Pos::S.mul(s)),
            Action::E(s) => self.pos = self.pos.add(Pos::E.mul(s)),
            Action::W(s) => self.pos = self.pos.add(Pos::W.mul(s)),
            Action::L(s) => for _ in 0..s / 90 {
                self.waypoint = self.waypoint.turn_l();
            },
            Action::R(s) => for _ in 0..s / 90 {
                self.waypoint = self.waypoint.turn_r();
            },
            Action::F(s) => self.pos = self.pos.add(self.waypoint.mul(s)),
        }
    }
    pub fn take_action_p2(&mut self, action: Action) {
        match action {
            Action::N(s) => self.waypoint = self.waypoint.add(Pos::N.mul(s)),
            Action::S(s) => self.waypoint = self.waypoint.add(Pos::S.mul(s)),
            Action::E(s) => self.waypoint = self.waypoint.add(Pos::E.mul(s)),
            Action::W(s) => self.waypoint = self.waypoint.add(Pos::W.mul(s)),
            Action::L(s) => for _ in 0..s / 90 {
                self.waypoint = self.waypoint.turn_l();
            },
            Action::R(s) => for _ in 0..s / 90 {
                self.waypoint = self.waypoint.turn_r();
            },
            Action::F(s) => self.pos = self.pos.add(self.waypoint.mul(s)),
        }
    }
}

pub fn part1(input: &str) -> Scalar {
    let actions = parse_input(input);
    let mut ship = Ship {
        pos: Pos::ZERO,
        waypoint: Pos::E,
    };
    for action in actions {
        ship.take_action_p1(action);
    }
    ship.pos.manhattan(Pos::ZERO)
}

pub fn part2(input: &str) -> Scalar {
    let actions = parse_input(input);
    let mut ship = Ship {
        pos: Pos::ZERO,
        waypoint: Pos { x: 10, y: 1 },
    };
    for action in actions {
        ship.take_action_p2(action);
    }
    ship.pos.manhattan(Pos::ZERO)
}

fn parse_input(input: &str) -> Vec<Action> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}
