use std::{str::FromStr, sync::LazyLock};

const SEA_MONSTER: &str = r"
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   
";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum IdPair {
    #[default]
    Zero,
    One(i64),
    Two(i64, i64),
}
impl IdPair {
    pub fn push(&mut self, v: i64) {
        match self {
            Self::Zero => *self = Self::One(v),
            Self::One(a) => *self = Self::Two(*a, v),
            Self::Two(_a, b) => *self = Self::Two(*b, v),
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = i64> {
        match self {
            Self::Zero => [None, None],
            Self::One(a) => [Some(*a), None],
            Self::Two(a, b) => [Some(*a), Some(*b)],
        }.into_iter().flatten()
    }
    pub fn len(&self) -> usize {
        match self {
            Self::Zero => 0,
            Self::One(_) => 1,
            Self::Two(_, _) => 2,
        }
    }
}

aoc_tools::aoc_sol!(day20 2020: part1, part2);
aoc_tools::map_struct!(Tile of bool { id: i64 }, pos i16);
impl Tile {
    pub fn set_at(&self, pos: Pos) -> bool {
        self.get_raw(pos) == Some(&true)
    }
    pub fn top_row(&self, transform: Transform) -> u32 {
        let mut output = 0;
        for x in 0..self.width() {
            let pos = Pos { x: x as i16, y: 0 };
            let pos = transform.untransform_pos(pos, self);
            output <<= 1;
            output |= self.set_at(pos) as u32;
        }
        output
    }
    pub fn top_row_2(&self, transform_1: Transform, transform_2: Transform) -> u32 {
        let mut output = 0;
        for x in 0..self.width() {
            let pos = Pos { x: x as i16, y: 0 };
            let pos = transform_1.untransform_pos(transform_2.untransform_pos(pos, self), self);
            output <<= 1;
            output |= self.set_at(pos) as u32;
        }
        output
    }
    pub fn all_edges(&self) -> [u32; 8] {
        Transform::all_transforms().map(|transform| self.top_row(transform))
    }
    #[inline(never)]
    pub fn transformed(&self, transform: Transform) -> Self {
        let mut transformed = self.clone();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let old_pos = Pos { x: x as i16, y: y as i16 };
                let new_pos = transform.transform_pos(old_pos, self);
                *transformed.get_mut_raw(new_pos).unwrap() = self.set_at(old_pos);
            }
        }
        transformed
    }
    pub fn monster_at(&self, pos: Pos) -> bool {
        static MONSTER: LazyLock<Vec<Vec<bool>>> = LazyLock::new(|| {
            aoc_tools::parse_map(SEA_MONSTER.trim_matches('\n'), |v| v == '#')
        });
        if pos.x as usize + MONSTER[0].len() > self.width() { return false }
        if pos.y as usize + MONSTER.len() > self.height() { return false }
        for x in 0..MONSTER[0].len() {
            for y in (0..MONSTER.len()).rev() {
                if !MONSTER[y][x] { continue }
                let y = pos.y + y as i16;
                let x = pos.x + x as i16;
                let test_pos = Pos { x, y };
                if !self.set_at(test_pos) {
                    return false;
                }
            }
        }
        true
    }
    pub fn count_monsters(&self, ) -> usize {
        let mut count = 0;
        for x in 0..self.width() {
            for y in 0..self.height() {
                let pos = Pos { x: x as i16, y: y as i16 };
                if self.monster_at(pos) {
                    count += 1;
                }
            }
        }
        count
    }
    pub fn roughness(&self) -> usize {
        let monsters = self.count_monsters();
        let per_monster = SEA_MONSTER.chars().filter(|&c| c == '#').count();
        let all_tiles = self.count_matching(|c| *c);
        all_tiles - monsters * per_monster
    }
    #[inline(never)]
    pub fn min_transformed_roughness(&self) -> usize {
        let mut min = usize::MAX;
        for transform in Transform::all_transforms() {
            let roughness = self.transformed(transform).roughness();
            if roughness < min {
                if min != usize::MAX {
                    return roughness;
                }
                min = roughness;
            }
            // min = min.min(roughness);
        }
        min
    }
}
impl FromStr for Tile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix("Tile ") else {
            return Err("Missing 'Tile'".to_string())
        };
        let Some((id, rows)) = s.split_once(":\n") else {
            return Err("Missing colon after tile ID".to_string());
        };
        let id = id.parse::<i64>().map_err(|e| e.to_string())?;
        Ok(Tile {
            id,
            rows: aoc_tools::parse_map(rows, |c| c == '#'),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Transform {
    cw_rotations: u8,
    flipped: bool,
}
impl Transform {
    #[allow(dead_code)]
    const IDENTITY: Self = Self { cw_rotations: 0, flipped: false };
    #[allow(dead_code)]
    const ROT_90: Self = Self { cw_rotations: 1, flipped: false };
    #[allow(dead_code)]
    const ROT_180: Self = Self { cw_rotations: 2, flipped: false };
    #[allow(dead_code)]
    const ROT_270: Self = Self { cw_rotations: 3, flipped: false };
    #[allow(dead_code)]
    const FLIP: Self = Self { cw_rotations: 0, flipped: true };
    #[allow(dead_code)]
    const FLIP_270: Self = Self { cw_rotations: 3, flipped: true };

    pub fn all_transforms() -> [Self; 8] {
        std::array::from_fn(|i| {
            Transform {
                cw_rotations: i as u8 / 2,
                flipped: (i & 0b1) == 1,
            }
        })
    }
    pub fn transform_pos(&self, pos: Pos, tile: &Tile) -> Pos {
        let mut pos = if self.flipped {
            Pos { x: pos.x, y: tile.height() as i16 - pos.y - 1 }
        } else {
            Pos { x: pos.x, y: pos.y }
        };
        for _ in 0..self.cw_rotations {
            pos = Pos {
                x: tile.width() as i16 - pos.y - 1,
                y: pos.x,
            };
        }
        pos
    }
    pub fn untransform_pos(&self, mut pos: Pos, tile: &Tile) -> Pos {
        for _ in self.cw_rotations..4 {
            pos = Pos {
                x: tile.width() as i16 - pos.y - 1,
                y: pos.x,
            };
        }
        if self.flipped {
            Pos { x: pos.x, y: tile.height() as i16 - pos.y - 1 }
        } else {
            Pos { x: pos.x, y: pos.y }
        }
    }
    pub fn rot_90_cw(&self) -> Self {
        Self {
            cw_rotations: (self.cw_rotations + 1) % 4,
            flipped: self.flipped,
        }
    }
}

fn gen_seen<'a>(tiles: impl Iterator<Item = &'a Tile>) -> HashMap<u32, IdPair> {
    let mut seen = HashMap::<u32, IdPair>::new();
    for tile in tiles {
        for edge in tile.all_edges() {
            seen.entry(edge).or_default().push(tile.id);
        }
    }
    seen
}
fn get_corners<'a>(tiles: impl Iterator<Item = &'a Tile>, seen: &HashMap<u32, IdPair>) -> Vec<i64> {
    let mut corners = vec![];
    for tile in tiles {
        let mut unmatched = 0;
        for edge in tile.all_edges() {
            if seen.get(&edge).unwrap().len() < 2 {
                unmatched += 1;
            }
        }
        if unmatched >= 4 {
            corners.push(tile.id);
        }
    }
    corners
}

