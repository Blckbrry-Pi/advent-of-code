use std::str::FromStr;
aoc_tools::aoc_sol!(day23 2021: part1, part2);

type Scalar = i16;
aoc_tools::pos!(Scalar; +y => D);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum AmphipodType {
    Amber = 1,
    Bronze = 10,
    Copper = 100,
    Desert = 1000,
}
impl AmphipodType {
    pub fn get_target_x_pos(&self) -> Scalar {
        match self {
            Self::Amber  => 3,
            Self::Bronze => 5,
            Self::Copper => 7,
            Self::Desert => 9,
        }
    }
    pub fn from_target_x_pos(x: Scalar) -> Option<Self> {
        match x {
            3 => Some(Self::Amber),
            5 => Some(Self::Bronze),
            7 => Some(Self::Copper),
            9 => Some(Self::Desert),
            _ => None,
        }
    }
}
impl Debug for AmphipodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Amber  => write!(f, "A"),
            Self::Bronze => write!(f, "B"),
            Self::Copper => write!(f, "C"),
            Self::Desert => write!(f, "D"),
        }
    }
}
impl FromStr for AmphipodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 { return Err("Amphipod types are 1 character long".to_string()) }
        let ty = match s.as_bytes()[0] {
            b'A' => Self::Amber,
            b'B' => Self::Bronze,
            b'C' => Self::Copper,
            b'D' => Self::Desert,
            c => return Err(format!("Invalid amphipod type char: {:?}", c as char)),
        };
        Ok(ty)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Amphipod {
    ty: AmphipodType,
    pos: Pos,
    done: bool,
}
impl Amphipod {
    pub fn x_diff(&self) -> i16 {
        (self.pos.x - self.ty.get_target_x_pos()).abs()
    }
}
impl Debug for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.done {
            write!(f, "\x1b[2m")?;
        }
        write!(f, "{:?}", self.ty)?;
        write!(f, "\x1b[0m")
    }
}


