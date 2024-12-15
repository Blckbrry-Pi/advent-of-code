#![feature(array_chunks)]
#![feature(portable_simd)]

type PosType = u8;

aoc_tools::aoc_sol!(day14: part1, part2);
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
    // for i in 0..1000 {
        let mut robots: Vec<_> = parse_input(input);
    
        let mut seen = HashSet::with_capacity(2_usize.pow(14));
        let mut step = 0;
    
        let mut best_christmas_tree_val = 0;
        let mut best_christmas_tree_step = step;
    
        // Ignore collisions, they're unlikely to happen with a maximum cycle
        // length of 5201500 in a u32
        while seen.insert(hash_robots(&robots)) {
            let christmas_treeness = treeness(&robots);
            if christmas_treeness > best_christmas_tree_val {
                best_christmas_tree_val = christmas_treeness;
                best_christmas_tree_step = step;
            }
    
            robots.iter_mut().for_each(|r| { r.step(); });
            step += 1;
        }
    
        // println!("{}", seen.len());

        best_christmas_tree_step
    // }

    // 0
}

#[inline(never)]
fn parse_input(input: &str) -> Vec<Robot> {
    fn parse_i16(s: &str) -> i16 {
        let (negate, s) = if s.as_bytes()[0] == b'-' {
            (true, &s[1..])
        } else {
            (false, s)
        };
    
        let mut val = 0;
        for i in (0..s.len()).rev() {
            val *= 10;
            val += (s.as_bytes()[i] - b'0') as i16;
        }
    
        val * if negate { -1 } else { 1 }
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
fn treeness(robots: &[Robot]) -> u16 {
    const MAX_MAD_DEV: u16 = (FIELD_MODULO.x / 2 + FIELD_MODULO.y / 2 + 2) as u16;

    assert!(!robots.is_empty());

    let mut sum_x: u16 = 0;
    let mut sum_y: u16 = 0;

    for robot in robots {
        sum_x += robot.pos.x as u16;
        sum_y += robot.pos.y as u16;
    }

    let mean_x = sum_x / robots.len() as u16;
    let mean_y = sum_y / robots.len() as u16;

    let mut n_mad = 0;
    for robot in robots {
        let diff_x = (robot.pos.x as u16).abs_diff(mean_x);
        let diff_y = (robot.pos.y as u16).abs_diff(mean_y);

        // n_stddev_sq += diff_x * diff_x + diff_y * diff_y;
        n_mad += diff_x + diff_y;
    }

    let stddev_sq = n_mad / robots.len() as u16;

    MAX_MAD_DEV - stddev_sq
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

fn hash_robots(robots: &[Robot]) -> u32 {
    const CONCURRENT: usize = 16;

    assert_eq!(u32::BITS / 8, std::mem::size_of::<Robot>() as u32);
    type SimdType = std::simd::Simd<u32, CONCURRENT>;

    let mut vals = SimdType::splat(3037000507);

    for i in 0..robots.len() / CONCURRENT {
        let robots_slice = &robots[i*CONCURRENT..(i+1)*CONCURRENT];
        let u32_slice: &[u32] = unsafe { std::mem::transmute(robots_slice) };
        unsafe { std::hint::assert_unchecked(u32_slice.len() == CONCURRENT) };
        let robot_vals = SimdType::from_slice(u32_slice);
        vals = SimdType::from_array(vals.as_array().map(|v| v.rotate_left(15)));
        vals ^= robot_vals;
    }

    std::simd::num::SimdUint::reduce_xor(vals)
}
