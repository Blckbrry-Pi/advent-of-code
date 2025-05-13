use std::str::FromStr;
aoc_tools::aoc_sol!(day22 2021: part1, part2);

type Scalar = i64;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Cube {
    x: (Scalar, Scalar),
    y: (Scalar, Scalar),
    z: (Scalar, Scalar),
    on: bool,
}
impl Cube {
    pub fn x_len(&self) -> Scalar {
        (self.x.1 + 1 - self.x.0).max(0)
    }
    pub fn y_len(&self) -> Scalar {
        (self.y.1 + 1 - self.y.0).max(0)
    }
    pub fn z_len(&self) -> Scalar {
        (self.z.1 + 1 - self.z.0).max(0)
    }
    pub fn volume(&self) -> Scalar {
        self.x_len() * self.y_len() * self.z_len()
    }
    pub fn overlap(&self, other: &Self) -> Option<Self> {
        let x = (self.x.0.max(other.x.0), self.x.1.min(other.x.1));
        let y = (self.y.0.max(other.y.0), self.y.1.min(other.y.1));
        let z = (self.z.0.max(other.z.0), self.z.1.min(other.z.1));
        if x.1 < x.0 || y.1 < y.0 || z.1 < z.0 {
            None
        } else {
            Some(Self { x, y, z, on: self.on })
        }
    }
    pub fn minus(&self, other: &Self) -> Vec<Self> {
        let Some(overlap) = self.overlap(other) else { return vec![*self] };
        let mut outputs = vec![];
        for face in 0..6 {
            let x = if face == 0 {
                (self.x.0, other.x.0-1)
            } else if face == 1 {
                (other.x.1+1, self.x.1)
            } else {
                overlap.x
            };
            let y = if face < 2 {
                self.y
            } else if face == 2 {
                (self.y.0, other.y.0-1)
            } else if face == 3 {
                (other.y.1+1, self.y.1)
            } else {
                overlap.y
            };
            let z = if face == 4 {
                (self.z.0, other.z.0-1)
            } else if face == 5 {
                (other.z.1+1, self.z.1)
            } else {
                self.z
            };
            if x.1 >= x.0 && y.1 >= y.0 && z.1 >= z.0 {
                outputs.push(Self { x, y, z, on: self.on });
            }
        }
        outputs
    }
    pub fn is_initialization(&self) -> bool {
        let x_good = self.x.0 >= -50 && self.x.1 <= 50;
        let y_good = self.y.0 >= -50 && self.y.1 <= 50;
        let z_good = self.z.0 >= -50 && self.z.1 <= 50;
        x_good && y_good && z_good
    }
}
impl Debug for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.on {
            write!(f, "on  ")
        } else {
            write!(f, "off ")
        }?;
        write!(f, "x={}..{}, ", self.x.0, self.x.1)?;
        write!(f, "y={}..{}, ", self.y.0, self.y.1)?;
        write!(f, "z={}..{}, ", self.z.0, self.z.1)?;
        write!(f, "vol={}", self.volume())
    }
}
impl FromStr for Cube {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (on, s) = if let Some(s) = s.strip_prefix("on x=") {
            (true, s)
        } else if let Some(s) = s.strip_prefix("off x=") {
            (false, s)
        } else {
            return Err("Missing `on` or `off` prefix".to_string());
        };
        let Some((x, s)) = s.split_once(",y=") else { return Err("Missing comma".to_string()) };
        let Some((y, z)) = s.split_once(",z=") else { return Err("Missing comma".to_string()) };
        let Some((x_lo, x_hi)) = x.split_once("..") else { return Err("Invalid x range".to_string()) };
        let Some((y_lo, y_hi)) = y.split_once("..") else { return Err("Invalid y range".to_string()) };
        let Some((z_lo, z_hi)) = z.split_once("..") else { return Err("Invalid z range".to_string()) };

        let x_lo = x_lo.parse::<Scalar>().map_err(|e| e.to_string())?;
        let x_hi = x_hi.parse::<Scalar>().map_err(|e| e.to_string())?;
        let y_lo = y_lo.parse::<Scalar>().map_err(|e| e.to_string())?;
        let y_hi = y_hi.parse::<Scalar>().map_err(|e| e.to_string())?;
        let z_lo = z_lo.parse::<Scalar>().map_err(|e| e.to_string())?;
        let z_hi = z_hi.parse::<Scalar>().map_err(|e| e.to_string())?;

        let x = (x_lo, x_hi);
        let y = (y_lo, y_hi);
        let z = (z_lo, z_hi);
        Ok(Self { x, y, z, on })
    }
}

#[derive(Debug)]
struct State {
    cubes: Vec<Cube>,
}
impl State {
    pub fn add_cube(&mut self, cube: Cube) {
        let mut to_add = Vec::with_capacity(self.cubes.len() * 3 + 1);
        to_add.push(cube);
        for existing in self.cubes.drain(..) {
            to_add.extend(existing.minus(&cube));
        }
        self.cubes = to_add
    }
    pub fn volume(&self) -> Scalar {
        self.cubes.iter().map(|cube| cube.volume() * cube.on as Scalar).sum()
    }
}

pub fn part1(input: &str) -> Scalar {
    let cubes = parse_input(input);
    let mut state = State { cubes: vec![] };

    for cube in cubes {
        if !cube.is_initialization() { break }
        state.add_cube(cube);
    }
    state.volume()
}

pub fn part2(input: &str) -> Scalar {
    let cubes = parse_input(input);
    let mut state = State { cubes: vec![] };

    for cube in cubes {
        state.add_cube(cube);
    }
    state.volume()
}

fn parse_input(input: &str) -> Vec<Cube> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(Cube::from_str)
        .map(Result::unwrap)
        .collect()
}
