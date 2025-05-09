use std::str::FromStr;

aoc_tools::aoc_sol!(day18 2021: part1, part2);

enum ReduceAction {
    Explode(u8, u8),
    AddLeft(u8),
    AddRight(u8),
    None,
    Handled,
}

#[derive(Clone, PartialEq, Eq)]
enum Entry {
    Literal(u8),
    Snailfish(Box<Number>),
}
impl Entry {
    pub fn split(n: u8) -> Self {
        Self::Snailfish(Box::new(Number(
            Entry::Literal(n / 2),
            Entry::Literal((n+1) / 2),
        )))
    }
    pub fn reduce_explode(&mut self, level: u8) -> ReduceAction {
        match self {
            Self::Snailfish(number) => number.reduce_explode(level + 1),
            _ => ReduceAction::None,
        }
    }
    pub fn reduce_split(&mut self) -> bool {
        match self {
            Self::Snailfish(number) => number.reduce_split(),
            &mut Self::Literal(l) => if l > 9 {
                *self = Self::split(l);
                true
            } else {
                false
            }
        }
    }
    pub fn as_literal(&self) -> Option<u8> {
        if let &Self::Literal(literal) = self {
            Some(literal)
        } else {
            None
        }
    }
    pub fn add_to_leftmost(&mut self, n: u8) {
        match self {
            Self::Literal(l) => *l += n,
            Self::Snailfish(num) => num.add_to_leftmost(n),
        }
    }
    pub fn add_to_rightmost(&mut self, n: u8) {
        match self {
            Self::Literal(l) => *l += n,
            Self::Snailfish(num) => num.add_to_rightmost(n),
        }
    }
    pub fn mag(&self) -> u16 {
        match self {
            Self::Literal(l) => *l as u16,
            Self::Snailfish(n) => n.mag(),
        }
    }
}
impl Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(n) => n.fmt(f),
            Self::Snailfish(n) => n.fmt(f),
        }
    }
}
impl FromStr for Entry {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.as_bytes()[0].is_ascii_alphanumeric() {
            Ok(Self::Literal(s.parse::<u8>().map_err(|e| e.to_string())?))
        } else {
            Ok(Self::Snailfish(Box::new(s.parse()?)))
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Number(Entry, Entry);
impl Number {
    pub fn reduce_split(&mut self) -> bool {
        if !self.0.reduce_split() {
            self.1.reduce_split()
        } else {
            true
        }
    }
    pub fn reduce_explode(&mut self, level: u8) -> ReduceAction {
        if level >= 4 {
            ReduceAction::Explode(self.0.as_literal().unwrap(), self.1.as_literal().unwrap())
        } else {
            match self.0.reduce_explode(level) {
                ReduceAction::Explode(l, r) => {
                    self.0 = Entry::Literal(0);
                    self.1.add_to_leftmost(r);
                    return ReduceAction::AddLeft(l);
                },
                ReduceAction::AddRight(r) => {
                    self.1.add_to_leftmost(r);
                    return ReduceAction::Handled;
                },
                ReduceAction::AddLeft(l) => return ReduceAction::AddLeft(l),
                ReduceAction::Handled => return ReduceAction::Handled,
                ReduceAction::None => (),
            }
            match self.1.reduce_explode(level) {
                ReduceAction::Explode(l, r) => {
                    self.1 = Entry::Literal(0);
                    self.0.add_to_rightmost(l);
                    ReduceAction::AddRight(r)
                },
                ReduceAction::AddLeft(l) => {
                    self.0.add_to_rightmost(l);
                    ReduceAction::Handled
                },
                rest => rest,
            }
        }
    }
    pub fn add_to_leftmost(&mut self, n: u8) {
        self.0.add_to_leftmost(n)
    }
    pub fn add_to_rightmost(&mut self, n: u8) {
        self.1.add_to_rightmost(n)
    }

    pub fn reduce_once(&mut self) -> bool {
        match self.reduce_explode(0) {
            ReduceAction::None => self.reduce_split(),
            _ => true,
        }
    }
    pub fn reduce(&mut self) {
        while self.reduce_once() {}
    }
    pub fn add(self, other: Self) -> Self {
        Self(
            Entry::Snailfish(Box::new(self)),
            Entry::Snailfish(Box::new(other)),
        )
    }
    pub fn add_reduce(self, other: Self) -> Self {
        let mut added = self.add(other);
        added.reduce();
        added
    }
    pub fn mag(&self) -> u16 {
        self.0.mag() * 3 + self.1.mag() * 2
    }
}
impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entry(&self.0).entry(&self.1).finish()
    }
}
impl FromStr for Number {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix('[') else { return Err("Missing leading [".to_string()) };
        let Some(s) = s.strip_suffix(']') else { return Err("Missing trailing ]".to_string()) };
        let mut nested = 0_u8;
        let mut i = 0;
        let left = loop {
            match s.as_bytes()[i] {
                b'[' => nested += 1,
                b']' => nested -= 1,
                b',' => if nested == 0 {
                    break &s[..i];
                }
                _ => (),
            }
            i += 1;
        };
        let right = &s[left.len()+1..];
        // let (left, right) = (left.parse()?, right.parse()?);
        // println!("{left}, {right}");
        let (left, right) = (left.parse().unwrap(), right.parse().unwrap());
        Ok(Self(left, right))
    }
}

pub fn part1(input: &str) -> u16 {
    let mut numbers = parse_input(input);
    let mut curr = numbers.remove(0);
    for number in numbers {
        curr = curr.add_reduce(number);
    }
    println!("{curr:?}");
    curr.mag()
}

pub fn part2(input: &str) -> u16 {
    let numbers = parse_input(input);
    let mut max_mag = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j { continue }
            let sum = numbers[i].clone().add_reduce(numbers[j].clone());
            max_mag = max_mag.max(sum.mag());
        }
    }
    max_mag
}

fn parse_input(input: &str) -> Vec<Number> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}
