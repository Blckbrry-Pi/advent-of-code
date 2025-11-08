aoc_tools::aoc_sol!(day15 2018: part1, part2);
aoc_tools::map_struct!(Map of Cell {
    g_range: HashSet<Pos>,
    e_range: HashSet<Pos>,
    units: HashSet<Pos>,
    rounds_completed: u64,
    elf_attack_power: u8,
}, pos Scalar; +y => D);
type Scalar = i8;

fn cmp_sol((step_a, final_a): (Pos, Pos), (step_b, final_b): (Pos, Pos)) -> std::cmp::Ordering {
    final_a.y.cmp(&final_b.y)
        .then(final_a.x.cmp(&final_b.x))
        .then(step_a.y.cmp(&step_b.y))
        .then(step_a.x.cmp(&step_b.x))
}

impl Map {
    pub fn update_metadata(&mut self) -> Vec<Pos> {
        self.g_range.clear();
        self.e_range.clear();
        self.units.clear();
        let mut units = vec![];

        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = Pos { x: x as Scalar, y: y as Scalar };
                let cell = *self.get_raw(pos).unwrap();
                if !matches!(cell, Cell::Elf(_) | Cell::Goblin(_)) { continue }

                units.push(pos);
                self.units.insert(pos);

                let is_goblin = matches!(cell, Cell::Goblin(_));

                for cell_offset in [Pos::N, Pos::E, Pos::S, Pos::W] {
                    let adjacent_pos = pos.add(cell_offset);
                    let Some(adjacent_cell) = self.get_raw(adjacent_pos) else { continue };
                    if matches!(adjacent_cell, Cell::Wall) { continue }
                    if is_goblin {
                        self.g_range.insert(adjacent_pos);
                    } else {
                        self.e_range.insert(adjacent_pos);
                    }
                }
            }
        }

        units
    }

    pub fn in_range(&self, pos: Pos, goblin: bool) -> bool {
        if goblin {
            self.g_range.contains(&pos)
        } else {
            self.e_range.contains(&pos)
        }
    }

    pub fn step_for(&self, from: Pos, goblin: bool) -> Option<Pos> {
        let mut next = vec![(from, from)];
        let mut new_next = Vec::new();

        let mut seen: HashSet<_> = next.iter().copied().collect();

        let mut distance = 0;
        let mut solution = None;
        while solution.is_none() && !next.is_empty() {
            for (mut initial_step, curr) in next.drain(..) {
                if self.in_range(curr, goblin) {
                    if let Some((best_initial, best_target)) = &mut solution {
                        if cmp_sol((initial_step, curr), (*best_initial, *best_target)).is_lt() {
                            *best_initial = initial_step;
                            *best_target = curr;
                        }
                    } else {
                        solution = Some((initial_step, curr));
                    }
                    continue;
                }
                for offset in [Pos::N, Pos::W, Pos::E, Pos::S] {
                    let curr_neighbor = curr.add(offset);
                    if !matches!(self.get_raw(curr_neighbor), Some(Cell::Empty)) { continue }

                    if distance == 0 { initial_step = curr_neighbor; }

                    if !seen.insert((initial_step, curr_neighbor)) { continue }
                    new_next.push((initial_step, curr_neighbor));
                }
            }
            std::mem::swap(&mut next, &mut new_next);
            distance += 1;
        }
        solution.map(|(step, _)| step)
    }

    pub fn do_move(&mut self, pos: Pos) -> (Pos, bool) {
        let goblin = matches!(self.get_raw(pos), Some(Cell::Goblin(_)));
        if goblin {
            if self.e_range.is_subset(&self.units) { return (pos, false); }
        } else {
            if self.g_range.is_subset(&self.units) { return (pos, false); }
        }

        let Some(step) = self.step_for(pos, !goblin) else { return (pos, false) };
        if step != pos {
            self.rows[step.y as usize][step.x as usize] = *self.get_raw(pos).unwrap();
            self.rows[pos.y as usize][pos.x as usize] = Cell::Empty;
            (step, true)
        } else {
            (pos, false)
        }
    }

    pub fn do_attack(&mut self, pos: Pos) -> Option<(Pos, Cell)> {
        let atckr_is_gob = matches!(self.get_raw(pos), Some(Cell::Goblin(_)));
        let mut best_target: Option<(u8, Pos)> = None;
        for offset in [Pos::N, Pos::W, Pos::E, Pos::S] {
            let target_pos = pos.add(offset);
            let Some(target) = self.get_raw(target_pos) else { continue };
            let ((Cell::Goblin(hp), false) | (Cell::Elf(hp), true)) = (*target, atckr_is_gob) else { continue };
            if let Some((best_hp, best_pos)) = &mut best_target {
                if hp < *best_hp {
                    *best_hp = hp;
                    *best_pos = target_pos;
                }
            } else {
                best_target = Some((hp, target_pos))
            }
        }
        if let Some((_, target_pos)) = best_target {
            let attack_damage = if atckr_is_gob { Cell::ATTACK_DAMAGE } else { self.elf_attack_power };
            let target = self.get_mut_raw(target_pos).unwrap();
            let before_attack = *target;
            if target.reduce_health(attack_damage) { return Some((target_pos, before_attack)) }
        }
        None
    }
    // #[inline(never)]
    pub fn do_round(&mut self, break_on_elf_death: bool) -> bool {
        let units = self.update_metadata();
        let mut dead_units: HashSet<Pos> = HashSet::new();

        for unit in units {
            if dead_units.contains(&unit) { continue }
            if self.combat_is_over() { return false; }

            let (new_pos, moved) = self.do_move(unit);
            let died = self.do_attack(new_pos);

            if moved || died.is_some() { self.update_metadata(); }
            // self.update_metadata();

            if let Some((died, before_death)) = died {
                dead_units.insert(died);
                if break_on_elf_death && matches!(before_death, Cell::Elf(_)) { return true }
            }
        }
        self.rounds_completed += 1;
        false
    }
    pub fn combat_is_over(&self) -> bool {
        self.e_range.is_empty() || self.g_range.is_empty()
    }
    pub fn score(&self) -> u64 {
        let mut total_health = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = Pos { x: x as Scalar, y: y as Scalar };
                let Some(&Cell::Goblin(h) | &Cell::Elf(h)) = self.get_raw(pos) else { continue };
                total_health += h as u64;
            }
        }
        total_health * self.rounds_completed
    }

    pub fn with_elf_attack_power(self, power: u8) -> Self {
        Self {
            elf_attack_power: power,
            ..self
        }
    }
    pub fn do_combat(&mut self, break_on_elf_death: bool) -> bool {
        self.update_metadata();
        while !self.combat_is_over() {
            if self.do_round(break_on_elf_death) && break_on_elf_death {
                return true
            }
        }
        false
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Goblin(u8),
    Elf(u8),
}
impl Cell {
    pub const MAX_HEALTH: u8 = 200;
    pub const ATTACK_DAMAGE: u8 = 3;

