#![feature(portable_simd)]

type PosType = u8;

aoc_tools::aoc_sol!(day14 2024: part1, part2);
aoc_tools::pos!(PosType);
aoc_tools::fast_hash!();

const PART_1_ITERS: PosType = 100;
pub fn part1(input: &str) -> usize {
    let mut robots = parse_input(input);

    robots.iter_mut().for_each(|robot| {
        robot.step_n(PART_1_ITERS);
    });

    let quadrant_ul = robots.iter().filter(|r| r.quadrant() == (-1, -1)).count();
    let quadrant_ur = robots.iter().filter(|r| r.quadrant() == (-1,  1)).count();
    let quadrant_dl = robots.iter().filter(|r| r.quadrant() == ( 1, -1)).count();
    let quadrant_dr = robots.iter().filter(|r| r.quadrant() == ( 1,  1)).count();

    quadrant_ul * quadrant_ur * quadrant_dl * quadrant_dr
}

pub fn part2(input: &str) -> usize {
    let mut robots: Vec<_> = parse_input(input);

    let mut step = 0;

    let mut best_christmas_tree_val = u16::MAX;
    let mut best_christmas_tree_step = step;

    while step < FIELD_MODULO.x as usize * FIELD_MODULO.y as usize {
        let christmas_untreeness = untreeness_and_step(&mut robots);
        if christmas_untreeness < best_christmas_tree_val {
            best_christmas_tree_val = christmas_untreeness;
            best_christmas_tree_step = step;
        }
        step += 1;
    }

    best_christmas_tree_step
}

#[inline(never)]
fn parse_input(input: &str) -> Vec<Robot> {
    fn parse_i16(s: &str) -> i16 {
        aoc_tools::parse_unsigned!(parse_u16<u16>(<= 8 digits));
        let (negate, s) = if s.as_bytes()[0] == b'-' {
            (true, &s[1..])
        } else {
            (false, s)
        };
        parse_u16(s) as i16 * if negate { -1 } else { 1 }
    }

    // 12 bytes is the minimum length of a robot
    let mut output = Vec::with_capacity(input.len() / 12);

    input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| &line[2..])
        .map(|line| line.split_once(' ').unwrap())
        .map(|(pos, vel)| (pos, &vel[2..]))
        .map(|(pos, vel)| {
            let (pos_x, pos_y) = pos.split_once(',').unwrap();
            let (vel_x, vel_y) = vel.split_once(',').unwrap();

            Robot {
                pos: Pos { x: parse_i16(pos_x) as u8, y: parse_i16(pos_y) as u8 },
                vel: Robot::fix_vel_positive(parse_i16(vel_x), parse_i16(vel_y)),
            }
        })
        .for_each(|r| output.push(r));

    output
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
    pos: Pos,
    vel: Pos,
}

// const FIELD_MODULO: Pos = Pos { x: 11, y: 7 };
const FIELD_MODULO: Pos = Pos { x: 101, y: 103 };

impl Robot {
    pub fn step(&mut self) {
        self.pos = self.pos.add(self.vel);
        self.pos.x = self.pos.x - if self.pos.x >= FIELD_MODULO.x { FIELD_MODULO.x } else { 0 };
        self.pos.y = self.pos.y - if self.pos.y >= FIELD_MODULO.y { FIELD_MODULO.y } else { 0 };
    }
    pub fn step_n(&mut self, n: PosType) {
        let coeff_x = n % FIELD_MODULO.x;
        let coeff_y = n % FIELD_MODULO.y;

        let new_pos_x = self.pos.x as u16 + self.vel.x as u16 * coeff_x as u16;
        let new_pos_y = self.pos.y as u16 + self.vel.y as u16 * coeff_y as u16;
        
        let new_pos_x = new_pos_x % FIELD_MODULO.x as u16;
        let new_pos_y = new_pos_y % FIELD_MODULO.y as u16;
        
        self.pos.x = new_pos_x as PosType;
        self.pos.y = new_pos_y as PosType;
    }

    pub fn quadrant(&self) -> (i8, i8) {
        let x = if self.pos.x < FIELD_MODULO.x / 2 {
            -1
        } else if self.pos.x == FIELD_MODULO.x / 2 {
            0
        } else {
            1
        };
        let y = if self.pos.y < FIELD_MODULO.y / 2 {
            -1
        } else if self.pos.y == FIELD_MODULO.y / 2 {
            0
        } else {
            1
        };

        (x, y)
    }

    pub fn fix_vel_positive(x: i16, y: i16) -> Pos {
        Pos {
            x: x.rem_euclid(FIELD_MODULO.x as i16) as PosType,
            y: y.rem_euclid(FIELD_MODULO.y as i16) as PosType,
        }
    }
}

#[inline(never)]
fn untreeness_and_step(robots: &mut [Robot]) -> u16 {
    assert!(!robots.is_empty());

    let mut sum_x: u16 = 0;
    let mut sum_y: u16 = 0;

    for robot in &*robots {
        sum_x += robot.pos.x as u16;
        sum_y += robot.pos.y as u16;
    }

    let mean_x = sum_x / robots.len() as u16;
    let mean_y = sum_y / robots.len() as u16;

    let mut n_mad = 0;
    for robot in robots {
        let diff_x = (robot.pos.x as u16).abs_diff(mean_x);
        let diff_y = (robot.pos.y as u16).abs_diff(mean_y);

        robot.step();

        n_mad += diff_x + diff_y;
    }

    n_mad
}

#[allow(dead_code)]
fn print_state(robots: &[Robot]) {
    for y in 0..FIELD_MODULO.y {
        for x in 0..FIELD_MODULO.x {
            if robots.iter().any(|r| r.pos == Pos { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
