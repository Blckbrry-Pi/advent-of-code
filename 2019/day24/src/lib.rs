type Scalar = i16;
aoc_tools::aoc_sol!(day24 2019: part1, part2);
aoc_tools::map_struct!(Map of bool, pos i16; +y=>D);

impl Map {
    pub fn empty_5x5() -> Self {
        Self { rows: vec![vec![false; 5]; 5] }
    }
    pub fn biodiversity(&self) -> u64 {
        let mut curr = 0;
        for y in (0..self.height() as Scalar).rev() {
            for x in (0..self.width() as Scalar).rev() {
                let pos = Pos { x, y };
                curr <<= 1;
                if self.get_raw(pos) == Some(&true) {
                    curr |= 1;
                }
            }
        }
        curr
    }
    pub fn step(&self) -> Self {
        let mut new = self.clone();
        for y in 0..self.height() as Scalar {
            for x in 0..self.width() as Scalar {
                let pos = Pos { x, y };
                let is_alive = self.get_raw(pos) == Some(&true);
                let mut surrounding_count = 0;
                for offset in [Pos::U, Pos::D, Pos::L, Pos::R] {
                    let neighbor = pos.add(offset);
                    if self.get_raw(neighbor) == Some(&true) {
                        surrounding_count += 1;
                    }
                }
                if is_alive && surrounding_count != 1 {
                    *new.get_mut_raw(pos).unwrap() = false;
                } else if !is_alive && (surrounding_count == 1 || surrounding_count == 2) {
                    *new.get_mut_raw(pos).unwrap() = true;
                }
            }
        }
        new
    }
    pub fn step_p2(&self, outside: &Map, inside: &Map) -> Self {
        let mut new = self.clone();
        for y in 0..self.height() as Scalar {
            for x in 0..self.width() as Scalar {
                let pos = Pos { x, y };
                if pos == (Pos { x: 2, y: 2 }) { continue }

                let is_alive = self.get_raw(pos) == Some(&true);
                let mut surrounding_count = 0;
                for offset in [Pos::U, Pos::D, Pos::L, Pos::R] {
                    let neighbor = pos.add(offset);
                    let value = if neighbor == (Pos { x: 2, y: 2 }) {
                        surrounding_count += if offset == Pos::U {
                            (0..5)
                                .map(|x| Pos { x, y: 4 })
                                .filter(|&v| inside.get_raw(v) == Some(&true))
                                .count()
                        } else if offset == Pos::D {
                            (0..5)
                                .map(|x| Pos { x, y: 0 })
                                .filter(|&v| inside.get_raw(v) == Some(&true))
                                .count()
                        } else if offset == Pos::L {
                            (0..5)
                                .map(|y| Pos { x: 4, y })
                                .filter(|&v| inside.get_raw(v) == Some(&true))
                                .count()
                        } else if offset == Pos::R {
                            (0..5)
                                .map(|y| Pos { x: 0, y })
                                .filter(|&v| inside.get_raw(v) == Some(&true))
                                .count()
                        } else { unreachable!("A") };
                        continue
                    } else if neighbor.x < 0 {
                        outside.get_raw(Pos { x: 1, y: 2 })
                    } else if neighbor.x >= 5 {
                        outside.get_raw(Pos { x: 3, y: 2 })
                    } else if neighbor.y < 0 {
                        outside.get_raw(Pos { x: 2, y: 1 })
                    } else if neighbor.y >= 5 {
                        outside.get_raw(Pos { x: 2, y: 3 })
                    } else {
                        self.get_raw(neighbor)
                    };
                    if value == Some(&true) {
                        surrounding_count += 1;
                    }
                }
                if is_alive && surrounding_count != 1 {
                    *new.get_mut_raw(pos).unwrap() = false;
                } else if !is_alive && (surrounding_count == 1 || surrounding_count == 2) {
                    *new.get_mut_raw(pos).unwrap() = true;
                }
            }
        }
        new
    }
}

struct BigMap {
    layers: Vec<Map>,
}
impl BigMap {
    pub fn new_with_layers(base_map: Map, layers_per_side: usize) -> Self {
        let before = std::iter::repeat_with(Map::empty_5x5).take(layers_per_side);
        let after = std::iter::repeat_with(Map::empty_5x5).take(layers_per_side);
        Self { layers: before.chain([base_map]).chain(after).collect()}
    }
    pub fn step(&self) -> Self {
        let extra_bounds = Map::empty_5x5();
        let mut new = Self { layers: Vec::with_capacity(self.layers.len()) };

        for i in 0..self.layers.len() {
            let outside = self.layers.get(i.wrapping_sub(1)).unwrap_or(&extra_bounds);
            let inside = self.layers.get(i.wrapping_add(1)).unwrap_or(&extra_bounds);
            new.layers.push(self.layers[i].step_p2(outside, inside));
        }
        new
    }
    pub fn count(&self) -> usize {
        self.layers
            .iter()
            .map(|m| m.count_matching(|n| *n))
            .sum()
    }
}

pub fn part1(input: &str) -> u64 {
    let mut map = parse_input(input);
    let mut seen = HashSet::new();
    while seen.insert(map.biodiversity()) {
        map = map.step();
    }
    map.biodiversity()
}

pub fn part2(input: &str) -> usize {
    let map = parse_input(input);
    let mut big_map = BigMap::new_with_layers(map, 100);
    for _ in 0..200 {
        big_map = big_map.step();
    }
    big_map.count()
}

fn parse_input(input: &str) -> Map {
    Map { rows: aoc_tools::parse_map(input, |v| v == '#') }
}