    pub fn reduce_health(&mut self, by: u8) -> bool {
        match self {
            Self::Elf(v) | Self::Goblin(v) => if by >= *v {
                *self = Self::Empty;
                true
            } else {
                *v -= by;
                false
            }
            c => panic!("Cell {c:?} cannot be attacked"),
        }
    }
}
impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const FILL_CHARS: [char; 8] = ['▁', '▂', '▃', '▄','▅','▆', '▇', '█'];
        match (self, f.alternate()) {
            (Self::Empty, false) => write!(f, " "),
            (Self::Wall , false) => write!(f, "#"),
            (Self::Empty, true ) => write!(f, "   "),
            (Self::Wall , true ) => write!(f, "###"),
            (Self::Goblin(v), _) | (Self::Elf(v), _) => {
                let char_idx = (*v as f64 / Self::MAX_HEALTH as f64 * FILL_CHARS.len() as f64) as usize;
                let chosen_char = FILL_CHARS[char_idx.clamp(0, FILL_CHARS.len() - 1)];
                let chosen_color = if matches!(self, Self::Goblin(_)) { 32 } else { 31 };
                if f.alternate() {
                    write!(f, "\x1b[{chosen_color}m{v:3}\x1b[0m")
                } else {
                    write!(f, "\x1b[{chosen_color};40m{chosen_char}\x1b[0m")
                }
            }
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut map = parse_input(input);
    map.do_combat(false);
    map.score()
}


// 54680 < a < 57400
pub fn part2(input: &str) -> u64 {
    let map = parse_input(input);
    let mut min = 4_u16;
    let mut max = 200_u16;
    while min != max {
        let test = (min + max) / 2;
        let mut test_map = map.clone().with_elf_attack_power(test as u8);
        if test_map.do_combat(true) {
            min = test + 1;
        } else {
            max = test;
        }
    }
    let mut final_map = map.with_elf_attack_power(min as u8);
    final_map.do_combat(false);
    final_map.score()
}

fn parse_input(input: &str) -> Map {
    let mut map = Map {
        rows: vec![],
        g_range: HashSet::new(),
        e_range: HashSet::new(),
        units: HashSet::new(),
        rounds_completed: 0,
        elf_attack_power: Cell::ATTACK_DAMAGE,
    };
    for line in input.lines() {
        if line.trim().is_empty() { continue }
        map.rows.push(Vec::with_capacity(line.len()));
        for c in line.chars() {
            match c {
                '.' => map.rows.last_mut().unwrap().push(Cell::Empty),
                '#' => map.rows.last_mut().unwrap().push(Cell::Wall),
                'G' => map.rows.last_mut().unwrap().push(Cell::Goblin(Cell::MAX_HEALTH)),
                'E' => map.rows.last_mut().unwrap().push(Cell::Elf(Cell::MAX_HEALTH)),
                _ => panic!("Unknown character {c}"),
            }
        }
    }
    map
}
