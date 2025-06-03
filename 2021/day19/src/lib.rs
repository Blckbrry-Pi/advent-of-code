use std::str::FromStr;

aoc_tools::aoc_sol!(day19 2021: part1, part2);
aoc_tools::pos3!(i32);

impl Pos3 {
    pub const fn rotation(&self, rotation: u8) -> Self {
        let base = match rotation / 4 {
            0 => *self,
            1 => self.rot90_y(),
            2 => self.rot90_y().rot90_y(),
            3 => self.rot90_y().rot90_y().rot90_y(),
            4 => self.rot90_z(),
            5 => self.rot90_z().rot90_z().rot90_z(),
            _ => panic!("Invalid rotation index"),
        };
        match rotation & 0b11 {
            0 => base,
            1 => base.rot90_x(),
            2 => base.rot90_x().rot90_x(),
            3 => base.rot90_x().rot90_x().rot90_x(),
            _ => panic!("Mod 4"),
        }
    }
    pub const fn inverse_rotation(&self, rotation: u8) -> Self {
        let base = match rotation & 0b11 {
            0 => *self,
            1 => self.rot90_x().rot90_x().rot90_x(),
            2 => self.rot90_x().rot90_x(),
            3 => self.rot90_x(),
            _ => panic!("Mod 4"),
        };
        match rotation / 4 {
            0 => base,
            1 => base.rot90_y().rot90_y().rot90_y(),
            2 => base.rot90_y().rot90_y(),
            3 => base.rot90_y(),
            4 => base.rot90_z().rot90_z().rot90_z(),
            5 => base.rot90_z(),
            _ => panic!("Invalid rotation index"),
        }
    }
    pub const fn rot90_x(&self) -> Self {
        Self {
            x: self.x,
            y: -self.z,
            z: self.y,
        }
    }
    pub const fn rot90_y(&self) -> Self {
        Self {
            x: self.z,
            y: self.y,
            z: -self.x,
        }
    }
    pub const fn rot90_z(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
            z: self.z,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BeaconGroup([Pos3; 2]);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GroupMeta {
    offset: Pos3,
    rotation: u8,
    scanner_id: u8,
}
impl GroupMeta {
    pub const fn transform(&self, pos: Pos3) -> Pos3 {
        pos.sub(self.offset).rotation(self.rotation)
    }
    pub const fn inverse_transform(&self, pos: Pos3) -> Pos3 {
        pos.inverse_rotation(self.rotation).add(self.offset)
    }
    pub fn min_pair([a, b, c]: [Pos3; 3], rotation: u8, scanner_id: u8) -> (Self, BeaconGroup) {
        [[a, b, c], [b, c, a], [c, a, b]].into_iter()
            .map(|[a, b, c]| (GroupMeta { rotation, offset: a, scanner_id }, b, c))
            .map(|(meta, b, c)| (meta, meta.transform(b), meta.transform(c)))
            .map(|(meta, b, c)| (meta, BeaconGroup([b.min(c), b.max(c)])))
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
    }
}
#[derive(Debug, Clone)]
struct ConversionMap {
    meta_pairs: HashMap<u8, (GroupMeta, GroupMeta)>,
}
impl ConversionMap {
    pub fn get_global(&self, id: u8, pos: Pos3) -> Option<Pos3> {
        let mut curr_id = id;
        let mut curr_pos = pos;
        while curr_id != 0 {
            let Some(transform) = self.meta_pairs.get(&curr_id) else { return None };
            curr_pos = transform.1.inverse_transform(transform.0.transform(curr_pos));
            curr_id = transform.1.scanner_id;
        }
        Some(curr_pos)
    }
    pub fn from_scanners(scanners: &[Scanner]) -> Self {
        let mut map = ConversionMap { meta_pairs: HashMap::new() };
        let mut registered = HashMap::<BeaconGroup, Vec<GroupMeta>>::new();

        let mut register_triads = true;
        while map.meta_pairs.len() < scanners.len()-1 {
            for scanner in scanners {
                if map.meta_pairs.contains_key(&scanner.id) || (scanner.id == 0 && !register_triads) { continue }
                for triad in scanner.triads() {
                    let mut needs_relative = true;
                    for rotation in 0..24 {
                        let (transform, group) = GroupMeta::min_pair(triad, rotation, scanner.id);
                        if needs_relative {
                            if let Some(mappings) = registered.get(&group) {
                                for mapping in mappings {
                                    if mapping.scanner_id == scanner.id { continue }
                                    if mapping.scanner_id != 0 && !map.meta_pairs.contains_key(&mapping.scanner_id) { continue }
                                    map.meta_pairs.entry(scanner.id).or_insert((transform, *mapping));
                                    needs_relative = false;
                                    break
                                }
                            }
                        }
                        if register_triads {
                            registered.entry(group)
                                .or_insert_with(|| Vec::with_capacity(1))
                                .push(transform);
                        }
                    }
                }
            }
            register_triads = false;
        }
        map
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    id: u8,
    relative_beacons: Vec<Pos3>,
}
impl Scanner {
    pub fn triads(&self) -> impl Iterator<Item = [Pos3; 3]> + '_ {
        (0..self.relative_beacons.len()-11)
            .flat_map(|a| (a+1..self.relative_beacons.len()-10).map(move |b| (a, b)))
            .flat_map(|(a, b)| (b+1..self.relative_beacons.len()-9).map(move |c| (a, b, c)))
            .map(|(a, b, c)| [self.relative_beacons[a], self.relative_beacons[b], self.relative_beacons[c]])
            // .flat_map(|[a, b, c]| [[a, b, c], [b, c, a], [c, a, b]])
            // .map(|mut triad| { triad.sort(); triad })
    }
}
impl FromStr for Scanner {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, beacons_str) = s.split_once('\n').unwrap();
        let id = header.strip_prefix("--- scanner ").unwrap().strip_suffix(" ---").unwrap();
        let id = id.parse().unwrap();

        let mut relative_beacons = vec![];
        for beacon in beacons_str.lines() {
            if beacon.trim().is_empty() { continue }
            let (x, beacon) = beacon.split_once(',').unwrap();
            let (y, z) = beacon.split_once(',').unwrap();
            let (x, y, z) = (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap());
            relative_beacons.push(Pos3 { x, y, z });
        }

        Ok(Self {
            id,
            relative_beacons,
        })
    }
}

pub fn part1(input: &str) -> usize {
    let scanners = parse_input(input);
    let map = ConversionMap::from_scanners(&scanners);

    let mut scanner_zero_space_beacons = HashSet::new();
    for scanner in &scanners {
        for &beacon in &scanner.relative_beacons {
            scanner_zero_space_beacons.insert(map.get_global(scanner.id, beacon).unwrap());
        }
    }
    scanner_zero_space_beacons.len()
}

pub fn part2(input: &str) -> i32 {
    let scanners = parse_input(input);
    let map = ConversionMap::from_scanners(&scanners);

    let mut relatives = vec![];

    for scanner in &scanners {
        let relative = map.get_global(scanner.id, Pos3::ZERO).unwrap();
        relatives.push(relative);
    }
    relatives.iter().flat_map(|a| relatives.iter().map(|&b| a.manhattan(b)).max()).max().unwrap()
}

fn parse_input(input: &str) -> Vec<Scanner> {
    input.split("\n\n").map(Scanner::from_str).map(Result::unwrap).collect()
}