#[derive(Clone)]
struct Map {
    rows: Vec<Vec<bool>>,
}
impl Map {
    pub fn get(&self, pos: Pos) -> Option<bool> {
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
    pub fn hallway_spots(&self) -> impl Iterator<Item = Pos> + '_ {
        [1, 11, 2, 10, 4, 8, 6].into_iter().rev().map(|x| Pos { x, y: 1 })
        // (0..self.rows.len())
        //     .flat_map(move |y| {
        //         (0..self.rows[y].len() / 2)
        //             .chain((self.rows[y].len() / 2..self.rows[y].len()).rev())
        //             .map(move |x| {
        //                 let pos = Pos { x: x as Scalar, y: y as Scalar };
        //                 (pos, self.get(pos))
        //             })
        //             .flat_map(|(pos, cell)| Some((pos, cell?)))
        //             .filter_map(|(pos, cell)| (cell.open && !cell.outside_room && cell.room.is_none()).then_some(pos))
        //     })
    }
    pub fn path(from: Pos, to: Pos) -> impl Iterator<Item = Pos> {
        // println!("from {from:?} to {to:?}");
        let leaving_start_room = (-from.y..-1).map(move |y| Pos { x: from.x, y: y.abs() });
        let moving_across = if from.x <= to.x {
            from.x..=to.x
        } else {
            -from.x..=-to.x
        }.map(move |x| Pos { x: x.abs(), y: 1 });
        let enter_final_room = (1..=to.y).skip(1).map(move |y| Pos { x: to.x, y });

        leaving_start_room.chain(moving_across).chain(enter_final_room)
    }
}
impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for col in row {
                write!(f, "{col:?}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    amphipods: Vec<Amphipod>,
    energy: usize,
    // positions: HashSet<Pos>,
    // prev: Option<Rc<Self>>,
}
impl State {
    pub fn start(amphipods: Vec<Amphipod>) -> Self {
        Self {
            // positions: amphipods.iter().map(|a| a.pos).collect(),
            amphipods,
            energy: 0,
            // prev: None,
        }
    }
    pub fn recursively_solve(&mut self, map: &Map, best_seen: &mut usize) {
        if self.is_done() {
            if self.energy < *best_seen {
                *best_seen = self.energy;
                println!("{}", self.energy);
            }
            return;
        }
        for idx in 0..self.amphipods.len() {
            let amphipod = self.amphipods[idx];
            if amphipod.done { continue }
            let in_hallway = amphipod.pos.y == 1;
            let room_blocked = self.room_blocked(amphipod.ty);
            let has_correct_x = amphipod.pos.x == amphipod.ty.get_target_x_pos();
            let should_stay = has_correct_x && !room_blocked;
            
            if should_stay {
                self.amphipods[idx].done = true;
                // self.move_to_end(idx);
                self.recursively_solve(map, best_seen);
                // self.move_from_end(idx);
                self.amphipods[idx].done = false;
                continue;
            } else if self.is_locked(map, amphipod.pos) {
                continue;
            }

            let mut covered_x_range = amphipod.pos.x..amphipod.pos.x;
            if !room_blocked && !has_correct_x {
                let target = self.get_next_pos_for_room(map, amphipod.ty);
                if let Some(energy) = self.reachable(amphipod, target, &mut covered_x_range) {
                    let old_pos = self.amphipods[idx].pos;
                    self.amphipods[idx].pos = target;
                    self.amphipods[idx].done = true;
                    self.energy += energy;
    
                    if self.min_total_energy(map) < *best_seen {
                        // self.move_to_end(idx);
                        self.recursively_solve(map, best_seen);
                        // self.move_from_end(idx);
                    }
    
                    self.amphipods[idx].pos = old_pos;
                    self.amphipods[idx].done = false;
                    self.energy -= energy;
                };
            }
            if !in_hallway && !should_stay {
                for target_hallway in map.hallway_spots() {
                    let Some(energy) = self.reachable(amphipod, target_hallway, &mut covered_x_range) else { continue };

                    let old_pos = self.amphipods[idx].pos;
                    self.amphipods[idx].pos = target_hallway;
                    self.energy += energy;

                    if self.min_total_energy(map) < *best_seen {
                        // self.move_to_end(idx);
                        self.recursively_solve(map, best_seen);
                        // self.move_from_end(idx);
                    }

                    self.amphipods[idx].pos = old_pos;
                    self.energy -= energy;
                }
            }
        }
    }
    pub fn reachable(&self, curr: Amphipod, to: Pos, covered_x_range: &mut std::ops::Range<i16>) -> Option<usize> {
        // let mut reachable = true;
        let mut energy = 0;
        // let mut covered_x_range = curr.pos.x..curr.pos.x;
        for passes_through in Map::path(curr.pos, to) {
            if passes_through == curr.pos { continue }
            energy += curr.ty as usize;
            if passes_through.y != 1 || !covered_x_range.contains(&passes_through.x) {
                if self.amphipods.iter().any(|a| a.pos == passes_through) {
                    return None;
                }
            }
            if passes_through.y == 1 {
                covered_x_range.start = covered_x_range.start.min(passes_through.x);
                covered_x_range.end = covered_x_range.end.max(passes_through.x+1);
            }
        }
        Some(energy)
    }

    pub fn solve(mut self, map: &Map) -> usize {
        let mut output = usize::MAX;
        self.recursively_solve(map, &mut output);
        output
    }

    pub fn move_to_end(&mut self, idx: usize) {
        let removed = self.amphipods.remove(idx);
        self.amphipods.push(removed);
    }
    pub fn move_from_end(&mut self, idx: usize) {
        let removed = self.amphipods.pop().unwrap();
        self.amphipods.insert(idx, removed);
    }

    pub fn room_blocked(&self, room: AmphipodType) -> bool {
        let mut other_occupant = false;
        for &amphipod in &self.amphipods {
            if amphipod.pos.x != room.get_target_x_pos() { continue }
            if amphipod.ty == room { continue }
            for y in 2..=5 {
                if amphipod.pos.y == y {
                    other_occupant = true;
                    break
                }
            }
            if other_occupant { break }
        }
        other_occupant
    }
    pub fn someone_is_at(&self, pos: Pos) -> bool {
        self.amphipods.iter().any(|a| a.pos == pos)
        // self.positions.contains(&pos)
    }
    pub fn is_done(&self) -> bool {
        for &amphipod in &self.amphipods {
            if amphipod.pos.x != amphipod.ty.get_target_x_pos() || (amphipod.pos.y < 2) {
                return false;
            }
        }
        true
    }
    pub fn is_locked(&self, map: &Map, pos: Pos) -> bool {
        Map::orthogonal_neighbors(pos).into_iter().all(|pos| !self.is_free(map, pos))
    }
    pub fn is_free(&self, map: &Map, pos: Pos) -> bool {
        map.get(pos).is_some_and(|c| c) && !self.someone_is_at(pos)
    }
    pub fn get_next_pos_for_room(&self, map: &Map, ty: AmphipodType) -> Pos {
        let x = ty.get_target_x_pos();
        let mut out_y = 1;
        for y in (2..=5).rev() {
            let pos = Pos { x, y };
            if map.get(pos).is_none_or(|c| !c) { continue }
            if self.someone_is_at(pos) {
                for &amphipod in &self.amphipods {
                    if amphipod.pos == pos {
                        if amphipod.ty != ty {
                            out_y = y;
                        }
                        break
                    }
                }
            } else {
                out_y = y;
            }
            if out_y != 1 {
                break;
            }
        }
        Pos {
            x,
            y: out_y,
        }
    }
    pub fn min_total_energy(&self, map: &Map) -> usize {
        let mut min = self.energy;
        for ty in [AmphipodType::Amber, AmphipodType::Bronze, AmphipodType::Copper, AmphipodType::Desert] {
            let Pos { y, .. } = self.get_next_pos_for_room(map, ty);
            let adjustment = (y - 1) as usize * y as usize / 2 * ty as usize;
            min += adjustment;
        }
        for &amphipod in &self.amphipods {
            if amphipod.x_diff() != 0 {
                let min_steps = amphipod.x_diff().abs() + amphipod.pos.y - 1;
                min += min_steps as usize * amphipod.ty as usize;
            }
        }
        min
    }
    // pub fn display_with(&self, map: &Map) {
    //     // if let Some(prev) = &self.prev {
    //     //     prev.display_with(map);
    //     //     println!("--------------");
    //     // }
    //     for y in 0..map.rows.len() {
    //         for x in 0..map.rows[y].len() {
    //             let pos = Pos { x: x as Scalar, y: y as Scalar };
    //             let mut amphipod_at = false;
    //             for (_, &amphipod) in self.amphipods.iter().enumerate() {
    //                 if amphipod.pos == pos {
    //                     if amphipod_at {
    //                         print!("\x1b[2m");
    //                     }
    //                     print!("{:?}", amphipod);
    //                     print!("\x1b[0m");
    //                     amphipod_at = true;
    //                 }
    //             }
    //             if !amphipod_at {
    //                 if map.get(pos).unwrap() {
    //                     print!(".");
    //                 } else {
    //                     print!("#")
    //                 }
    //             }
    //         }
    //         println!();
    //     }
    // }
}

pub fn part1(input: &str) -> u32 {
    let (map, amphipods) = parse_input(input);
    let state = State::start(amphipods);
    println!("{:?}", state.solve(&map));
    0
}

pub fn part2(input: &str) -> u64 {
    let (map, amphipods) = parse_input(&map_for_part_2(input));
    let state = State::start(amphipods);
    println!("{:?}", state.solve(&map));
    0
}

fn map_for_part_2(input: &str) -> String {
    let (top, bottom) = input.split_once("  ").unwrap();
    format!(
        "{top}{}\n{}\n{}{}",
        "  #D#C#B#A#",
        "  #D#B#A#C#",
        "  ",
        bottom,
    )
}

fn parse_input(input: &str) -> (Map, Vec<Amphipod>) {
    let mut rows = vec![];
    let mut amphipods = vec![];
    let lines: Vec<_> = input.lines().filter(|l| !l.trim().is_empty()).collect();
    for y in 0..lines.len() {
        rows.push(vec![]);
        for x in 0..lines[y].len() {
            let cell = match lines[y].as_bytes()[x] {
                b'.' => true,
                b'#' | b' ' => false,
                b'A' | b'B' | b'C' | b'D' => {
                    let amphipod_type = AmphipodType::from_str(&lines[y][x..x+1]).unwrap();
                    amphipods.push(Amphipod {
                        ty: amphipod_type,
                        pos: Pos { x: x as Scalar, y: y as Scalar },
                        done: false,
                    });
                    true
                },
                c => panic!("Invalid map character {:?}", c as char),
            };
            rows[y].push(cell);

        }
    }
    amphipods.sort_by(|a, b| {
        let ty_cmp = (a.ty as usize).cmp(&(b.ty as usize));
        let y_cmp = a.pos.y.cmp(&b.pos.y);
        ty_cmp.reverse().then(y_cmp)
    });
    (Map { rows }, amphipods)
}
