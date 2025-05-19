aoc_tools::aoc_sol!(day11 2020: part1, part2);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Floor,
    Empty,
    Taken
}
impl Cell {
    pub fn parse(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Taken,
            c => panic!("Invalid cell {c:?}"),
        }
    }
}
impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Floor => write!(f, "."),
            Self::Empty => write!(f, "L"),
            Self::Taken => write!(f, "#"),
        }
    }
}

aoc_tools::map_struct!(Ferry of Cell, pos i16);
impl Ferry {
    pub fn step(&mut self, part_2: bool) -> usize {
        let mut changes = 0;
        let mut output = self.clone();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = Pos { x: x as i16, y: y as i16 };
                if *output.get_raw(pos).unwrap() == Cell::Floor { continue }

                let occupied_neighbors = if part_2 {
                    self.occupied_sightlines(pos)
                } else {
                    self.occupied_neighbors(pos)
                };
                if *output.get_raw(pos).unwrap() == Cell::Empty {
                    if occupied_neighbors == 0 {
                        changes += 1;
                        *output.get_mut_raw(pos).unwrap() = Cell::Taken;
                    }
                } else if occupied_neighbors >= if part_2 { 5 } else { 4 } {
                    changes += 1;
                    *output.get_mut_raw(pos).unwrap() = Cell::Empty;
                }
            }
        }
        *self = output;
        changes
    }
    pub fn occupied_neighbors(&self, pos: Pos) -> usize {
        let mut output = 0;
        for pos in [
            pos.add(Pos::N).add(Pos::W),
            pos.add(Pos::N),
            pos.add(Pos::N).add(Pos::E),

            pos.add(Pos::W),
            pos.add(Pos::E),

            pos.add(Pos::S).add(Pos::W),
            pos.add(Pos::S),
            pos.add(Pos::S).add(Pos::E),
        ] {
            if self.get_raw(pos) == Some(&Cell::Taken) {
                output += 1;
            }
        }
        output
    }
    pub fn occupied_sightlines(&self, pos: Pos) -> usize {
        let mut output = 0;
        for offset in [
            Pos::N.add(Pos::W),
            Pos::N,
            Pos::N.add(Pos::E),

            Pos::W,
            Pos::E,

            Pos::S.add(Pos::W),
            Pos::S,
            Pos::S.add(Pos::E),
        ] {
            let mut curr_pos = pos.add(offset);
            loop {
                match self.get_raw(curr_pos).copied() {
                    Some(Cell::Floor) => curr_pos = curr_pos.add(offset),
                    None | Some(Cell::Empty) => break,
                    Some(Cell::Taken) => {
                        output += 1;
                        break
                    },
                }
            }
        }
        output
    }
}

pub fn part1(input: &str) -> usize {
    let mut ferry = parse_input(input);
    while ferry.step(false) > 0 {}
    ferry.count_matching(|&c| c == Cell::Taken)
}

pub fn part2(input: &str) -> usize {
    let mut ferry = parse_input(input);
    // println!("{ferry:?}");
    while ferry.step(true) > 0 {
        // println!("{ferry:?}");
    }
    ferry.count_matching(|&c| c == Cell::Taken)
}

fn parse_input(input: &str) -> Ferry {
    Ferry {
        rows: aoc_tools::parse_map(input, Cell::parse),
    }
}
