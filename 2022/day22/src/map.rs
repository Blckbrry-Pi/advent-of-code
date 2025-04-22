use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use crate::{Pos, Scalar};
use crate::instructions::Instruction;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {
    pos: Pos,
    facing: Pos,
}
impl State {
    pub fn naive_advance(&self) -> Self {
        Self {
            pos: self.pos.add(self.facing),
            facing: self.facing,
        }
    }
    pub fn advance(&self, map: &Map) -> Self {
        let new = self.naive_advance();
        let new = map.redirects.get(&new).copied().unwrap_or(new);
        if map.rows[new.pos.y as usize][new.pos.x as usize] == Tile::Open {
            new
        } else {
            *self
        }
    }
    pub fn advance_n(&self, map: &Map, n: usize) -> Self {
        let mut curr = *self;
        for _ in 0..n {
            let new = curr.advance(map);
            if curr == new { break }
            curr = new;
        }
        curr
    }
    pub fn turn_l(&self) -> Self {
        State {
            pos: self.pos,
            facing: self.facing.turn_l(),
        }
    }
    pub fn turn_r(&self) -> Self {
        State {
            pos: self.pos,
            facing: self.facing.turn_r(),
        }
    }
    pub fn handle(&self, map: &Map, instruction: Instruction) -> Self {
        match instruction {
            Instruction::TurnL => self.turn_l(),
            Instruction::TurnR => self.turn_r(),
            Instruction::Advance(n) => self.advance_n(map, n)
        }
    }
    pub fn password(&self) -> Scalar {
        let facing =
            if self.facing == Pos::E { 0 }
            else if self.facing == Pos::S { 1 }
            else if self.facing == Pos::W { 2 }
            else { 3 };
        (self.pos.y + 1) * 1000 + (self.pos.x + 1) * 4 + facing
    }
}
impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) facing ", self.pos.x, self.pos.y)?;
        if self.facing == Pos::N {
            write!(f, "N")
        } else if self.facing == Pos::S {
            write!(f, "S")
        } else if self.facing == Pos::E {
            write!(f, "E")
        } else {
            write!(f, "W")
        }
    }
}

