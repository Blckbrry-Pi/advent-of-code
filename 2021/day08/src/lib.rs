#![feature(iterator_try_collect)]

use std::str::FromStr;

use aoc_tools::SmallVec;

aoc_tools::aoc_sol!(day08 2021: part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Letter { A = 0, B = 1, C = 2, D = 3, E = 4, F = 5, G = 6 }
impl Letter {
    pub const fn in_order() -> [Self; 7] {
        use Letter::*;
        [A, B, C, D, E, F, G]
    }
    pub const fn from_char(c: char) -> Option<Self> {
        let output = match c {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            'e' => Self::E,
            'f' => Self::F,
            'g' => Self::G,
            _ => return None,
        };
        Some(output)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Set {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
}

macro_rules! number {
    ($($set_to_true:ident)*) => {
        Set {
            $($set_to_true: true,)*
            ..Set::empty()
        }
    };
}
impl Set {
    pub const NUMBERS: [Self; 10] = [
        number!(a b c e f g),
        number!(c f),
        number!(a c d e g),
        number!(a c d f g),
        number!(b c d f),
        number!(a b d f g),
        number!(a b d e f g),
        number!(a c f),
        number!(a b c d e f g),
        number!(a b c d f g),
    ];

    pub const fn empty() -> Self {
        Self {
            a: false,
            b: false,
            c: false,
            d: false,
            e: false,
            f: false,
            g: false,
        }
    }
    pub const fn intersect(&self, other: &Self) -> Self {
        Self {
            a: self.a && other.a,
            b: self.b && other.b,
            c: self.c && other.c,
            d: self.d && other.d,
            e: self.e && other.e,
            f: self.f && other.f,
            g: self.g && other.g,
        }
    }
    pub const fn union(&self, other: &Self) -> Self {
        Self {
            a: self.a || other.a,
            b: self.b || other.b,
            c: self.c || other.c,
            d: self.d || other.d,
            e: self.e || other.e,
            f: self.f || other.f,
            g: self.g || other.g,
        }
    }
    pub const fn complement(&self) -> Self {
        Self {
            a: !self.a,
            b: !self.b,
            c: !self.c,
            d: !self.d,
            e: !self.e,
            f: !self.f,
            g: !self.g,
        }
    }

    pub const fn count(&self) -> u8 {
        self.a as u8 +
        self.b as u8 +
        self.c as u8 +
        self.d as u8 +
        self.e as u8 +
        self.f as u8 +
        self.g as u8
    }
    pub const fn has(&self, letter: Letter) -> bool {
        match letter {
            Letter::A => self.a,
            Letter::B => self.b,
            Letter::C => self.c,
            Letter::D => self.d,
            Letter::E => self.e,
            Letter::F => self.f,
            Letter::G => self.g,
        }
    }
    pub const fn add(&mut self, letter: Letter) {
        match letter {
            Letter::A => self.a = true,
            Letter::B => self.b = true,
            Letter::C => self.c = true,
            Letter::D => self.d = true,
            Letter::E => self.e = true,
            Letter::F => self.f = true,
            Letter::G => self.g = true,
        }
    }
    pub fn first(self) -> Option<Letter> {
        self.into_iter().next()
    }
    pub fn into_iter(self) -> impl Iterator<Item = Letter> {
        Letter::in_order().into_iter().filter(move |l| self.has(*l))
    }
}
impl FromStr for Set {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut output = Self::empty();
        for c in s.chars() {
            let Some(letter) = Letter::from_char(c) else {
                return Err(format!("Invalid character {c}"));
            };
            output.add(letter);
        }
        Ok(output)
    }
}
impl Debug for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut set = f.debug_set();
        if self.a { set.entry(&"a"); }
        if self.b { set.entry(&"b"); }
        if self.c { set.entry(&"c"); }
        if self.d { set.entry(&"d"); }
        if self.e { set.entry(&"e"); }
        if self.f { set.entry(&"f"); }
        if self.g { set.entry(&"g"); }
        set.finish()
    }
}

fn deduce_mapping(numbers: [Set; 10]) -> [Letter; 7] {
    let n1 = numbers.into_iter().find(|n| n.count() == 2).unwrap();
    let n7 = numbers.into_iter().find(|n| n.count() == 3).unwrap();
    let n4 = numbers.into_iter().find(|n| n.count() == 4).unwrap();
    let n8 = numbers.into_iter().find(|n| n.count() == 7).unwrap();

    let n235: SmallVec<3, _> = numbers.into_iter().filter(|n| n.count() == 5).collect();

    let a = n7.intersect(&n1.complement()).first().unwrap();

    let n2 = *n235.iter().find(|&&n| n4.union(&n) == n8).unwrap();
    let n3 = *n235.iter().find(|&&n| n1.union(&n) == n).unwrap();
    let n5 = *n235.iter().find(|&&n| n != n2 && n != n3).unwrap();

    let b = n2.union(&n1).complement().first().unwrap();
    let c = n1.intersect(&n5.complement()).first().unwrap();
    let d = n4.intersect(&n2).intersect(&n5).first().unwrap();
    let e = n1.union(&n5).complement().first().unwrap();
    let f = n1.intersect(&n2.complement()).first().unwrap();
    let g = n7.union(&n4).complement().intersect(&n5).first().unwrap();

    [a, b, c, d, e, f, g]
}

fn map(mapping: [Letter; 7], wires: Set) -> Set {
    let mut output = Set::empty();
    for wire in wires.into_iter() {
        let mapped = Letter::in_order()
            .into_iter()
            .zip(mapping)
            .find_map(|(actual, aliased)| (wire == aliased).then_some(actual))
            .unwrap();
        output.add(mapped);
    }
    output
}

#[derive(Debug, Clone, Copy, PartialEq, )]
struct Display {
    numbers: [Set; 10],
    output: [Set; 4],
}
impl Display {
    fn digits(&self) -> [u8; 4] {
        let mapping = deduce_mapping(self.numbers);
        self.output
            .map(|output| map(mapping, output))
            .map(
                |output| Set::NUMBERS
                    .into_iter()
                    .enumerate()
                    .find_map(|(i, n)| (n == output).then_some(i as u8))
                    .unwrap(),
            )
    }
}
impl FromStr for Display {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((numbers, output)) = s.split_once(" | ") else {
            return Err("Missing delimiter".to_string());
        };
        let mut numbers = numbers.split(' ');
        let mut output = output.split(' ');
        let mut result = Self {
            numbers: [Set::empty(); 10],
            output: [Set::empty(); 4],
        };
        for i in 0..10 {
            let number = numbers.next().unwrap();
            let number: Set = number.parse()?;
            result.numbers[i] = number;
        }
        for i in 0..4 {
            let number = output.next().unwrap();
            let number: Set = number.parse()?;
            result.output[i] = number;
        }
        Ok(result)
    }
}

pub fn part1(input: &str) -> u32 {
    let displays = parse_input(input);
    let mut count_1478 = 0;
    for display in displays {
        let digits = display.digits();
        for digit in digits {
            if digit == 1 || digit == 4 || digit == 7 || digit == 8 {
                count_1478 += 1;
            }
        }
    }
    
    count_1478
}

pub fn part2(input: &str) -> u32 {
    let displays = parse_input(input);
    let mut sum = 0;
    for display in displays {
        let digits = display.digits();
        let thou = digits[0] as u32 * 1000;
        let hund = digits[1] as u32 * 100;
        let tens = digits[2] as u32 * 10;
        let ones = digits[3] as u32;

        sum += thou + hund + tens + ones;
    }
    
    sum
}

fn parse_input(input: &str) -> Vec<Display> {
    input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}
