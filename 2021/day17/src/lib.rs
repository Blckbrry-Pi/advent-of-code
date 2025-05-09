use std::num::NonZeroU16;

type Scalar = i32;

aoc_tools::aoc_sol!(day17 2021: part1, part2);
aoc_tools::pos!(Scalar; +y => U);

#[derive(Debug, Clone, PartialEq)]
struct YHitRange {
    discrete_times: HashMap<u16, Vec<Scalar>>,
    max: u16,
}
impl YHitRange {
    pub fn empty() -> Self {
        Self {
            discrete_times: HashMap::new(),
            max: 0,
        }
    }
    pub fn add(&mut self, y: Scalar, hit_range: HitRange) {
        for time in hit_range.discrete_times {
            self.discrete_times.entry(time).or_default().push(y);
            self.max = self.max.max(time);
        }
    }
    pub fn intersection_count(&self, hit_range: &HitRange) -> i32 {
        let mut all_y_vels = HashSet::new();
        for &time in &hit_range.discrete_times {
            if let Some(y_vels) = self.discrete_times.get(&time) {
                all_y_vels.extend(y_vels.iter().copied());
            }
        }
        if let Some(after) = hit_range.after {
            let after = after.get();
            for time in after..=self.max {
                let Some(y_vels) = self.discrete_times.get(&time) else { continue };
                all_y_vels.extend(y_vels.iter().copied());
            }
        }
        all_y_vels.len() as i32
    }
}

#[derive(Debug, Clone, PartialEq)]
struct HitRange {
    discrete_times: HashSet<u16>,
    after: Option<NonZeroU16>,
}
impl HitRange {
    pub fn empty() -> Self {
        Self {
            discrete_times: HashSet::new(),
            after: None,
        }
    }
    pub fn add(&mut self, other: &Self) {
        self.discrete_times.extend(other.discrete_times.iter().copied());
        self.after = match (self.after, other.after) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };
    }
    pub fn intersects(&self, other: &Self) -> Option<u16> {
        if let Some(&matching) = self.discrete_times.intersection(&other.discrete_times).next() {
            return Some(matching);
        }
        match (self.after, other.after) {
            (Some(a), Some(b)) => Some(a.min(b).into()),
            (Some(a), None) => other.discrete_times.iter().copied().find(|&v| a.get() <= v),
            (None, Some(b)) => self .discrete_times.iter().copied().find(|&v| b.get() <= v),
            (None, None) => None,
        }
    }
}

fn min_x_vel(min_target_x: Scalar) -> Scalar {
    (min_target_x * 2).isqrt()
}
fn x_hits_target(x_vel: Scalar, x_range: (Scalar, Scalar)) -> Option<HitRange> {
    let mut x = 0;
    let mut x_vel = x_vel;
    let mut i = 0;
    let mut discrete_times = HashSet::new();
    while x <= x_range.1 && x_vel != 0 {
        if x_range.0 <= x && x <= x_range.1 {
            discrete_times.insert(i);
        }
        x += x_vel;
        x_vel = (x_vel.abs() - 1).max(0) * x_vel.signum();
        i += 1;
    }
    if x_vel == 0 && x_range.0 <= x && x <= x_range.1 {
        Some(HitRange {
            discrete_times,
            after: NonZeroU16::new(i)
        })
    } else if !discrete_times.is_empty() {
        Some(HitRange {
            discrete_times,
            after: None,
        })
    } else {
        None
    }
}
fn valid_x_vals_iter(x_range: (Scalar, Scalar)) -> impl Iterator<Item = (Scalar, HitRange)> {
    let min = min_x_vel(x_range.0);
    let max = x_range.1;
    (min..=max).filter_map(move |x_vel| Some((x_vel, x_hits_target(x_vel, x_range)?)))
}

// After 2 * y_vel + 1 steps, we're at ?,0 with a velocity of -(y_vel + 1)
// This only works if the target is below the starting point, which I think
// always is the case with the AoC input
fn max_y_vel(y_range: (Scalar, Scalar)) -> Scalar {
    assert!(y_range.0 < 0);
    1 - y_range.0
}
fn y_hits_target(y_vel: Scalar, y_range: (Scalar, Scalar)) -> Option<HitRange> {
    let mut y = 0;
    let mut y_vel = y_vel;
    let mut i = 0;
    let mut discrete_times = HashSet::new();
    while y >= y_range.0 {
        if y_range.0 <= y && y <= y_range.1 {
            discrete_times.insert(i);
        }
        y += y_vel;
        y_vel -= 1;
        i += 1;
    }
    if !discrete_times.is_empty() {
        Some(HitRange {
            discrete_times,
            after: None,
        })
    } else {
        None
    }
}
fn valid_y_vals_iter(y_range: (Scalar, Scalar)) -> impl Iterator<Item = (Scalar, HitRange)> {
    let max = max_y_vel(y_range);
    let min = y_range.0;
    (min..=max).rev().filter_map(move |y_vel| Some((y_vel, y_hits_target(y_vel, y_range)?)))
}

pub fn part1(input: &str) -> i32 {
    let (min, max) = parse_input(input);
    let valid_x_vals = valid_x_vals_iter((min.x, max.x))
        .fold(HitRange::empty(), |mut curr, (_, next)| {
            curr.add(&next);
            curr
        });
    let max_y_vel = valid_y_vals_iter((min.y, max.y))
        .find(|(_, hit_range)| valid_x_vals.intersects(hit_range).is_some())
        .unwrap()
        .0;
    max_y_vel * (max_y_vel + 1) / 2
}

pub fn part2(input: &str) -> i32 {
    let (min, max) = parse_input(input);

    let mut count = 0;
    let valid_y_vals = valid_y_vals_iter((min.y, max.y))
        .fold(YHitRange::empty(), |mut range, (y, y_hit_range)| {
            range.add(y, y_hit_range);
            range
        });
    for (_, x_hit_range) in valid_x_vals_iter((min.x, max.x)) {
        count += valid_y_vals.intersection_count(&x_hit_range);
    }
    count
}

fn parse_input(input: &str) -> (Pos, Pos) {
    let input = input.trim().trim_start_matches("target area: x=");
    let (x, y) = input.split_once(", y=").unwrap();
    let (x_min, x_max) = x.split_once("..").unwrap();
    let (y_min, y_max) = y.split_once("..").unwrap();

    let (x_min, x_max) = (x_min.parse().unwrap(), x_max.parse().unwrap());
    let (y_min, y_max) = (y_min.parse().unwrap(), y_max.parse().unwrap());

    (
        Pos { x: x_min, y: y_min },
        Pos { x: x_max, y: y_max },
    )
}
