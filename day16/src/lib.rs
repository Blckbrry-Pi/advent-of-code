// use std::sync::Arc;
use std::rc::Rc;

aoc_tools::aoc_sol!(day16 test: part1, part2);
type Scalar = i16;
type ScoreScalar = u32;
aoc_tools::pos!(i16);
aoc_tools::fast_hash!();

pub fn part1(input: &str) -> ScoreScalar {
    let map = parse_input(input);

    map.min_score::<false>().0
}

pub fn part2(input: &str) -> usize {
    let map = parse_input(input);

    map.min_score::<true>().1.len()
}

fn parse_input(input: &str) -> Map {
    let mut start = Pos { x: 0, y: 0 };
    let mut end = Pos { x: 0, y: 0 };

    let capacity = (input.len() as f64).sqrt() as usize + 2;

    let input = input.trim().as_bytes();
    let mut clear = Vec::with_capacity(capacity);
    clear.push(Vec::with_capacity(capacity));

    let mut i = 0;
    let mut x = 0;
    let mut y = 0;
    while i < input.len() {
        let b = input[i];
        if b == b'\n' {
            x = 0;
            y += 1;
            i += 1;

            clear.push(Vec::with_capacity(capacity));
            continue;
        }

        clear[y as usize].push(match b {
            b'#' => false,
            b'S' => {
                start = Pos { x, y };
                true
            },
            b'E' => {
                end = Pos { x, y };
                true
            },
            _ => true,
        });

        x += 1;
        i += 1;
    }

    Map { clear, start, end }
}

#[derive(Clone)]
struct Map {
    clear: Vec<Vec<bool>>,
    start: Pos,
    end: Pos,
}

impl Map {
    fn min_score<const HISTORIES: bool>(&self) -> (ScoreScalar, FastSet<Pos>) {
        let mut current_vals: FastMap<ReindeerState, (ScoreScalar, History)> = [
            ReindeerState { pos: self.start, direction: Pos { x: 1, y:  0 } },
        ].into_iter().map(|v| (
            v,
            (0, if HISTORIES {
                History::Base(ReindeerState { pos: self.start, direction: Pos { x:  1, y:  0 } })
            } else {
                History::Null
            })
        )).collect();
        let mut records: FastMap<ReindeerState, ScoreScalar> = current_vals.iter().map(|(&k, (v, _))| (k, *v)).collect();
        let mut new_vals: FastMap<ReindeerState, (ScoreScalar, Vec<History>)> = new_fastmap_with_capacity(64);

        let mut winning_positions = new_fastset_with_capacity(self.clear.len() * 8);
        let mut winning_score = ScoreScalar::MAX;

        while !current_vals.is_empty() {
            for (state, (score, history)) in current_vals.drain() {
                let new_states = [
                    (state.turn_l().step(), score + 1001),
                    (state.turn_r().step(), score + 1001),
                    (state.step(), score + 1),
                ];
                for (new_state, new_score) in new_states {
                    if self.clear_at(new_state.pos) {
                        let old_score = records.get(&new_state).copied().unwrap_or(ScoreScalar::MAX);
                        let curr_score = new_vals.get_mut(&new_state).map(|(a, _)| *a).unwrap_or(ScoreScalar::MAX);
                        // let new_score = score + 1001;

                        if new_score <= old_score {
                            if new_score == curr_score {
                                new_vals.get_mut(&new_state).unwrap().1.push(history.clone());
                            } else if new_score < curr_score {
                                let mut new_vec = Vec::with_capacity(8);
                                new_vec.push(history.clone());
                                new_vals.insert(new_state, (new_score, new_vec));
                            }
                        }
                    }
                }
            }

            for (state, (score, history)) in new_vals.drain() {
                let new_history = || if HISTORIES {
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
                    let record = *records.entry(state).or_insert(ScoreScalar::MAX);
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

    #[allow(dead_code)]
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
    #[allow(dead_code)]
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


