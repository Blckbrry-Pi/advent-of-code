aoc_tools::aoc_sol!(day03 2020: part1, part2);
aoc_tools::map_struct!(Map of bool, pos i16);
impl Map {
    pub fn get(&self, pos: Pos) -> bool {
        *self.get_raw(Pos {
            x: pos.x.rem_euclid(self.width() as i16),
            y: pos.y.rem_euclid(self.height() as i16),
        }).unwrap()
    }
    pub fn count_with_slope(&self, slope: Pos) -> u64 {
        let mut count = 0;
        let mut curr_pos = Pos::ZERO;
        while curr_pos.y < self.height() as i16 {
            if self.get(curr_pos) {
                count += 1;
            }
            curr_pos = curr_pos.add(slope);
        }
        count
    }
}

pub fn part1(input: &str) -> u64 {
    let map = parse_input(input);
    map.count_with_slope(Pos { x: 3, y: 1 })
}

pub fn part2(input: &str) -> u64 {
    let map = parse_input(input);
    const SLOPES: &[Pos] = &[
        Pos { x: 1, y: 1 },
        Pos { x: 3, y: 1 },
        Pos { x: 5, y: 1 },
        Pos { x: 7, y: 1 },
        Pos { x: 1, y: 2 },
    ];
    SLOPES.iter().map(|&s| map.count_with_slope(s)).product()
}

fn parse_input(input: &str) -> Map {
    Map {
        rows: aoc_tools::parse_map(input, |c| c == '#'),
    }
}
