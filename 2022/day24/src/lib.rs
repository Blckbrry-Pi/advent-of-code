use std::fmt::Formatter;

type Scalar = i16;

aoc_tools::aoc_sol!(day24 2022: part1, part2);
aoc_tools::pos!(Scalar; +y=>DOWN);

pub fn part1(input: &str) -> u16 {
    let mut map = parse_input(input);
    let mut finder = Pathfinder::new();
    finder.search_p1(&mut map).timestamp
}

pub fn part2(input: &str) -> u16 {
    let mut map = parse_input(input);
    let mut finder = Pathfinder::new();
    finder.search_p2(&mut map).timestamp
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct DirSet(u8);
impl DirSet {
    const N: Self = Self(0b1000);
    const S: Self = Self(0b0100);
    const E: Self = Self(0b0010);
    const W: Self = Self(0b0001);
    pub fn with(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
    pub fn iter(self) -> impl Iterator<Item = Pos> {
        (0..4).filter_map(move |i| {
            if (self.0 >> i) & 1 == 1 {
                Some([Pos::W, Pos::E, Pos::S, Pos::N][i])
            } else {
                None
            }
        })
    }
    pub fn from_dir(dir: Pos) -> Self {
        match dir.x {
            1 => Self::E,
            0 => if dir.y == 1 { Self::S } else { Self::N },
            _ => Self::W,
        }
    }
    pub fn count(self) -> u8 {
        self.0.count_ones() as u8
    }

    pub fn as_new(self) -> Self {
        Self(self.0 << 4)
    }
}

#[derive(Clone, PartialEq, Eq)]
struct BlizzardMap { data: Box<[DirSet]>, width: usize }
impl BlizzardMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![DirSet(0); width * height].into_boxed_slice(),
            width,
        }
    }
    fn index(&self, pos: Pos) -> usize {
        pos.y as usize * self.width + pos.x as usize
    }
    pub fn get(&self, pos: Pos) -> DirSet {
        let idx = self.index(pos);
        if idx >= self.data.len() {
            DirSet(0)
        } else {
            self.data[self.index(pos)]
        }
    }
    pub fn set(&mut self, pos: Pos, value: DirSet) {
        let idx = self.index(pos);
        self.data[idx] = value;
    }
    pub fn blizzard_at(&self, pos: Pos) -> Option<Result<Blizzard, u8>> {
        let set = self.get(pos);
        match set.count() {
            0 => None,
            1 => set.iter().next().map(|dir| Ok(Blizzard::new(pos, dir))),
            n => Some(Err(n)),
        }
    }
    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }
    pub fn add_blizzard(&mut self, blizzard: Blizzard) {
        let old = self.get(blizzard.pos);
        let new = old.with(DirSet::from_dir(blizzard.dir));
        self.set(blizzard.pos, new);
    }
    pub fn add_blizzard_new(&mut self, blizzard: Blizzard) {
        let old = self.get(blizzard.pos);
        let new = old.with(DirSet::from_dir(blizzard.dir).as_new());
        self.set(blizzard.pos, new);
    }
    pub fn un_new(&mut self) {
        self.data.iter_mut().for_each(|v| v.0 >>= 4);
    }
    pub fn advance(&mut self) {
        for y in 0..self.height() {
            for x in 0..self.width {
                let pos = Pos { x: x as Scalar, y: y as Scalar };
                for dir in self.get(pos).iter() {
                    let pos = pos.add(dir);
                    let pos = Pos {
                        x: pos.x.rem_euclid(self.width as i16),
                        y: pos.y.rem_euclid(self.height() as i16),
                    };
                    self.add_blizzard_new(Blizzard::new(pos, dir));
                }
            }
        }
        self.un_new();
    }
}
impl Debug for BlizzardMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Top row
        for i in 0..self.width + 2 {
            if i != 1 {
                write!(f, "#")
            } else {
                write!(f, ".")
            }?;
        }
        writeln!(f)?;

        // Main map
        for y in 0..self.height() {
            write!(f, "#")?;

            let y = y as Scalar;
            for x in 0..self.width {
                let x = x as Scalar;
                match self.blizzard_at(Pos { x, y }) {
                    Some(Ok(blizzard)) => write!(f, "{blizzard:?}"),
                    Some(Err(count)) => write!(f, "{count}"),
                    None => write!(f, "."),
                }?;
            }

            writeln!(f, "#")?;
        }

        // Bottom row
        for i in 0..self.width + 2 {
            if i != self.width {
                write!(f, "#")
            } else {
                write!(f, ".")
            }?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct SafetyMap { data: Box<[u8]>, width: usize, height: usize }
impl SafetyMap {
    pub fn from_blizzard_map(map: &BlizzardMap) -> Self {
        let mut output = SafetyMap {
            data: vec![0; map.width * map.height() / 8 + 1].into_boxed_slice(),
            width: map.width,
            height: map.height(),
        };
        for y in 0..map.height() {
            for x in 0..map.width {
                let pos = Pos { x: x as Scalar, y: y as Scalar };
                if map.get(pos) == DirSet(0) {
                    output.set(pos);
                }
            }
        }
        output
    }

    fn in_range(&self, pos: Pos) -> bool {
        let x_in_range = 0 <= pos.x && pos.x < self.width as Scalar;
        let y_in_range = 0 <= pos.y && pos.y < self.height as Scalar;
        x_in_range && y_in_range
    }
    fn index(&self, pos: Pos) -> usize {
        (pos.y as usize * self.width + pos.x as usize) >> 3
    }
    fn mask(&self, pos: Pos) -> u8 {
        let shift = (pos.y as usize * self.width + pos.x as usize) & 0b111;
        1 << shift
    }
    pub fn set(&mut self, pos: Pos) {
        let index = self.index(pos);
        let mask = self.mask(pos);
        self.data[index] |= mask;
    }
    pub fn unset(&mut self, pos: Pos) {
        let index = self.index(pos);
        let mask = self.mask(pos);
        self.data[index] &= !mask;
    }
    pub fn is_safe(&self, pos: Pos) -> bool {
        if !self.in_range(pos) { return false }
        let index = self.index(pos);
        let mask = self.mask(pos);
        self.data[index] & mask != 0
    }
}

#[derive(Clone)]
struct Map {
    blizzards: BlizzardMap,
    timestamp: u16,
}
impl Map {
    fn add_blizzard(&mut self, blizzard: Blizzard) {
        self.blizzards.add_blizzard(blizzard);
    }
    fn advance_blizzards(&mut self) {
        self.blizzards.advance();
        self.timestamp += 1;
    }
    fn exit_node(&self) -> Node {
        Node {
            pos: Pos { x: self.blizzards.width as Scalar - 1, y: self.blizzards.height() as Scalar },
            timestamp: self.timestamp,
        }
    }
}
impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.blizzards.fmt(f)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Blizzard {
    pos: Pos,
    dir: Pos,
}
impl Blizzard {
    pub fn new(pos: Pos, dir: Pos) -> Self {
        Self { pos, dir }
    }
}
impl Debug for Blizzard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.dir == Pos::N {
            write!(f, "^")
        } else if self.dir == Pos::S {
            write!(f, "v")
        } else if self.dir == Pos::E {
            write!(f, ">")
        } else {
            write!(f, "<")
        }
    }
}