fn as_ul_corner<'a>(id: i64, tiles: &'a HashMap<i64, Tile>, seen: &HashMap<u32, IdPair>) -> (&'a Tile, Transform) {
    let row_start_tile = tiles.get(&id).unwrap();

    // Rotate until the bottom and the right are connected
    let mut transform = Transform { cw_rotations: 0, flipped: false };
    loop {
        let right = row_start_tile.top_row_2(transform, Transform::ROT_270);
        let bottom = row_start_tile.top_row_2(transform, Transform::ROT_180);
        if seen.get(&right).unwrap().len() >= 2 && seen.get(&bottom).unwrap().len() >= 2 { break }
        transform.cw_rotations += 1;
    }
    (row_start_tile, transform)
}

pub fn part1(input: &str) -> i64 {
    let tiles = parse_input(input);
    let seen = gen_seen(tiles.values());
    let corners = get_corners(tiles.values(), &seen);
    corners.into_iter().product()
}

fn extend_new_rows(new_rows: &mut [Vec<bool>], tile: &Tile, transform: Transform) {
    for y in 1..tile.height()-1 {
        for x in 1..tile.width()-1 {
            let pos = Pos { x: x as i16, y: y as i16 };
            new_rows[y-1].push(tile.set_at(transform.untransform_pos(pos, tile)));
        }
    }
}

