aoc_tools::aoc_sol!(day25 2018: part1);

type Scalar = i16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos4 { x: Scalar, y: Scalar, z: Scalar, t: Scalar }
#[allow(unused)]
impl Pos4 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0, t: 0 };

    pub const U1000: Self = Self { x: 1, y: 0, z: 0, t: 0 };
    pub const U0100: Self = Self { x: 0, y: 1, z: 0, t: 0 };
    pub const U0010: Self = Self { x: 0, y: 0, z: 1, t: 0 };
    pub const U0001: Self = Self { x: 0, y: 0, z: 0, t: 1 };

    pub const fn new(x: Scalar, y: Scalar, z: Scalar, t: Scalar) -> Self {
        Self { x, y, z, t }
    }
    pub const fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            t: self.t + other.t,
        }
    }
    pub const fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            t: self.t - other.t,
        }
    }
    pub const fn mul(&self, scalar: Scalar) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            t: self.t * scalar,
        }
    }
    pub const fn div(&self, scalar: Scalar) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            t: self.t / scalar,
        }
    }
    pub const fn neg(&self) -> Self {
        self.mul(-1)
    }
    pub const fn manhattan(&self, other: &Self) -> Scalar {
        let diff = other.sub(self);
        diff.x.abs() + diff.y.abs() + diff.z.abs() + diff.t.abs()
    }
}

pub fn part1(input: &str) -> usize {
    let points = parse_input(input);
    let mut distances = vec![HashSet::new(); points.len()];
    for a in 0..points.len() {
        for b in a..points.len() {
            if points[a].manhattan(&points[b]) <= 3 {
                distances[a].insert(b);
                distances[b].insert(a);
            }
        }
    }
    let mut included = HashSet::<usize>::new();
    let mut constellations = vec![];
    loop {
        let Some(next) = (0..points.len()).find(|v| !included.contains(v)) else { break };
        let mut processed = HashSet::new();
        let mut to_process = vec![next];
        while let Some(point) = to_process.pop() {
            if processed.contains(&point) || included.contains(&point) { continue }
            processed.insert(point);
            to_process.extend(distances[point].iter().filter(|v| !processed.contains(v)).copied());
        }
        included.extend(processed.iter().copied());
        constellations.push(processed);
    }
    constellations.len()
}

pub fn part2(_input: &str) -> () {}

fn parse_input(input: &str) -> Vec<Pos4> {
    input.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (x, l) = l.split_once(',').unwrap();
            let (y, l) = l.split_once(',').unwrap();
            let (z, t) = l.split_once(',').unwrap();

            let x = x.trim().parse::<Scalar>().unwrap();
            let y = y.trim().parse::<Scalar>().unwrap();
            let z = z.trim().parse::<Scalar>().unwrap();
            let t = t.trim().parse::<Scalar>().unwrap();

            Pos4 { x, y, z, t }
        })
        .collect()
}
