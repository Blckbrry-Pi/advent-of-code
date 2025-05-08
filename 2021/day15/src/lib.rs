use std::{collections::BTreeMap, str::FromStr};

aoc_tools::aoc_sol!(day15 2021: part1, part2);
aoc_tools::pos!(i16; +y => D);

#[derive(Clone)]
struct Map {
    rows: Vec<Vec<u8>>,
    distance: Vec<Vec<u32>>,
}
impl Map {
    #[inline(never)]
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
    #[inline(never)]
    pub fn get_dist(&self, pos: Pos) -> Option<u32> {
        if !(0..self.distance.len()).contains(&(pos.y as usize)) {
            return None;
        }
        let row = self.distance[pos.y as usize].as_slice();
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

    #[inline(never)]
    pub fn update_distances(&mut self) {
        let mut queue = BTreeMap::new();
        let last = Pos { x: self.rows[0].len() as i16 - 1, y: self.rows.len() as i16 - 1 };
        let adjustment = move |pos| last.manhattan(pos) as u32;

        queue.insert(adjustment(Pos { x: 0, y: 0 }), vec![Pos { x: 0, y: 0 }]);
        while let Some((&v, _)) = queue.first_key_value() {
            let next = queue.get_mut(&v).unwrap().pop().unwrap();
            if queue.get(&v).unwrap().is_empty() {
                queue.remove(&v);
            }
            let v = v - adjustment(next);

            let Some(self_val) = self.get(next).map(|d| d as u32) else { continue };
            let new_val = if next != (Pos { x: 0, y: 0 }) {
                v + self_val
            } else {
                0
            };
            let prev_distance = self.get_dist(next).unwrap();
            if new_val < prev_distance {
                for neighbor in Self::orthogonal_neighbors(next) {
                    let key = adjustment(neighbor) + new_val;
                    // let key = new_val;
                    queue.entry(key).or_insert_with(|| Vec::with_capacity(1)).push(neighbor);
                }
                self.distance[next.y as usize][next.x as usize] = new_val;
            }
        }
    }
    pub fn bottom_right(&self) -> u32 {
        self.distance[self.distance.len() - 1][self.distance[0].len() - 1]
    }
    pub fn duplicate_for_part_2(&mut self) {
        let new_rows = (0..5)
            .flat_map(
                |y_rep| self.rows.iter().map(
                    move |r| (0..5).flat_map(|x_rep| r.iter().map(move |&v| (v - 1 + y_rep + x_rep) % 9 + 1)).collect::<Vec<_>>(),
                ),
            )
            .collect::<Vec<_>>();
        self.rows = new_rows;

        let val: u32 = self.rows.iter().map(|r| r.iter().map(|v| *v as u32).sum::<u32>()).sum();
        self.distance = vec![vec![val; self.rows[0].len()]; self.rows.len()];
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
        let val: u32 = rows.iter().map(|r| r.iter().map(|v| *v as u32).sum::<u32>()).sum();
        Ok(Self {
            distance: vec![vec![val; rows[0].len()]; rows.len()],
            rows,
        })
    }
}

pub fn part1(input: &str) -> u32 {
    let mut map = parse_input(input);
    map.update_distances();
    map.bottom_right()
}

pub fn part2(input: &str) -> u32 {
    let mut map = parse_input(input);
    map.duplicate_for_part_2();
    map.update_distances();
    map.bottom_right()
}

fn parse_input(input: &str) -> Map {
    input.parse().unwrap()
}