fn get_right_connected<'a>(
    right: u32,
    curr_id: i64,
    tiles: &'a HashMap<i64, Tile>,
    seen: &HashMap<u32, IdPair>,
) -> Option<(&'a Tile, Transform)> {
    let matching_ids = seen.get(&right).unwrap();
    let next_id = matching_ids.iter().find(|&id| id != curr_id)?;
    let next_tile = tiles.get(&next_id).unwrap();
    let transform = Transform::all_transforms()
        .into_iter()
        .find(|transform| next_tile.top_row(transform.rot_90_cw()) == right)
        .unwrap();
    Some((next_tile, transform))
}
fn get_bottom_connected<'a>(
    bottom: u32,
    curr_id: i64,
    tiles: &'a HashMap<i64, Tile>,
    seen: &HashMap<u32, IdPair>,
) -> Option<(&'a Tile, Transform)> {
    let matching_ids = seen.get(&bottom).unwrap();
    let next_id = matching_ids.iter().find(|&id| id != curr_id)?;
    let next_tile = tiles.get(&next_id).unwrap();
    let transform = Transform::all_transforms()
        .into_iter()
        .find(|transform| next_tile.top_row(*transform) == bottom)
        .unwrap();
    Some((next_tile, transform))
}

pub fn part2(input: &str) -> usize {
    let tiles = parse_input(input);
    let seen = gen_seen(tiles.values());
    let corners = get_corners(tiles.values(), &seen);

    let trimmed_tile_dim = tiles.values().next().unwrap().width()-2;
    let image_dim = tiles.len().isqrt() * trimmed_tile_dim;

    let mut image = vec![];
    let mut row_start_tile_id = 0;

    let (row_start_tile, transform) = as_ul_corner(corners[0], &tiles, &seen);
    let mut bottom = row_start_tile.top_row(transform);
    let mut right: u32;
    while let Some((next_tile, transform)) = get_bottom_connected(bottom, row_start_tile_id, &tiles, &seen) {
        bottom = next_tile.top_row_2(transform, Transform::FLIP);
        right = next_tile.top_row_2(transform, Transform::FLIP_270);
        row_start_tile_id = next_tile.id;

        image.extend(std::iter::repeat_with(|| Vec::with_capacity(image_dim)).take(trimmed_tile_dim));
        let mut new_rows = {
            let start = image.len() - trimmed_tile_dim;
            &mut image[start..]
        };
        extend_new_rows(&mut new_rows, &next_tile, transform);

        let mut curr_id = row_start_tile_id;
        while let Some((next_tile, transform)) = get_right_connected(right, curr_id, &tiles, &seen) {
            right = next_tile.top_row_2(transform, Transform::FLIP_270);
            curr_id = next_tile.id;
            extend_new_rows(&mut new_rows, &next_tile, transform);
        }
    }
    Tile { id: 0, rows: image }.min_transformed_roughness()
}

fn parse_input(input: &str) -> HashMap<i64, Tile> {
    input.trim()
        .split("\n\n")
        .map(|t| t.parse::<Tile>().unwrap())
        .map(|t| (t.id, t))
        .collect()
}
