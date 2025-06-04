use std::str::FromStr;

use aoc_tools::lcm;

type Scalar = i64;

aoc_tools::aoc_sol!(day12 2019: part1, part2);
aoc_tools::pos3!(Scalar);

#[derive(Clone, Copy, PartialEq, Eq)]
struct Moon {
    pos: Pos3,
    vel: Pos3,
}
impl Moon {
    pub fn do_gravity(&mut self, other: &mut Self) {
        let dvx = (other.pos.x - self.pos.x).signum();
        let dvy = (other.pos.y - self.pos.y).signum();
        let dvz = (other.pos.z - self.pos.z).signum();

        self.vel.x += dvx;
        other.vel.x -= dvx;

        self.vel.y += dvy;
        other.vel.y -= dvy;

        self.vel.z += dvz;
        other.vel.z -= dvz;
    }
    pub fn step(&mut self) {
        self.pos = self.pos.add(self.vel);
    }
    pub fn kin(&self) -> Scalar {
        self.vel.manhattan(Pos3::ZERO)
    }
    pub fn pot(&self) -> Scalar {
        self.pos.manhattan(Pos3::ZERO)
    }
    pub fn total_energy(&self) -> Scalar {
        self.kin() * self.pot()
    }
}
impl FromStr for Moon {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((x, rest)) = s.split_once(", y=") else {
            return Err("Missing y coord".to_string());
        };
        let Some((y, z)) = rest.split_once(", z=") else {
            return Err("Missing z coord".to_string());
        };
        let Some(x) = x.strip_prefix("<x=") else {
            return Err("Missing x coord".to_string());
        };
        let Some(z) = z.strip_suffix('>') else {
            return Err("Improper format".to_string());
        };
        let x = x.parse::<Scalar>().map_err(|e| e.to_string())?;
        let y = y.parse::<Scalar>().map_err(|e| e.to_string())?;
        let z = z.parse::<Scalar>().map_err(|e| e.to_string())?;

        Ok(Self {
            pos: Pos3 { x, y, z },
            vel: Pos3::ZERO,
        })
    }
}

fn do_step(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in i+1..moons.len() {
            let [a, b] = moons.get_disjoint_mut([i, j]).unwrap();
            a.do_gravity(b);
        }
    }
    for moon in moons {
        moon.step();
    }
}

fn axis_equivalent(a: &[Moon], b: &[Moon]) -> (bool, bool, bool) {
    let mut matches = (true, true, true);
    for (&a, &b) in a.iter().zip(b) {
        if a.pos.x != b.pos.x || a.vel.x != b.vel.x { matches.0 = false }
        if a.pos.y != b.pos.y || a.vel.y != b.vel.y { matches.1 = false }
        if a.pos.z != b.pos.z || a.vel.z != b.vel.z { matches.2 = false }
    }
    matches
}
fn get_periods(start: &[Moon]) -> (Scalar, Scalar, Scalar) {
    let mut periods = (None, None, None);
    let mut moons = start.to_vec();
    let mut i = 0;
    while periods.0.is_none() || periods.1.is_none() || periods.2.is_none() {
        do_step(&mut moons);
        i += 1;

        let curr_matches = axis_equivalent(&moons, start);
        if curr_matches.0 && periods.0.is_none() {
            periods.0 = Some(i);
        }
        if curr_matches.1 && periods.1.is_none() {
            periods.1 = Some(i);
        }
        if curr_matches.2 && periods.2.is_none() {
            periods.2 = Some(i);
        }
        if i % 10_000_000 == 0 { println!("{periods:?}"); }
    }
    (periods.0.unwrap(), periods.1.unwrap(), periods.2.unwrap())
}

pub fn part1(input: &str) -> Scalar {
    let mut moons = parse_input(input);
    for _ in 0..1000 { do_step(&mut moons) }
    moons.iter().map(|m| m.total_energy()).sum()
}

pub fn part2(input: &str) -> Scalar {
    let moons = parse_input(input);
    let (x_period, y_period, z_period) = get_periods(&moons);
    lcm(lcm(x_period, y_period), z_period)
}

fn parse_input(input: &str) -> Vec<Moon> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}
