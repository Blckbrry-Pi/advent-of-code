aoc_tools::aoc_sol!(day18 2019: part1, part2);
aoc_tools::map_struct!(Map of Tile, pos i16; +y=>D);

impl Map {
    pub fn fill_dead_ends(&mut self) {
        loop {
            let mut dead_end_filled = false;
            for y in 1..self.height() as i16 - 1 {
                for x in 1..self.width() as i16 - 1 {
                    let pos = Pos { x, y };
                    if self.get_raw(pos) != Some(&Tile::Open) { continue };
                    let mut adjacent_wall_count = 0;
                    for offset in [Pos::U, Pos::D, Pos::L, Pos::R] {
                        let neighbor = pos.add(offset);
                        if self.get_raw(neighbor) == Some(&Tile::Wall) {
                            adjacent_wall_count += 1;
                        }
                    }
                    if adjacent_wall_count >= 3 {
                        *self.get_mut_raw(pos).unwrap() = Tile::Wall;
                        dead_end_filled = true;
                    }
                }
            }
            if !dead_end_filled { break }
        }
    }
    pub fn do_replacement_for_p2(&mut self) {
        let [pos] = self.get_agents::<1>();
        *self.get_mut_raw(Pos { x: pos.x - 1, y: pos.y - 1 }).unwrap() = Tile::Entrance;
        *self.get_mut_raw(Pos { x: pos.x + 1, y: pos.y - 1 }).unwrap() = Tile::Entrance;
        *self.get_mut_raw(Pos { x: pos.x - 1, y: pos.y + 1 }).unwrap() = Tile::Entrance;
        *self.get_mut_raw(Pos { x: pos.x + 1, y: pos.y + 1 }).unwrap() = Tile::Entrance;

        *self.get_mut_raw(Pos { x: pos.x, y: pos.y - 1 }).unwrap() = Tile::Wall;
        *self.get_mut_raw(Pos { x: pos.x, y: pos.y + 1 }).unwrap() = Tile::Wall;
        *self.get_mut_raw(Pos { x: pos.x, y: pos.y }).unwrap() = Tile::Wall;
        *self.get_mut_raw(Pos { x: pos.x - 1, y: pos.y }).unwrap() = Tile::Wall;
        *self.get_mut_raw(Pos { x: pos.x + 1, y: pos.y }).unwrap() = Tile::Wall;
    }
    pub fn get_agents<const N: usize>(&self) -> [Pos; N] {
        let mut agents = [Option::<Pos>::None; N];
        let mut i = 0;
        for y in 0..self.height() as i16 {
            for x in 0..self.width() as i16 {
                let pos = Pos { x, y };
                if matches!(self.get_raw(pos), Some(Tile::Entrance)) {
                    agents[i] = Some(pos);
                    i += 1;
                }
            }
        }
        if i < N { panic!("Expected {N} agents, found {i}") }
        agents.map(|a| a.unwrap())
    }
    pub fn steps_to_solve<const N: usize>(&self) -> usize {
        let target_keyset = KeySet::with_n_keys(self.count_matching(|k| matches!(k, Tile::Key(_))));
        let mut to_explore = vec![(self.get_agents::<N>(), KeySet::empty(), None)];
        let mut seen = HashSet::<(_, _)>::new();
        let mut i = 0;
        while !to_explore.is_empty() {
            let mut new_to_explore = vec![];
            for (agents, keyset, curr_mover) in to_explore {
                if keyset == target_keyset { return i }
                for i in 0..N {
                    if curr_mover.is_some() && Some(i as u8) != curr_mover { continue }
                    let pos = agents[i];
                    for offset in [Pos::U, Pos::D, Pos::L, Pos::R] {
                        let mut new_agents = agents;
                        let new_pos = pos.add(offset);
                        new_agents[i] = new_pos;

                        let Some(tile) = self.get_raw(new_pos) else { continue };
                        if !tile.can_move_to(&keyset) { continue }
                        let mut new_keyset = keyset;
                        tile.collect_key(&mut new_keyset);

                        let curr_mover =
                            if keyset == new_keyset { curr_mover.or(Some(i as u8)) }
                            else { None };

                        let new_state = (new_agents, new_keyset, curr_mover);
                        if !seen.insert((new_agents, new_keyset)) { continue }

                        new_to_explore.push(new_state);
                    }
                }
            }
            to_explore = new_to_explore;
            i += 1;
        }
        panic!("No solution found")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Open,
    Wall,
    Entrance,
    Key(u8),
    Door(u8),
}
impl Tile {
    pub fn parse(c: char) -> Self {
        match c {
            '.' => Self::Open,
            '#' => Self::Wall,
            '@' => Self::Entrance,
            k @ ('a'..='z') => Self::Key(k as u8),
            d @ ('A'..='Z') => Self::Door(d as u8),
            other => panic!("Invalid tile {other:?}"),
        }
    }
    pub fn can_move_to(&self, have_keys: &KeySet) -> bool {
        match self {
            Self::Open | Self::Entrance | Self::Key(_) => true,
            Self::Door(d) => have_keys.has(*d as char),
            Self::Wall => false,
        }
    }
    pub fn collect_key(&self, have_keys: &mut KeySet) {
        if let Self::Key(k) = self {
            have_keys.add(*k as char);
        }
    }
}
impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "."),
            Self::Wall => write!(f, "#"),
            Self::Entrance => write!(f, "@"),
            Self::Key (k) => write!(f, "{}", *k as char),
            Self::Door(d) => write!(f, "{}", *d as char),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct KeySet(u32);
impl KeySet {
    pub fn empty() -> Self {
        Self(0)
    }
    pub fn with_n_keys(n: usize) -> Self {
        Self((1 << n) - 1)
    }
    pub fn add(&mut self, key: char) {
        let idx = key.to_ascii_lowercase() as u8 - b'a';
        self.0 |= 1 << idx;
    }
    pub fn has(&self, key: char) -> bool {
        let idx = key.to_ascii_lowercase() as u8 - b'a';
        self.0 & 1 << idx != 0
    }
}

pub fn part1(input: &str) -> usize {
    let mut map = parse_input(input);
    map.fill_dead_ends();
    map.steps_to_solve::<1>()
}

pub fn part2(input: &str) -> usize {
    let mut map = parse_input(input);
    map.do_replacement_for_p2();
    map.fill_dead_ends();
    map.steps_to_solve::<4>()
}

fn parse_input(input: &str) -> Map {
    Map { rows: aoc_tools::parse_map(input, Tile::parse) }
}
