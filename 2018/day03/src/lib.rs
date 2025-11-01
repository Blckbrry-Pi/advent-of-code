aoc_tools::aoc_sol!(day03 2018: part1, part2);
aoc_tools::pos!(Scalar; +y => D);

type Scalar = i32;

pub fn part1(input: &str) -> usize {
    let claims = parse_input(input);
    let mut processed = vec![];
    let mut overlaps = vec![];
    for claim in claims {
        for c in processed.iter() {
            if let Some(area) = claim.overlap(c) {
                overlaps.push(area);
            }
        }
        processed.push(claim);
    }
    let mut overlap_cells = HashSet::new();
    for (ul, dr) in overlaps {
        for y in ul.y..dr.y {
            for x in ul.x..dr.x {
                let pos = Pos { x, y };
                overlap_cells.insert(pos);
            }
        }
    }
    overlap_cells.len()
}

pub fn part2(input: &str) -> usize {
    let claims = parse_input(input);
    for &claim in &claims {
        if claims
            .iter()
            .filter(|c| claim.overlap(c).is_some())
            .nth(1) // Will always overlap with itself
            .is_none() {
            return claim.id;
        }
    }
    panic!("No non-overlapping claim found")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Claim {
    id: usize,
    offset: Pos,
    size: Pos,
}

impl Claim {
    pub fn overlap(&self, othr: &Self) -> Option<(Pos, Pos)> {
        let corner_ul = Pos {
            x: self.offset.x.max(othr.offset.x),
            y: self.offset.y.max(othr.offset.y),
        };
        let corner_dr = Pos {
            x: self.offset.add(self.size).x.min(othr.offset.add(othr.size).x),
            y: self.offset.add(self.size).y.min(othr.offset.add(othr.size).y),
        };
        if corner_ul.x >= corner_dr.x { None }
        else if corner_ul.y >= corner_dr.y { None }
        else { Some((corner_ul, corner_dr)) }
    }
}
fn parse_input(input: &str) -> Vec<Claim> {
    input.lines()
        .filter(|l| !l.is_empty())
        .map(|rest| {
            let (id, rest) = rest.split_once(" @ ").unwrap();
            let (offset, size) = rest.split_once(": ").unwrap();
            let id = id.trim_start_matches('#').parse().unwrap();

            let (offset_x, offset_y) = offset.split_once(',').unwrap();
            let (offset_x, offset_y) = (offset_x.parse().unwrap(), offset_y.parse().unwrap());
            let offset = Pos { x: offset_x, y: offset_y };

            let (size_x, size_y) = size.split_once('x').unwrap();
            let (size_x, size_y) = (size_x.parse().unwrap(), size_y.parse().unwrap());
            let size = Pos { x: size_x, y: size_y };

            Claim { id, offset, size }
        })
        .collect()
}
