use std::fmt::Formatter;
use aoc_tools::SmallVec;

aoc_tools::aoc_sol!(day17 2022: part1, part2);
type Scalar = i64;
aoc_tools::pos!(Scalar; +y => U);

pub fn part1(input: &str) -> Scalar {
    let jets = parse_input(input);
    let mut state = State::new(jets);
    let mut fallen = 0;
    while fallen < 2022 {
        if state.advance() {
            fallen += 1;
        }
    }
    state.height()
}

pub fn part2(input: &str) -> Scalar {
    let jets = parse_input(input);
    let mut state = State::new(jets);
    let mut seen = HashMap::new();
    seen.insert(state.view(), (0, state.height()));
    let mut fallen = 0 as Scalar;
    while fallen < 1_000_000_000_000 {
        if state.advance() {
            fallen += 1;
            state.remove_removable();
            if let Some((old_fallen, old_height)) = seen.insert(state.view(), (fallen, state.height())) {
                let diff = fallen - old_fallen;
                let reps_needed = (1_000_000_000_000 - fallen) / diff;
                fallen += reps_needed * diff;
                state.removed += reps_needed * (state.height() - old_height);
                break;
            }
        }
    }
    while fallen < 1_000_000_000_000 {
        if state.advance() {
            fallen += 1;
        }
    }
    state.height()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Jet { L, R }
impl Debug for Jet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Jet::L => write!(f, "<"),
            Jet::R => write!(f, ">"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Hori,
    Plus,
    Angl,
    Vert,
    Sqre,
}
impl Rock {
    pub fn next(&self) -> Self {
        match self {
            Self::Hori => Self::Plus,
            Self::Plus => Self::Angl,
            Self::Angl => Self::Vert,
            Self::Vert => Self::Sqre,
            Self::Sqre => Self::Hori,
        }
    }
    pub fn pos_iter(&self) -> impl Iterator<Item = Pos> {
        match self {
            Self::Hori => [
                Pos { x: 0, y: 0 },
                Pos { x: 1, y: 0 },
                Pos { x: 2, y: 0 },
                Pos { x: 3, y: 0 },
            ].as_slice(),
            Self::Plus => [
                Pos { x: 1, y: 0 },
                Pos { x: 0, y: 1 },
                Pos { x: 1, y: 1 },
                Pos { x: 2, y: 1 },
                Pos { x: 1, y: 2 },
            ].as_slice(),
            Self::Angl => [
                Pos { x: 0, y: 0 },
                Pos { x: 1, y: 0 },
                Pos { x: 2, y: 0 },
                Pos { x: 2, y: 1 },
                Pos { x: 2, y: 2 },
            ].as_slice(),
            Self::Vert => [
                Pos { x: 0, y: 0 },
                Pos { x: 0, y: 1 },
                Pos { x: 0, y: 2 },
                Pos { x: 0, y: 3 },
            ].as_slice(),
            Self::Sqre => [
                Pos { x: 0, y: 0 },
                Pos { x: 1, y: 0 },
                Pos { x: 0, y: 1 },
                Pos { x: 1, y: 1 },
            ].as_slice(),
        }.iter().copied()
    }
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    fallen: SmallVec<90, u8>,
    rock: (Rock, Pos),
    stream: Vec<Jet>,
    stream_idx: usize,
    removed: Scalar,
}
impl State {
    pub fn new(stream: Vec<Jet>) -> Self {
        Self {
            fallen: SmallVec::new(),
            rock: (Rock::Hori, Pos { x: 2, y: 3 }),
            stream,
            stream_idx: 0,
            removed: 0,
        }
    }
    pub fn advance(&mut self) -> bool {
        let saved_pos = self.rock.1;
        match self.stream[self.stream_idx] {
            Jet::L => self.rock.1.x -= 1,
            Jet::R => self.rock.1.x += 1,
        }
        self.stream_idx += 1;
        self.stream_idx %= self.stream.len();
        if self.rock_iter().any(|pos| !self.clear_at(pos)) {
            self.rock.1 = saved_pos;
        }
        let saved_pos = self.rock.1;
        self.rock.1.y -= 1;
        if self.rock.1.y < 0 || self.rock_iter().any(|pos| !self.clear_at(pos)) {
            self.rock.1 = saved_pos;
            for pos in self.rock_iter() {
                self.add_rock_at(pos);
                if self.fallen[pos.y as usize] == 0 {
                    self.remove_n(pos.y.saturating_sub(3));
                }
            }
            self.rock.0 = self.rock.0.next();
            self.rock.1.x = 2;
            self.rock.1.y = self.fallen.len() as Scalar + 3;
            true
        } else {
            false
        }
    }
    pub fn remove_removable(&mut self) {
        let mut seen = HashSet::new();
        let mut stack: Vec<_> = (0..7)
            .map(|x| Pos { x, y: self.fallen.len() as Scalar - 1 })
            .collect();
        while let Some(pos) = stack.pop() {
            if seen.contains(&pos) { continue }
            if pos.y >= self.fallen.len() as Scalar { continue }
            seen.insert(pos);
            let tests = [
                pos.add(Pos::N),
                pos.add(Pos::S),
                pos.add(Pos::E),
                pos.add(Pos::W),
            ];
            for test in tests {
                if self.clear_at(test) {
                    stack.push(test);
                }
            }
        }
        let min_y = seen.iter().map(|pos| pos.y).min().unwrap_or(0);
        self.remove_n(min_y);
    }
    pub fn remove_n(&mut self, n: Scalar) {
        let mut new_fallen = SmallVec::new();
        for i in n as usize..self.fallen.len() {
            new_fallen.push(self.fallen[i]);
        }
        self.fallen = new_fallen;
        self.removed += n;
        self.rock.1.y -= n;
    }
    pub fn clear_at(&self, pos: Pos) -> bool {
        if 0 > pos.y || 0 > pos.x || pos.x >= 7 {
            false
        } else if pos.y as usize >= self.fallen.len() {
            true
        } else {
            // println!("{:?}", self.fallen);
            self.fallen[pos.y as usize] & Self::mask(pos.x) == 0
        }
    }
    pub fn add_rock_at(&mut self, pos: Pos) {
        while pos.y as usize >= self.fallen.len() {
            self.fallen.push(0);
        }
        self.fallen[pos.y as usize] |= Self::mask(pos.x);
    }
    pub fn mask(x: Scalar) -> u8 {
        1 << x
    }
    pub fn rock_iter(&self) -> impl Iterator<Item = Pos> {
        let offset = self.rock.1;
        self.rock.0.pos_iter().map(move |pos| pos.add(offset))
    }
    pub fn view(&self) -> StateView {
        StateView(
            self.fallen.iter().copied().collect(),
            self.rock.0,
            self.rock.1,
            self.stream_idx
        )
    }
    pub fn height(&self) -> Scalar {
        self.fallen.len() as Scalar + self.removed as Scalar
    }
}
impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.fallen.len()+7).rev() {
            for x in 0..7 {
                let pos = Pos { x: x as Scalar, y: y as Scalar };

                if !self.clear_at(pos) {
                    write!(f, "#")
                } else if self.rock_iter().any(|rock_pos| rock_pos == pos) {
                    write!(f, "@")
                } else {
                    write!(f, ".")
                }?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct StateView(SmallVec<48, u8>, Rock, Pos, usize);

fn parse_input(input: &str) -> Vec<Jet> {
    input.chars().filter_map(|input| match input {
        '<' => Some(Jet::L),
        '>' => Some(Jet::R),
        _ => None,
    }).collect()
}