const TIMESTAMP_MUL: u64 = 9949;
const X_DIFF_MUL: u64 = 9973;
const Y_DIFF_MUL: u64 = 9967;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    pos: Pos,
    timestamp: u16,
}
impl Node {
    pub fn descendents(self) -> impl Iterator<Item = Node> {
        let possibilities = [
            Pos { x: 0, y: 0 },
            Pos::N,
            Pos::S,
            Pos::E,
            Pos::W,
        ];
        possibilities.into_iter().map(move |pos_offset| {
            Self {
                pos: self.pos.add(pos_offset),
                timestamp: self.timestamp + 1,
            }
        })
    }

    pub fn start_node() -> Self {
        Self {
            pos: Pos { x: 0, y: -1 },
            timestamp: 0,
        }
    }
}
impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) @ {}", self.pos.x, self.pos.y, self.timestamp)
    }
}

#[derive(Debug)]
struct Pathfinder {
    histories: Vec<SafetyMap>,
    target_pos: Pos,
    max_timestamp: u16,
}
impl Pathfinder {
    fn new() -> Self {
        Self {
            histories: vec![],
            target_pos: Pos { x: 0, y: 0 },
            max_timestamp: 0,
        }
    }
    fn extend_one(&mut self, map: &mut Map) {
        if map.timestamp == 0 {
            self.histories.push(SafetyMap::from_blizzard_map(&map.blizzards));
        }
        map.advance_blizzards();
        self.histories.push(SafetyMap::from_blizzard_map(&map.blizzards));
        self.max_timestamp = map.timestamp;
        self.target_pos = map.exit_node().pos;
    }
    fn search_p1(&mut self, map: &mut Map) -> Node {
        self.search_from(map, Node::start_node(), map.exit_node().pos)
    }
    fn search_p2(&mut self, map: &mut Map) -> Node {
        let first = self.search_from(map, Node::start_node(), map.exit_node().pos);
        let second = self.search_from(map, first, Node::start_node().pos);
        let third = self.search_from(map, second, map.exit_node().pos);
        third
    }
    fn search_from(&mut self, map: &mut Map, start: Node, goal: Pos) -> Node {
        let mut curr: HashSet<Node> = [start].into_iter().collect();
        let mut new_curr = HashSet::new();
        while !curr.is_empty() {
            for node in curr.drain() {
                while node.timestamp >= self.max_timestamp {
                    self.extend_one(map);
                }
                for next_node in node.descendents() {
                    if next_node.pos == goal {
                        return next_node;
                    }
                    let is_start = next_node.pos == Node::start_node().pos;
                    let is_exit = next_node.pos == map.exit_node().pos;
                    let is_safe = self.histories[next_node.timestamp as usize].is_safe(next_node.pos);
                    if is_start || is_exit || is_safe {
                        new_curr.insert(next_node);
                    }
                }
            }
            std::mem::swap(&mut curr, &mut new_curr);
        }
        panic!("Unable to find path")
    }
}

fn parse_input(input: &str) -> Map {
    let mut width = 0;
    let mut height = 0;
    let blizzards: Vec<_> = input.lines()
        .map(|line| line.trim_matches('#'))
        .filter(|line| line.len() > 1)
        .enumerate()
        .flat_map(|(y, line)| {
            width = width.max(line.len());
            height = height.max(y + 1);
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as Scalar, y as Scalar), c))
                .map(|((x, y), c)| (Pos { x, y }, c))
        })
        .filter_map(|(pos, c)| match c {
            '.' => None,
            '^' => Some(Blizzard::new(pos, Pos::N)),
            'v' => Some(Blizzard::new(pos, Pos::S)),
            '>' => Some(Blizzard::new(pos, Pos::E)),
            '<' => Some(Blizzard::new(pos, Pos::W)),
            _ => panic!("This shouldn't happen")
        })
        .collect();
    let mut map = BlizzardMap::new(width, height);
    for blizzard in blizzards {
        map.add_blizzard(blizzard);
    }
    Map {
        blizzards: map,
        timestamp: 0,
    }
}
