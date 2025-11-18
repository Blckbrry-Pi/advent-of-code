aoc_tools::aoc_sol!(day17 2018: part1, part2);
aoc_tools::map_struct!(Map of Cell, pos Scalar; +y => D);
type Scalar = i32;


impl Map {
    pub fn source_loc(&self) -> Pos {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = Pos { x: x as Scalar, y: y as Scalar };
                if matches!(self.get_raw(pos), Some(Cell::Sorc)) {
                    return pos;
                }
            }
        }
        panic!("No source found")
    }
    pub fn begin_pour(&mut self, pos: Pos) -> bool {
        *self.get_mut_raw(pos).unwrap() = Cell::Flow;

        let below = pos.add(Pos::S);
        match self.get_raw(below) {
            None | Some(Cell::Flow) => return false,
            Some(Cell::Sand) => if !self.begin_pour(below) {
                return false;
            }
            _ => (),
        }
        let l = pos.add(Pos::W);
        let r = pos.add(Pos::E);
        let l_cell = *self.get_raw(l).expect("edge cells should not be flowing sideways");
        let r_cell = *self.get_raw(r).expect("edge cells should not be flowing sideways");
        let l_will_hold = match l_cell {
            Cell::Sorc | Cell::Clay | Cell::Rest => Some(true),
            Cell::Flow => None,
            Cell::Sand => Some(self.begin_pour(l)),
        };
        let r_will_hold = match r_cell {
            Cell::Sorc | Cell::Clay | Cell::Rest => Some(true),
            Cell::Flow => None,
            Cell::Sand => Some(self.begin_pour(r)),
        };

        if matches!((l_will_hold, r_will_hold), (Some(true), Some(true))) {
            let mut pos_to_update = pos;
            while matches!(self.get_raw(pos_to_update), Some(Cell::Flow)) {
                *self.get_mut_raw(pos_to_update).unwrap() = Cell::Rest;
                pos_to_update = pos_to_update.add(Pos::W);
            }
            let mut pos_to_update = pos.add(Pos::E);
            while matches!(self.get_raw(pos_to_update), Some(Cell::Flow)) {
                *self.get_mut_raw(pos_to_update).unwrap() = Cell::Rest;
                pos_to_update = pos_to_update.add(Pos::E);
            }
        }
        !matches!(l_will_hold, Some(false)) && !matches!(r_will_hold, Some(false))
    }

    pub fn wet_tiles(&self) -> usize {
        self.rows
            .iter()
            .map(|r| r.iter().filter(|c| matches!(c, Cell::Flow | Cell::Rest)).count())
            .sum()
    }
    pub fn resting_tiles(&self) -> usize {
        self.rows
            .iter()
            .map(|r| r.iter().filter(|c| matches!(c, Cell::Rest)).count())
            .sum()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Sand,
    Clay,
    Flow,
    Rest,
    Sorc,
}
impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sand => write!(f, "."),
            Self::Clay => write!(f, "#"),
            Self::Flow => write!(f, "|"),
            Self::Rest => write!(f, "~"),
            Self::Sorc => write!(f, "+"),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut map = parse_input(input);
    let source = map.source_loc();
    map.begin_pour(source.add(Pos::S));
    map.wet_tiles()
}

pub fn part2(input: &str) -> usize {
    let mut map = parse_input(input);
    let source = map.source_loc();
    map.begin_pour(source.add(Pos::S));
    map.resting_tiles()
}

fn parse_input(input: &str) -> Map {
    let positions: Vec<_> = input.lines()
        .filter(|l| !l.trim().is_empty())
        .flat_map(|l| {
            let (a, b) = l.split_once(", ").unwrap();
            let (x, y) = if a.starts_with('x') {
                (a, b)
            } else {
                (b, a)
            };
            let x = x.strip_prefix("x=").unwrap();
            let y = y.strip_prefix("y=").unwrap();
            let (x_min, x_max) = x.split_once("..").unwrap_or((x, x));
            let (y_min, y_max) = y.split_once("..").unwrap_or((y, y));

            let x_min = x_min.parse().unwrap();
            let x_max = x_max.parse().unwrap();
            let y_min = y_min.parse().unwrap();
            let y_max = y_max.parse().unwrap();

            (y_min..=y_max).flat_map(move |y| (x_min..=x_max).map(move |x| Pos { x, y }))
        })
        .collect();
    let (min, max) = positions.iter().fold(
        (Pos { x: i32::MAX, y: i32::MAX }, Pos { x: 0, y: 0 }),
        |(min, max), new_pos| (
            Pos { x: min.x.min(new_pos.x), y: min.y.min(new_pos.y) },
            Pos { x: max.x.max(new_pos.x), y: max.y.max(new_pos.y) },
        ),
    );
    let w = (max.x + 1) - (min.x - 1) + 1;
    let h = max.y - (min.y - 1) + 1;
    let mut map = Map { rows: vec![vec![Cell::Sand; w as usize]; h as usize] };
    for pos in positions {
        let pos = pos.sub(min);
        map.rows[pos.y as usize + 1][pos.x as usize + 1] = Cell::Clay;
    }
    map.rows[0][500 - min.x as usize + 1] = Cell::Sorc;
    map
}
