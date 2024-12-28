aoc_tools::aoc_sol!(day20: part1, part2);
type Scalar = i16;
aoc_tools::pos!(Scalar; +y=>D);
aoc_tools::fast_hash!();

pub fn part1(input: &str) -> usize {
    const CHEAT_IMPROVEMENT_THRESHOLD: Scalar = 100;

    let map = parse_input(input);
    let base_start_lens = map.distances_from(map.start);
    let len = base_start_lens[map.end.y as usize][map.end.x as usize];
    let max_len = len - CHEAT_IMPROVEMENT_THRESHOLD;
    let cheats = map.cheat_lengths(2, max_len);

    cheats.iter().filter(|(_, new_len)| len - *new_len >= CHEAT_IMPROVEMENT_THRESHOLD).count()
}

pub fn part2(input: &str) -> usize {
    const CHEAT_IMPROVEMENT_THRESHOLD: Scalar = 100;

    let map = parse_input(input);
    let base_start_lens = map.distances_from(map.start);
    let len = base_start_lens[map.end.y as usize][map.end.x as usize];
    let max_len = len - CHEAT_IMPROVEMENT_THRESHOLD;
    let cheats = map.cheat_lengths(20, max_len);

    cheats.iter().filter(|(_, new_len)| len - *new_len >= CHEAT_IMPROVEMENT_THRESHOLD).count()
}

fn parse_input(input: &str) -> Map {
    let mut start = Pos { x: 0, y: 0 };
    let mut end = Pos { x: 0, y: 0 };
    let mut clear = vec![];
    input.lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .for_each(|(y, l)| {
            clear.push(vec![]);
            l.chars()
                .enumerate()
                .for_each(|(x, c)| {
                    let pos = Pos { x: x as Scalar, y: y as Scalar };
                    match c {
                        '#' => clear.last_mut().unwrap().push(false),
                        '.' => clear.last_mut().unwrap().push(true),
                        'S' => {
                            start = pos;
                            clear.last_mut().unwrap().push(true);
                        },
                        'E' => {
                            end = pos;
                            clear.last_mut().unwrap().push(true);
                        },
                        _ => panic!("Invalid map char")
                    }
                })
        });

    Map { clear, start, end }
}

#[derive(Clone)]
struct Map {
    clear: Vec<Vec<bool>>,
    start: Pos,
    end: Pos,
}

impl Map {
    fn distances_from(&self, pos: Pos) -> Vec<Vec<Scalar>> {
        let mut curr = vec![pos];
        let mut next = new_fastset();
        let mut seen = new_fastmap_with_capacity(self.clear.len().pow(2));
        seen.insert(pos, 0);

        for i in 1.. {
            for base in curr.drain(..) {
                let adjacent = [Pos::N, Pos::E, Pos::S, Pos::W];
                for offset in adjacent {
                    let new_pos = base.add(offset);
                    if self.get(new_pos) == Some(true) && !seen.contains_key(&new_pos) {
                        next.insert(new_pos);
                        seen.insert(new_pos, i);
                    }
                }
            }

            curr.extend(next.drain());
            if curr.is_empty() { break; }
        }

        let mut map = vec![vec![-1; self.clear[0].len()]; self.clear.len()];
        for (pos, len) in seen {
            map[pos.y as usize][pos.x as usize] = len;
        }

        map
    }

    fn cheat_lengths(&self, max_cheat_distance: Scalar, max_dist: Scalar) -> Vec<(Cheat, Scalar)> {
        let start_distances = self.distances_from(self.start);
        let end_distances = self.distances_from(self.end);

        let max_cheats = self.clear.len().pow(2) * max_cheat_distance.pow(2) as usize;
        let mut distances = Vec::with_capacity(max_cheats / 8);
        for sy in 0..self.clear.len() as Scalar {
            for sx in 0..self.clear[0].len() as Scalar {
                let start = Pos { x: sx, y: sy };

                let dist_from_start = start_distances[sy as usize][sx as usize];
                if dist_from_start == -1 { continue }
                if dist_from_start > max_dist { continue }

                let y_range = (max_dist - dist_from_start).min(max_cheat_distance);
                let min_y = 0.max(sy-y_range);
                let max_y = (sy+y_range).min(self.clear.len() as Scalar - 1);
                for ey in min_y..=max_y {
                    let y_dist = (start.y - ey).abs();
                    let x_range = y_range - y_dist;
                    let min_x = 0.max(sx-x_range);
                    let max_x = (sx+x_range).min(self.clear[0].len() as Scalar - 1);

                    for ex in min_x..=max_x {
                        let x_dist = (start.x - ex).abs();
                        let end = Pos { x: ex, y: ey };

                        let dist_to_end = end_distances[ey as usize][ex as usize];
                        if dist_to_end == -1 { continue }

                        let cheated_distance = dist_from_start + x_dist + y_dist + dist_to_end;
                        if cheated_distance > max_dist { continue; }

                        distances.push((Cheat(start, end), cheated_distance));
                    }
                }

            }
        }

        distances
    }

    fn get(&self, p: Pos) -> Option<bool> {
        if !(0..self.clear.len()).contains(&(p.y as usize)) { return None }
        if !(0..self.clear[0].len()).contains(&(p.x as usize)) { return None }
        Some(self.is_clear_at(p))
    }
    fn is_clear_at(&self, p: Pos) -> bool {
        self.clear[p.y as usize][p.x as usize]
    }

    #[allow(dead_code)]
    fn show_path(&self, path: &[Pos]) {
        for y in 0..self.clear.len() {
            for x in 0..self.clear[0].len() {
                let pos = Pos { x: x as Scalar, y: y as Scalar };
                let char = match (pos == self.start, pos == self.end, self.clear[y][x]) {
                    (true, _, _) => 'S',
                    (_, true, _) => 'E',
                    (_, _, true) => '.',
                    (_, _, false) => '#',
                };
                if path.iter().any(|&s| s == pos) {
                    print!("O");
                } else {
                    print!("{char}");
                }
            }
            println!();
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.clear.len() {
            for x in 0..self.clear[0].len() {
                let pos = Pos { x: x as Scalar, y: y as Scalar };
                let char = match (pos == self.start, pos == self.end, self.clear[y][x]) {
                    ( true,    _,     _) => 'S',
                    (    _, true,     _) => 'E',
                    (    _,    _, true ) => '.',
                    (    _,    _, false) => '#',
                };
                write!(f, "{char}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cheat(Pos, Pos);

