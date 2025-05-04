use std::str::FromStr;

aoc_tools::aoc_sol!(day09 2021: part1, part2);
aoc_tools::pos!(i16; +y => D);

#[derive(Clone)]
struct Map {
    rows: Vec<Vec<u8>>,
}
impl Map {
    pub fn count(&self) -> usize { self.rows.len() * self.rows[0].len() }
    pub fn get(&self, pos: Pos) -> Option<u8> {
        if !(0..self.rows.len()).contains(&(pos.y as usize)) {
            return None;
        }
        let row = self.rows[pos.y as usize].as_slice();
        if !(0..row.len()).contains(&(pos.x as usize)) {
            return None;
        }
        Some(row[pos.x as usize])
    }
    pub fn orthogonal_neighbors(pos: Pos) -> [Pos; 4] {
        [
            pos.add(Pos::N),
            pos.add(Pos::S),
            pos.add(Pos::E),
            pos.add(Pos::W),
        ]
    }
    pub fn pos_iter(&self) -> impl Iterator<Item = Pos> {
        let width = self.rows[0].len();
        let height = self.rows.len();
        (0..height).flat_map(move |y| (0..width).map(move |x| {
            Pos { x: x as i16, y: y as i16 }
        }))
    }
    pub fn low_points(&self) -> impl Iterator<Item = Pos> + '_ {
        self.pos_iter()
            .filter(|&pos| {
                let value = self.get(pos).unwrap();
                let mut is_low = true;
                for neighbor in Map::orthogonal_neighbors(pos) {
                    let Some(neighbor) = self.get(neighbor) else { continue };
                    if neighbor <= value {
                        is_low = false;
                    }
                }
                is_low
            })
    }
    fn find_basins(&self) -> HashMap<Pos, u32> {
        let mut seen: HashSet<Pos> = HashSet::with_capacity(self.count());
        let mut queue: Vec<Pos> = Vec::with_capacity(self.count());
        let mut map: HashMap<Pos, Pos> = HashMap::with_capacity(self.count());

        let mut low_point_count = 0;
        for low_point in self.low_points() {
            map.insert(low_point, low_point);
            queue.push(low_point);
            low_point_count += 1;
        }
        let mut counts: HashMap<Pos, u32> = HashMap::with_capacity(low_point_count);

        while let Some(next) = queue.pop() {
            // Skip on already seen, 9, or OOB
            if !seen.insert(next) { continue }
            let Some(value) = self.get(next) else { continue };
            if value == 9 { continue }

            let flows_to = 'to: {
                for neighbor in Self::orthogonal_neighbors(next) {
                    let Some(neighbor_val) = self.get(neighbor) else { continue };
                    if !map.contains_key(&neighbor) { continue }
                    if neighbor_val <= value {
                        break 'to neighbor;
                    }
                }
                // If there is nothing lower, it must be a low point, and is its
                // own basin
                next
            };
            let basin = *map.get(&flows_to).unwrap();
            map.insert(next, basin);
            *counts.entry(basin).or_default() += 1;

            for neighbor in Self::orthogonal_neighbors(next) {
                if let Some(neighbor_val) = self.get(neighbor) {
                    if neighbor_val > value {
                        queue.push(neighbor);
                    }
                }
            }
        }

        counts
    }
}
impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for col in row {
                write!(f, "{col}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl FromStr for Map {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<u8>> = s.trim()
            .lines()
            .map(
                |line| line.chars().map(|cell| {
                    cell as u8 - b'0'
                }).collect(),
            )
            .collect();
        Ok(Self { rows })
    }
}

pub fn part1(input: &str) -> u32 {
    let map = parse_input(input);
    map.low_points().map(|pos| map.get(pos).unwrap() as u32 + 1).sum()
}

pub fn part2(input: &str) -> u32 {
    let map = parse_input(input);
    let basins = map.find_basins();
    let (a, b, c) = basins.into_values().fold(
        (0, 0, 0),
        |(a, b, c), new| {
            if new > b {
                if new > a {
                    (new, a, b)
                } else {
                    (a, new, b)
                }
            } else if new > c {
                (a, b, new)
            } else {
                (a, b, c)
            }
        }
    );
    a * b * c
}

fn parse_input(input: &str) -> Map {
    input.parse().unwrap()
}
