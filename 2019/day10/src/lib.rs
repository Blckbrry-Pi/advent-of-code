use std::sync::OnceLock;

aoc_tools::aoc_sol!(day10 2019: part1, part2);
aoc_tools::map_struct!(Map of bool { directions: OnceLock<Vec<Pos>> }, pos Scalar; +y => D);

type Scalar = i16;

fn gcd(a: Scalar, b: Scalar) -> Scalar {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Map {
    pub fn sightline(&self, from: Pos, direction: Pos) -> Option<Pos> {
        let mut curr = from.add(direction);
        while let Some(&asteroid_at) = self.get_raw(curr) {
            if asteroid_at { return Some(curr); }
            curr = curr.add(direction);
        }
        None
    }
    pub fn direction_iter(&self) -> impl Iterator<Item = Pos> + Clone {
        fn direction_iter_inner(w: i16, h: i16) -> Vec<Pos> {
            fn pos_angle_value(mut pos: Pos, mult: u64) -> u64 {
                let mut offset = 0;
                while pos.x < 0 || pos.y >= 0 {
                    pos = pos.turn_l();
                    offset += mult * mult;
                }
                offset + pos.x as u64 * mult / -pos.y as u64
            }
            let mut directions: Vec<_> = (-h..=h)
                .flat_map(move |y| (-w..=w).map(move |x| Pos { x, y }))
                .filter(|pos| gcd(pos.x.abs(), pos.y.abs()) == 1)
                .collect();
            directions.sort_by_key(|d| pos_angle_value(*d, w as u64 * h as u64));
            directions
        }

        self.directions
            .get_or_init(|| direction_iter_inner(self.width() as i16, self.height() as i16))
            .clone()
            .into_iter()
    }
    pub fn count_at(&self, station_at: Pos) -> usize {
        let mut count = 0;
        for direction in self.direction_iter() {
            if self.sightline(station_at, direction).is_some() {
                count += 1;
            }
        }
        count
    }
    pub fn best_station_pos(&self) -> Pos {
        let mut best_so_far = (0, Pos::ZERO);
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = Pos { x: x as i16, y: y as i16 };
                if self.get_raw(pos) != Some(&true) { continue }

                let new_score = self.count_at(pos);
                if new_score > best_so_far.0 {
                    best_so_far = (new_score, pos);
                }
            }
        }
        best_so_far.1
    }
    pub fn do_laser(&mut self, from: Pos) -> Pos {
        let mut i = 0;

        let directions = self.direction_iter();
        let mut directions = std::iter::repeat(()).flat_map(|_| directions.clone());
        loop {
            let direction = directions.next().unwrap();
            if let Some(to_destroy) = self.sightline(from, direction) {
                *self.get_mut_raw(to_destroy).unwrap() = false;
                i += 1;
                if i == 200 { return to_destroy }
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let map = parse_input(input);
    map.count_at(map.best_station_pos())
}

pub fn part2(input: &str) -> i16 {
    let mut map = parse_input(input);
    let pos = map.do_laser(map.best_station_pos());
    pos.x * 100 + pos.y
}

fn parse_input(input: &str) -> Map {
    Map {
        rows: aoc_tools::parse_map(input, |c| c == '#'),
        directions: OnceLock::new(),
    }
}