#[derive(Clone)]
pub struct Map {
    rows: Vec<Vec<Tile>>,
    redirects: HashMap<State, State>,
}
impl Map {
    pub fn start(&self) -> State {
        let x = (0..).find(|i| self.rows[0][*i] != Tile::OutOfBounds).unwrap();
        State {
            pos: Pos { x: x as Scalar, y: 0 },
            facing: Pos::E,
        }
    }
    pub fn setup_part1_redirects(&mut self) {
        self.redirects.reserve(self.rows.len() * 2 + self.rows[0].len() * 2);

        for y in 0..self.rows.len() {
            let mut first_not_oob = 0;
            while self.rows[y][first_not_oob as usize] == Tile::OutOfBounds {
                first_not_oob += 1;
            }
            let mut last_not_oob = self.rows[y].len() as Scalar - 1;
            while self.rows[y][last_not_oob as usize] == Tile::OutOfBounds {
                last_not_oob -= 1;
            }

            let y = y as Scalar;
            self.redirects.insert(
                State {
                    pos: Pos { x: first_not_oob - 1, y },
                    facing: Pos::W,
                },
                State {
                    pos: Pos { x: last_not_oob, y },
                    facing: Pos::W,
                }
            );
            self.redirects.insert(
                State {
                    pos: Pos { x: last_not_oob + 1, y },
                    facing: Pos::E,
                },
                State {
                    pos: Pos { x: first_not_oob, y },
                    facing: Pos::E,
                }
            );
        }
        for x in 0..self.rows[0].len() {
            let mut first_not_oob = 0;
            while self.rows[first_not_oob as usize][x] == Tile::OutOfBounds {
                first_not_oob += 1;
            }
            let mut last_not_oob = self.rows.len() as Scalar - 1;
            while self.rows[last_not_oob as usize][x] == Tile::OutOfBounds {
                last_not_oob -= 1;
            }

            let x = x as Scalar;
            self.redirects.insert(
                State {
                    pos: Pos { x, y: first_not_oob - 1 },
                    facing: Pos::N,
                },
                State {
                    pos: Pos { x, y: last_not_oob },
                    facing: Pos::N,
                }
            );
            self.redirects.insert(
                State {
                    pos: Pos { x, y: last_not_oob + 1 },
                    facing: Pos::S,
                },
                State {
                    pos: Pos { x, y: first_not_oob },
                    facing: Pos::S,
                }
            );
        }
    }
    /// Looking at the cube head on, the sides are...
    /// - 0: Front
    /// - 1: Right
    /// - 2: Back
    /// - 3: Left
    /// - 4: Top
    /// - 5: Bottom
    /// 
    /// The ranges are:
    /// - x
    ///     - 0 -> Left
    ///     - 1 -> Right
    /// - y
    ///     - 0 -> Top
    ///     - 1 -> Bottom
    /// - z
    ///     - 0 -> Front
    ///     - 1 -> Back
    pub fn setup_part2_redirects(&mut self) {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        struct Corner { x: i8, y: i8, z: i8 }
        fn get_swapped(
            keep: (Corner, Corner),
            swap: (Corner, Corner),
        ) -> (Corner, Corner) {
            let x_relevant = keep.0.x == keep.1.x;
            let y_relevant = keep.0.y == keep.1.y;
            let z_relevant = keep.0.z == keep.1.z;
            let mut output = swap;
            if x_relevant {
                output.0.x = 1 - output.0.x;
                output.1.x = 1 - output.1.x;
            }
            if y_relevant {
                output.0.y = 1 - output.0.y;
                output.1.y = 1 - output.1.y;
            }
            if z_relevant {
                output.0.z = 1 - output.0.z;
                output.1.z = 1 - output.1.z;
            }
            output
        }
        macro_rules! iter_options {
            ($side:ident, $block_size:ident: $([$a:ident -> $b:ident: $facing:expr, $($type:tt)+])+) => {
                [
                    $(
                        (
                            (iter_options!(@impl corner $side $a), iter_options!(@impl corner $side $b)),
                            Box::new(
                                iter_options!(@impl iter $($type)+).map(|pos| State { pos, facing: $facing })
                            ) as Box<dyn DoubleEndedIterator<Item = State>>,
                        ),
                    )+
                ]
            };
            (@impl corner $side:ident ul) => { $side.canon_ul };
            (@impl corner $side:ident ur) => { $side.canon_ur };
            (@impl corner $side:ident dl) => { $side.canon_dl };
            (@impl corner $side:ident dr) => { $side.canon_dr };

            (@impl iter $x:expr, $y:expr) => {
                ($x)
                    .zip($y)
                    .map(|(x, y)| Pos { x, y })
            };
        }
        enum MaybeRev<T, I: DoubleEndedIterator<Item = T>> {
            Forward(I),
            Backward(I),
        }
        impl<T, I: DoubleEndedIterator<Item = T>> Iterator for MaybeRev<T, I> {
            type Item = T;
            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Self::Forward(i) => i.next(),
                    Self::Backward(i) => i.next_back(),
                }
            }
        }
        fn get_to_iter(
            from: Corner,
            to: Corner,
            to_side: Side,
            block_size: Scalar, 
        ) -> Option<impl Iterator<Item = State>> {
            let repeat = |v| std::iter::repeat_n(v, block_size as usize);
            let options = iter_options!(
                to_side, block_size:
                [ul -> ur: Pos::S, to_side.ul.x..to_side.ul.x + block_size, repeat(to_side.ul.y)]
                [dl -> dr: Pos::N, to_side.ul.x..to_side.ul.x + block_size, repeat(to_side.ul.y + block_size - 1)]
                [ul -> dl: Pos::E, repeat(to_side.ul.x), to_side.ul.y..to_side.ul.y + block_size]
                [ur -> dr: Pos::W, repeat(to_side.ul.x + block_size - 1), to_side.ul.y..to_side.ul.y + block_size]
            );

            for (pair, iter) in options {
                if (from, to) == pair {
                    return Some(MaybeRev::Forward(iter))
                } else if (to, from) == pair {
                    return Some(MaybeRev::Backward(iter))
                }
            }
            None
        }
        fn get_from_iter(
            from: Corner,
            to: Corner,
            from_side: Side,
            block_size: Scalar,
        ) -> Option<impl Iterator<Item = State>> {
            let repeat = |v| std::iter::repeat_n(v, block_size as usize);
            let options = iter_options!(
                from_side, block_size:
                [ul -> ur: Pos::N, from_side.ul.x..from_side.ul.x + block_size, repeat(from_side.ul.y - 1)]
                [dl -> dr: Pos::S, from_side.ul.x..from_side.ul.x + block_size, repeat(from_side.ul.y + block_size)]
                [ul -> dl: Pos::W, repeat(from_side.ul.x - 1), from_side.ul.y..from_side.ul.y + block_size]
                [ur -> dr: Pos::E, repeat(from_side.ul.x + block_size), from_side.ul.y..from_side.ul.y + block_size]
            );
            for (pair, iter) in options {
                if (from, to) == pair {
                    return Some(MaybeRev::Forward(iter))
                } else if (to, from) == pair {
                    return Some(MaybeRev::Backward(iter))
                }
            }
            None
        }

        // The area will always be 3n*4n or 4n*3n because of how cube nets work
        let block_size = self.rows.len().min(self.rows[0].len()) as Scalar / 3;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct Side {
            canon_ul: Corner,
            canon_ur: Corner,
            canon_dl: Corner,
            canon_dr: Corner,
            ul: Pos,
        }

        let mut chosen = Vec::with_capacity(6);
        let start_pos = self.start().pos;
        let start_side = Side {
            canon_ul: Corner { x: 0, y: 0, z: 0 },
            canon_ur: Corner { x: 1, y: 0, z: 0 },
            canon_dl: Corner { x: 0, y: 1, z: 0 },
            canon_dr: Corner { x: 1, y: 1, z: 0 },
            ul: start_pos,
        };

        // Get all sides
        let mut seen = HashSet::with_capacity(12);
        let mut queue = Vec::with_capacity(12);
        queue.push(start_side);
        while let Some(maybe_side) = queue.pop() {
            // Reject already-seen/not-populated stuff
            if seen.contains(&maybe_side.ul) { continue }
            seen.insert(maybe_side.ul);
            if 0 > maybe_side.ul.y || maybe_side.ul.y as usize >= self.rows.len() { continue }
            if 0 > maybe_side.ul.x || maybe_side.ul.x as usize >= self.rows[0].len() { continue }
            if self.rows[maybe_side.ul.y as usize][maybe_side.ul.x as usize] == Tile::OutOfBounds {
                continue
            }
            chosen.push(maybe_side);

            // Left neighbor
            {
                let (canon_ul, canon_dl) = get_swapped(
                    (maybe_side.canon_ul, maybe_side.canon_dl),
                    (maybe_side.canon_ur, maybe_side.canon_dr),
                );
                queue.push(Side {
                    canon_ul,
                    canon_ur: maybe_side.canon_ul,
                    canon_dl,
                    canon_dr: maybe_side.canon_dl,
                    ul: maybe_side.ul.add(Pos { x: -block_size, y: 0 }),
                });
            }
            // Right neighbor
            {
                let (canon_ur, canon_dr) = get_swapped(
                    (maybe_side.canon_ur, maybe_side.canon_dr),
                    (maybe_side.canon_ul, maybe_side.canon_dl),
                );
                queue.push(Side {
                    canon_ul: maybe_side.canon_ur,
                    canon_ur,
                    canon_dl: maybe_side.canon_dr,
                    canon_dr,
                    ul: maybe_side.ul.add(Pos { x: block_size, y: 0 }),
                });
            }
            // Down neighbor
            {
                let (canon_dl, canon_dr) = get_swapped(
                    (maybe_side.canon_dl, maybe_side.canon_dr),
                    (maybe_side.canon_ul, maybe_side.canon_ur),
                );
                queue.push(Side {
                    canon_ul: maybe_side.canon_dl,
                    canon_ur: maybe_side.canon_dr,
                    canon_dl,
                    canon_dr,
                    ul: maybe_side.ul.add(Pos { x: 0, y: block_size }),
                });
            }
        }

        self.redirects.reserve(block_size as usize * 4 * 6);
        for from_side in chosen.iter().copied() {
            for (from, to) in [
                (from_side.canon_ul, from_side.canon_ur),
                (from_side.canon_dl, from_side.canon_dr),
                (from_side.canon_ul, from_side.canon_dl),
                (from_side.canon_ur, from_side.canon_dr),
            ] {
                let from_iter = get_from_iter(
                    from,
                    to,
                    from_side,
                    block_size,
                ).unwrap();
                let to_iter = chosen.iter().copied().find_map(|to_side| {
                    if to_side == from_side { return None }
                    get_to_iter(from, to, to_side, block_size)
                }).unwrap();
                for (from_state, to_state) in from_iter.zip(to_iter) {
                    self.redirects.insert(from_state, to_state);
                }
            }
        }
    }
}
impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().map(|line| line.len()).max().unwrap();
        let rows = s.lines()
            .map(|line| (0..width).map(|i| if i < line.len() {
                &line[i..i+1]
            } else {
                " "
            }))
            .map(|tiles| tiles.map(|s| s.parse()).collect())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            rows,
            redirects: HashMap::new(),
        })
    }
}
impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.rows.len() {
            for x in 0..self.rows[0].len() {
                write!(f, "{:?}", self.rows[y][x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    OutOfBounds,
    Open,
    Wall,
}
impl FromStr for Tile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            " " => Ok(Tile::OutOfBounds),
            "." => Ok(Tile::Open),
            "#" => Ok(Tile::Wall),
            _ => Err(format!("Unknown tile: {}", s)),
        }
    }
}
impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBounds => write!(f, " "),
            Self::Open => write!(f, "."),
            Self::Wall => write!(f, "#"),
        }
    }
}
