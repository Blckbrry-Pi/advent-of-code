use std::str::FromStr;

aoc_tools::aoc_sol!(day24 2020: part1, part2);
aoc_tools::map_struct!(Map of Color, pos i16);
impl Pos {
    pub const E: Self = Self { x: 1, y: 0 };
    pub const NE: Self = Self { x: 1, y: 1 };
    pub const NW: Self = Self { x: 0, y: 1 };
    pub const W: Self = Self { x: -1, y: 0 };
    pub const SW: Self = Self { x: -1, y: -1};
    pub const SE: Self = Self { x: 0, y: -1 };

    pub fn from_hex_str(s: &str) -> impl Iterator<Item = Pos> + '_ {
        (0..s.len())
            .filter(
                |i| s.as_bytes()
                    .get(i.wrapping_sub(1))
                    .is_none_or(|&b| b == b'e' || b == b'w'),
            )
            .map(|i| {
                match s.as_bytes()[i] {
                    b'e' | b'w' => s[i..i+1].parse().unwrap(),
                    _ => s[i..i+2].parse().unwrap(),
                }
            })
    }
}
impl FromStr for Pos {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "e"  => Ok(Self::E),
            "ne" => Ok(Self::NE),
            "nw" => Ok(Self::NW),
            "w"  => Ok(Self::W),
            "sw" => Ok(Self::SW),
            "se" => Ok(Self::SE),
            _ => Err("Invalid hex offset".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Black,
}
impl Map {
    pub fn from_setup(black_tiles: HashMap<Pos, bool>) -> Self {
        let pos_min = black_tiles.keys().fold(
            Pos { x: i16::MAX, y: i16::MAX },
            |a, b| Pos { x: a.x.min(b.x), y: a.y.min(b.y) },
        );
        let pos_max = black_tiles.keys().fold(
            Pos { x: i16::MIN, y: i16::MIN },
            |a, b| Pos { x: a.x.max(b.x), y: a.y.max(b.y) },
        );
        let width = (pos_max.x - pos_min.x + 1) as usize;
        let height = (pos_max.y - pos_min.y + 1) as usize;
        let mut output = vec![vec![Color::White; width]; height];
        for y in pos_min.y..=pos_max.y {
            for x in pos_min.x..=pos_max.x {
                let pos = Pos { x, y };
                if black_tiles.get(&pos) == Some(&true) {
                    output[(y - pos_min.y) as usize][(x - pos_min.x) as usize] = Color::Black;
                }
            }
        }
        Self { rows: output }
    }

    pub fn hex_neighbors(pos: Pos) -> [Pos; 6] {
        [
            pos.add(Pos::E),
            pos.add(Pos::NE),
            pos.add(Pos::NW),
            pos.add(Pos::W),
            pos.add(Pos::SW),
            pos.add(Pos::SE),
        ]
    }
    pub fn tile_is_black(&self, pos: Pos) -> bool {
        self.get_raw(pos) == Some(&Color::Black)
    }
    pub fn count_black_tile_neighbors(&self, pos: Pos) -> u8 {
        Self::hex_neighbors(pos).into_iter().filter(|&p| self.tile_is_black(p)).count() as u8
    }
    pub fn step(&self) -> Self {
        let mut output = Self {
            rows: vec![vec![Color::White; self.width()+2]; self.height()+2],
        };
        for y in 0..self.height()+2 {
            for x in 0..self.width()+2 {
                let new_pos = Pos { x: x as i16, y: y as i16 };
                let old_pos = Pos { x: x as i16 - 1, y: y as i16 - 1 };
                let black_tile_neighbors = self.count_black_tile_neighbors(old_pos);
                let new_color = if self.tile_is_black(old_pos) {
                    if black_tile_neighbors == 0 || black_tile_neighbors > 2 {
                        Color::White
                    } else {
                        Color::Black
                    }
                } else {
                    if black_tile_neighbors == 2 {
                        Color::Black
                    } else {
                        Color::White
                    }
                };
                *output.get_mut_raw(new_pos).unwrap() = new_color;
            }
        }
        output
    }
}
// impl Display

pub fn part1(input: &str) -> usize {
    let tile_list = parse_input(input);
    let mut black_tiles = HashMap::new();
    for tile in tile_list {
        let output = tile.into_iter().reduce(|a, b| a.add(b)).unwrap();
        *black_tiles.entry(output).or_insert(false) ^= true;
    }
    black_tiles.values().filter(|v| **v).count()
}

pub fn part2(input: &str) -> usize {
    let tile_list = parse_input(input);
    let mut black_tiles = HashMap::new();
    for tile in tile_list {
        let output = tile.into_iter().reduce(|a, b| a.add(b)).unwrap();
        *black_tiles.entry(output).or_insert(false) ^= true;
    }
    let mut map = Map::from_setup(black_tiles);
    for _ in 0..100 {
        map = map.step();
    }
    map.count_matching(|c| *c == Color::Black)
}

fn parse_input(input: &str) -> Vec<Vec<Pos>> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| Pos::from_hex_str(l).collect())
        .collect()
}
