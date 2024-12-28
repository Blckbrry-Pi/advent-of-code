aoc_tools::aoc_sol!(day18: part1, part2);
aoc_tools::pos!(isize; +y => D);

pub fn part1(input: &str) -> usize {
    // const BYTES_TO_FALL_FIRST: usize = 12;
    const BYTES_TO_FALL_FIRST: usize = 1024;

    let bytes = parse_input(input);
    let mut map = Map::new();
    for i in 0..BYTES_TO_FALL_FIRST {
        map.set(bytes[i], true);
    }

    map.path(Pos { x: 0, y: 0 }, DIMENSIONS.sub(Pos { x: 1, y: 1 })).unwrap()
}

pub fn part2(input: &str) -> String {
    let bytes = parse_input(input);

    let start_pos = Pos { x: 0, y: 0 };
    let end_pos = DIMENSIONS.add(Pos::N).add(Pos::W);

    let mut start = 0;
    let mut end = bytes.len();
    while start != end {
        let test = (start + end) / 2;
        let map = Map::n_corrupted(&bytes, test);

        if map.path(Pos { x: 0, y: 0 }, DIMENSIONS.sub(Pos { x: 1, y: 1 })).is_some() {
            start = test + 1;
        } else {
            end = test;
        }
    }

    assert!(Map::n_corrupted(&bytes, start-1).path(start_pos, end_pos).is_some());
    assert!(Map::n_corrupted(&bytes, start).path(start_pos, end_pos).is_none());

    let blocking_byte = bytes[start-1];

    format!("{},{}", blocking_byte.x, blocking_byte.y)
}

fn parse_input(input: &str) -> Vec<Pos> {
    input.lines()
        .filter(|v| !v.is_empty())
        .map(|v| {
            let (x, y) = v.split_once(',').unwrap();
            Pos { x: x.parse().unwrap(), y: y.parse().unwrap() }
        })
        .collect()
}

// const DIMENSIONS: Pos = Pos { x: 7, y: 7 };
const DIMENSIONS: Pos = Pos { x: 71, y: 71 };

#[derive(Clone, PartialEq, Eq)]
struct Map {
    corrupted: Vec<Vec<bool>>,
}
impl Map {
    pub fn new() -> Self {
        Self {
            corrupted: vec![vec![false; DIMENSIONS.x as usize]; DIMENSIONS.y as usize],
        }
    }
    pub fn get(&self, pos: Pos) -> Option<bool> {
        if !(0..DIMENSIONS.x).contains(&pos.x) { return None; }
        if !(0..DIMENSIONS.y).contains(&pos.y) { return None; }
        Some(self.corrupted[pos.y as usize][pos.x as usize])
    }
    pub fn set(&mut self, pos: Pos, b: bool) {
        self.corrupted[pos.y as usize][pos.x as usize] = b;
    }

    pub fn n_corrupted(bytes: &[Pos], n: usize) -> Self {
        let mut map = Self::new();
        for i in 0..n {
            map.set(bytes[i], true);
        }
        map
    }

    pub fn path(&self, start: Pos, end: Pos) -> Option<usize> {
        let mut curr = vec![start];
        let mut next = HashSet::new();
        let mut seen = HashSet::new();

        for i in 0.. {
            for base in curr.drain(..) {
                let adjacent = [Pos::N, Pos::E, Pos::S, Pos::W];
                for offset in adjacent {
                    let new_pos = base.add(offset);
                    if self.get(new_pos) == Some(false) && !seen.contains(&new_pos) {
                        next.insert(new_pos);
                        seen.insert(new_pos);

                        if new_pos == end {
                            return Some(i + 1);
                        }
                    }
                }
            }

            curr.extend(next.drain());
            if curr.is_empty() { break; }
            // println!("{i}: {:?} -> {end:?}", curr);
            // println!("{seen:?}");
        }

        // panic!("Cry")
        None
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..DIMENSIONS.y {
            for x in 0..DIMENSIONS.x {
                let ch = match self.get(Pos { x, y }) {
                    Some(true) => '#',
                    Some(false) => '.',
                    None => '?',
                };

                write!(f, "{ch}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
