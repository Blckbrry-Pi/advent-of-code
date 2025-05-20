aoc_tools::aoc_sol!(day17 2020: part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos4 { x: i16, y: i16, z: i16, w: i16 }
impl Pos4 {
    pub fn new(x: i16, y: i16, z: i16, w: i16) -> Self {
        Self { x, y, z, w }
    }
    pub fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

#[derive(Clone)]
struct Pocket4 {
    volumes: Vec<Vec<Vec<Vec<bool>>>>,
}
impl Pocket4 {
    pub fn from_initial(initial: Vec<Vec<bool>>) -> Self {
        Self {
            volumes: vec![vec![initial]],
        }
    }
    pub fn get(&self, pos: Pos4) -> Option<bool> {
        let Pos4 { x, y, z, w } = pos;
        let curr = &self.volumes;
        if w < 0 || w >= self.volumes.len() as i16 {
            return None;
        }
        let curr = &curr[w as usize];
        if z < 0 || z >= curr.len() as i16 {
            return None;
        }
        let curr = &curr[z as usize];
        if y < 0 || y >= curr.len() as i16 {
            return None;
        }
        let curr = &curr[y as usize];
        if x < 0 || x >= curr.len() as i16 {
            return None;
        }
        Some(curr[x as usize])
    }
    pub fn is_set(&self, pos: Pos4) -> bool {
        self.get(pos) == Some(true)
    }
    pub fn set(&mut self, pos: Pos4, value: bool) {
        let Pos4 { x, y, z, w } = pos;
        self.volumes[w as usize][z as usize][y as usize][x as usize] = value;
    }
    pub fn active_neighbors<const PART_2: bool>(&self, pos: Pos4) -> u8 {
        let mut count = 0;
        let hyperspace = &self.volumes;
        for w in -(PART_2 as i16)..=PART_2 as i16 {
            let zeros = w == 0;
            let w = w + pos.w;
            if w < 0 || w >= hyperspace.len() as i16 { continue }
            let volume = &hyperspace[w as usize];
            for z in -1..=1 {
                let zeros = zeros && z == 0;
                let z = z + pos.z;
                if z < 0 || z >= volume.len() as i16 { continue }
                let plane = &volume[z as usize];
                for y in -1..=1 {
                    let zeros = zeros && y == 0;
                    let y = y + pos.y;
                    if y < 0 || y >= plane.len() as i16 { continue }
                    let line = &plane[y as usize];
                    for x in -1..=1 {
                        let zeros = zeros && x == 0;
                        if zeros { continue }
                        let x = x + pos.x;
                        if x < 0 || x >= line.len() as i16 { continue }
                        if line[x as usize] {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
    pub fn advanced<const PART_2: bool>(&self) -> Self {
        let new_w_dim = if PART_2 { self.volumes.len()+2 } else { 1 };
        let new_z_dim = self.volumes[0].len()+2;
        let new_y_dim = self.volumes[0][0].len()+2;
        let new_x_dim = self.volumes[0][0][0].len()+2;
        let mut new = Self {
            volumes: vec![vec![vec![vec![false; new_x_dim]; new_y_dim]; new_z_dim]; new_w_dim],
        };
        let mut min = Pos4::new(100, 100, 100, 100);
        let mut max = Pos4::new(0, 0, 0, 0);
        for w in 0..new_w_dim {
            // let mut any_set = false;
            for z in 0..new_z_dim {
                for y in 0..new_y_dim {
                    for x in 0..new_x_dim {
                        let pos = Pos4::new(x as i16, y as i16, z as i16, w as i16);
                        let old_pos = pos.add(Pos4::new(-1, -1, -1, -(PART_2 as i16)));
                        let active_neighbors = self.active_neighbors::<PART_2>(old_pos);
                        if active_neighbors == 3 || (self.is_set(old_pos) && active_neighbors == 2) {
                            new.set(pos, true);
                            // any_set = true;
                            min.x = min.x.min(pos.x);
                            min.y = min.y.min(pos.y);
                            min.z = min.z.min(pos.z);
                            min.w = min.w.min(pos.w);

                            max.x = max.x.max(pos.x);
                            max.y = max.y.max(pos.y);
                            max.z = max.z.max(pos.z);
                            max.w = max.w.max(pos.w);
                        }
                    }
                }
            }
        }
        let x_span = max.x - min.x + 1;
        let y_span = max.y - min.y + 1;
        let z_span = max.z - min.z + 1;
        let w_span = max.w - min.w + 1;
        let x_matches = x_span == new_x_dim as i16;
        let y_matches = y_span == new_y_dim as i16;
        let z_matches = z_span == new_z_dim as i16;
        let w_matches = w_span == new_w_dim as i16;
        if !x_matches || !y_matches || !z_matches || !w_matches {
            new.volumes.drain(max.w as usize + 1..);
            new.volumes.drain(..min.w as usize);
            if !x_matches || !y_matches || !z_matches {
                for volume in new.volumes.iter_mut() {
                    volume.drain(max.z as usize + 1..);
                    volume.drain(..min.z as usize);
                    if !x_matches || !y_matches {
                        for plane in volume {
                            plane.drain(max.y as usize + 1..);
                            plane.drain(..min.y as usize);
                            if !x_matches {
                                for row in plane {
                                    row.drain(max.x as usize + 1..);
                                    row.drain(..min.x as usize);
                                }
                            }
                        }
                    }
                }
            }
        }
        new
    }

    pub fn count(&self) -> usize {
        self.volumes.iter().map(
            |volume| volume.iter().map(
                |plane| plane.iter().map(
                    |row| row.iter().filter(|&&v| v).count()
                ).sum::<usize>()
            ).sum::<usize>()
        ).sum()
    }
}

pub fn part1(input: &str) -> usize {
    let mut pocket = parse_input(input);
    for _ in 0..6 {
        pocket = pocket.advanced::<false>();
    }
    pocket.count()
}

pub fn part2(input: &str) -> usize {
    let mut pocket = parse_input(input);
    for _ in 0..6 {
        pocket = pocket.advanced::<true>();
    }
    pocket.count()
}

fn parse_input(input: &str) -> Pocket4 {
    Pocket4::from_initial(
        aoc_tools::parse_map(input, |c| c == '#')
    )
}
