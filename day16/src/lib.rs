// use std::sync::Arc;
use std::rc::Rc;

aoc_tools::aoc_sol!(day16: part1, part2);
type Scalar = i16;
aoc_tools::pos!(i16);
aoc_tools::fast_hash!();
// aoc_tools::arena!();


pub fn part1(input: &str) -> usize {
    let map = parse_input(input);

    map.min_score(false).0
}

pub fn part2(input: &str) -> usize {
    let map = parse_input(input);

    let positions = map.min_score(true).1;
    positions.len()
}

fn parse_input(input: &str) -> Map {
    let mut start = Pos { x: 0, y: 0 };
    let mut end = Pos { x: 0, y: 0 };
    let clear = input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(y, l)| {
            l.bytes().enumerate().map(|(x, b)| match b {
                b'#' => false,
                b'.' => true,
                b'S' => {
                    start = Pos { x: x as Scalar, y: y as Scalar };
                    true
                },
                b'E' => {
                    end = Pos { x: x as Scalar, y: y as Scalar };
                    true
                },
                _ => panic!("Invalid char {b}")
            }).collect()
        })
        .collect();

    Map { clear, start, end }
}

#[derive(Clone)]
struct Map {
    clear: Vec<Vec<bool>>,
    start: Pos,
    end: Pos,
}

impl Map {
    fn min_score(&self, track_histories: bool) -> (usize, FastSet<Pos>) {
        // let longest_path_estimate = (self.clear.len() + self.clear[0].len()) * 8;

        let mut current_vals: FastMap<ReindeerState, (usize, History)> = [
            ReindeerState { pos: self.start, direction: Pos { x:  1, y:  0 } },
        ].into_iter().map(|v| (
            v,
            (0, if track_histories {
                History::Base(ReindeerState { pos: self.start, direction: Pos { x:  1, y:  0 } })
            } else {
                History::Null
            })
        )).collect();
        let mut records: FastMap<ReindeerState, usize> = current_vals.iter().map(|(&k, (v, _))| (k, *v)).collect();
        let mut new_vals: FastMap<ReindeerState, (usize, Vec<History>)> = new_fastmap_with_capacity(64);


        let mut winning_positions = new_fastset_with_capacity(self.clear.len() * 8);
        let mut winning_score = usize::MAX;

        while !current_vals.is_empty() {
            for (state, (score, history)) in current_vals.drain() {
                let l = state.turn_l().step();
                if self.clear_at(l.pos) {
                    let old_score = records.get(&l).copied().unwrap_or(usize::MAX);
                    let curr_score = new_vals.get_mut(&l).map(|(a, _)| *a).unwrap_or(usize::MAX);
                    let new_score = score + 1001;

                    if new_score <= old_score {
                        if new_score == curr_score {
                            new_vals.get_mut(&l).unwrap().1.push(history.clone());
                        } else if new_score < curr_score {
                            let mut new_vec = Vec::with_capacity(8);
                            new_vec.push(history.clone());
                            new_vals.insert(l, (new_score, new_vec));
                        }
                    }
                }

                let r = state.turn_r().step();
                if self.clear_at(r.pos) {
                    let old_score = records.get(&r).copied().unwrap_or(usize::MAX);
                    let curr_score = new_vals.get_mut(&r).map(|(a, _)| *a).unwrap_or(usize::MAX);
                    let new_score = score + 1001;

                    if new_score <= old_score {
                        if new_score == curr_score {
                            new_vals.get_mut(&r).unwrap().1.push(history.clone());
                        } else if new_score < curr_score {
                            let mut new_vec = Vec::with_capacity(8);
                            new_vec.push(history.clone());
                            new_vals.insert(r, (new_score, new_vec));
                        }
                    }
                }
                let f = state.step();
                if self.clear_at(f.pos) {
                    let old_score = records.get(&f).copied().unwrap_or(usize::MAX);
                    let curr_score = new_vals.get_mut(&f).map(|(a, _)| *a).unwrap_or(usize::MAX);
                    let new_score = score + 1;

                    if new_score <= old_score {
                        if new_score == curr_score {
                            new_vals.get_mut(&f).unwrap().1.push(history.clone());
                        } else if new_score < curr_score {
                            let mut new_vec = Vec::with_capacity(8);
                            new_vec.push(history.clone());
                            new_vals.insert(f, (new_score, new_vec));
                        }
                    }
                }
            }

            for (state, (score, history)) in new_vals.drain() {
                let new_history = || if track_histories {
                    History::With(Rc::new(history), state)
                } else {
                    History::Null
                };

                if state.pos == self.end {
                    if score < winning_score {
                        winning_positions.clear();
                    }
                    if score <= winning_score {
                        winning_score = score;
                        new_history().add_positions(&mut winning_positions);
                    }
                } else {
                    let record = *records.entry(state).or_insert(usize::MAX);
                    if record == score {
                        current_vals.get_mut(&state).unwrap().1 = new_history();
                    } else if score < record {
                        records.insert(state, score);
                        current_vals.insert(state, (score, new_history()));
                    }
                }
            }
        }

        (winning_score, winning_positions)
    }

    fn clear_at(&self, p: Pos) -> bool {
        self.clear[p.y as usize][p.x as usize]
    }

    fn show_path(&self, path: &[ReindeerState]) {
        for y in 0..self.clear.len() {
            for x in 0..self.clear[0].len() {
                let pos = Pos { x: x as Scalar, y: y as Scalar };
                let char = match (pos == self.start, pos == self.end, self.clear[y][x]) {
                    (true, _, _) => 'S',
                    (_, true, _) => 'E',
                    (_, _, true) => '.',
                    (_, _, false) => '#',
                };
                if path.iter().any(|s| s.pos == pos) {
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

#[derive(Debug, Clone)]
enum History {
    Null,
    Base(ReindeerState),
    With(Rc<Vec<History>>, ReindeerState),
}

impl History {
    pub fn vecs(&self) -> Box<dyn Iterator<Item = Vec<ReindeerState>> + '_> {
        match self {
            Self::Null => Box::new([].into_iter()),
            Self::Base(state) => Box::new([vec![*state]].into_iter()),
            Self::With(rc, state) => Box::new(rc.iter().flat_map(
                move |h| h.vecs().map(move |mut v| { v.push(*state); v })
            )),
        }
    }

    fn add_positions(&self, working_set: &mut FastSet<Pos>) {
        match self {
            Self::Null => (),
            Self::Base(state) => { working_set.insert(state.pos); },
            Self::With(rc, state) => {
                for history in rc.iter() {
                    history.add_positions(working_set);
                }
                working_set.insert(state.pos);
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ReindeerState {
    pos: Pos,
    direction: Pos,
}

impl ReindeerState {
    pub fn step(&self) -> Self {
        Self {
            pos: self.pos.add(self.direction),
            direction: self.direction,
        }
    }

    pub fn turn_r(&self) -> Self {
        Self {
            pos: self.pos,
            direction: Pos {
                x: -self.direction.y,
                y: self.direction.x,
            }
        }
    }

    pub fn turn_l(&self) -> Self {
        Self {
            pos: self.pos,
            direction: Pos {
                x: self.direction.y,
                y: -self.direction.x,
            }
        }
    }
}

impl Debug for ReindeerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self.direction {
            Pos { x:  0, y: -1 } => '^',
            Pos { x:  0, y:  1 } => 'v',
            Pos { x: -1, y:  0 } => '<',
            Pos { x:  1, y:  0 } => '>',
            _ => panic!("Invalid direction"),
        };
        write!(f, "{ch}")?;
        write!(f, "({}, {})", self.pos.x, self.pos.y)
    }
}


