use std::{collections::{HashMap, HashSet}, time::Instant};

fn main() {
    part1();
    part2();
}

#[allow(dead_code)]
const TEST: &str = include_str!("../../../data/2019/day03/test.txt");
const INPUT: &str = include_str!("../../../data/2019/day03/input.txt");

fn part1() {
    let start = Instant::now();
    let (wire_a, wire_b) = parse_input(INPUT);

    let pos = wire_a.intersection_point(&wire_b)
        .keys()
        .copied()
        .min_by_key(|a| a.0.abs() + a.1.abs());

    let out = pos.unwrap().0.abs() + pos.unwrap().1.abs();

    println!("Part 1: {out:?} {:?}", start.elapsed());
}

fn part2() {
    let start = Instant::now();
    let (wire_a, wire_b) = parse_input(INPUT);

    let out = wire_a.intersection_point(&wire_b)
        .into_iter()
        .min_by_key(|(_, steps)| *steps)
        .map(|(_, steps)| steps)
        .unwrap();

    println!("Part 2: {out} {:?}", start.elapsed());
}


fn parse_input(input: &'static str) -> (Wire, Wire) {
    let (wire_a, wire_b) = input.split_once('\n').unwrap();

    let wire_a = wire_a.split(',').map(WireSegment::from_str);
    let wire_b = wire_b.split(',').map(WireSegment::from_str);

    let wire_a = Wire::from_segments(wire_a);
    let wire_b = Wire::from_segments(wire_b);

    (wire_a, wire_b)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Wire {
    segments: Vec<WireSegment>,
    points: Vec<(isize, isize)>,
    distances: Vec<isize>,
}

impl Wire {
    pub fn from_segments(mut segments: impl Iterator<Item = WireSegment>) -> Self {
        let mut segment_vec = vec![];
        let mut point_vec = vec![(0, 0)];
        let mut dist_vec = vec![0];
        while let Some(segment) = segments.next() {
            let curr_point = *point_vec.last().unwrap();
            let new_point = (curr_point.0 + segment.dx(), curr_point.1 + segment.dy());
            point_vec.push(new_point);

            segment_vec.push(segment);

            let curr_dist = *dist_vec.last().unwrap() as isize;
            let new_dist = curr_dist + segment.dx().abs() + segment.dy().abs();
            dist_vec.push(new_dist);
        }

        Self {
            segments: segment_vec,
            points: point_vec,
            distances: dist_vec,
        }
    }

    pub fn intersection_point(&self, other: &Self) -> HashMap<(isize, isize), isize> {
        let mut intersections = HashMap::new();

        let points_a = self.points.iter().copied();
        let segments_a = self.segments.iter().copied();
        let dists_a = self.distances.iter().copied();
        for (wire_a, dist_a) in points_a.zip(segments_a).zip(dists_a) {
            let points_b = other.points.iter().copied();
            let segments_b = other.segments.iter().copied();
            let dists_b = other.distances.iter().copied();
            for (wire_b, dist_b) in points_b.zip(segments_b).zip(dists_b) {
                if let Some((x, y)) = WireSegment::intersects(wire_a, wire_b) {
                    let dist_a = dist_a as isize + (x - wire_a.0.0).abs() + (y - wire_a.0.1).abs();
                    let dist_b = dist_b as isize + (x - wire_b.0.0).abs() + (y - wire_b.0.1).abs();

                    let dist = dist_a + dist_b;
                    if x != 0 || y != 0 {
                        let curr_dist = intersections.get(&(x, y)).map(|v| *v).unwrap_or(isize::MAX);
                        intersections.insert((x, y), curr_dist.min(dist));
                    }
                }
            }
        }

        intersections
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WireSegment {
    Hori(isize),
    Vert(isize),
}

impl WireSegment {
    pub fn from_str(text: &str) -> Self {
        // println!("{}", &text[1..]);
        match text.chars().next().unwrap() {
            'L' => Self::Hori(-text[1..].parse::<isize>().unwrap()),
            'R' => Self::Hori(text[1..].parse::<isize>().unwrap()),
            'U' => Self::Vert(-text[1..].parse::<isize>().unwrap()),
            'D' => Self::Vert(text[1..].parse::<isize>().unwrap()),
            _ => unreachable!(),
        }
    }
    pub fn dx(&self) -> isize {
        match *self {
            Self::Hori(dx) => dx,
            Self::Vert(_) => 0,
        }
    }
    pub fn dy(&self) -> isize {
        match *self {
            Self::Hori(_) => 0,
            Self::Vert(dy) => dy,
        }
    }

    pub fn intersects(a: ((isize, isize), Self), b: ((isize, isize), Self)) -> Option<(isize, isize)> {
        match (a.1, b.1) {
            (Self::Hori(da), Self::Vert(db)) => {
                let a_start = a.0.0;
                let a_end = a.0.0 + da;
                let a_range = a_start.min(a_end)..=a_start.max(a_end);

                let b_start = b.0.1;
                let b_end = b.0.1 + db;
                let b_range = b_start.min(b_end)..=b_start.max(b_end);

                if a_range.contains(&b.0.0) && b_range.contains(&a.0.1) {
                    Some((b.0.0, a.0.1))
                } else {
                    None
                }
            },
            (Self::Vert(_), Self::Hori(_)) => Self::intersects(b, a),
            (Self::Vert(da), Self::Vert(db)) => {
                if a.0.0 != b.0.0 {
                    return None;
                }
                let a_start = a.0.1;
                let a_end = a.0.1 + da;
                let a_range = a_start.min(a_end)..=a_start.max(a_end);

                let b_start = b.0.1;
                let b_end = b.0.1 + db;
                let b_range = b_start.min(b_end)..=b_start.max(b_end);

                let intersect_start = (*a_range.start()).max(*b_range.start());
                let intersect_end = (*a_range.end()).min(*b_range.end());
                if intersect_start > intersect_end {
                    return None;
                }

                let y = if da == 0 || db == 0 {
                    intersect_start
                } else if da.signum() != db.signum() {
                    (intersect_start + intersect_end) / 2
                } else if da > 0 {
                    intersect_start
                } else {
                    intersect_end
                };

                Some((a.0.0, y))
            },
            (Self::Hori(da), Self::Hori(db)) => {
                let new_a = ((a.0.1, a.0.0), Self::Vert(da));
                let new_b = ((b.0.1, b.0.0), Self::Vert(db));
                let (y, x) = Self::intersects(new_a, new_b)?;
                Some((x, y))
            }
        }
    }
}
