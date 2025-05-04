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
struct Set(u8);

macro_rules! number {
    ($($set_to_true:ident)*) => {
        {
            let mut output = Set(0);
            $(output.add(Letter::$set_to_true);)*
            output
        }
    };
}
impl Set {
    pub const NUMBERS: [Self; 10] = [
        number!(A B C E F G),
        number!(C F),
        number!(A C D E G),
        number!(A C D F G),
        number!(B C D F),
        number!(A B D F G),
        number!(A B D E F G),
        number!(A C F),
        number!(A B C D E F G),
        number!(A B C D F G),
    ];

    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn intersect(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }
    pub const fn union(&self, other: &Self) -> Self {
        Self(self.0 | other.0)
    }
    pub const fn complement(&self) -> Self {
        Self(!self.0)
    }

    pub const fn count(&self) -> u8 {
        self.0.count_ones() as u8
    }
    pub const fn has(&self, letter: Letter) -> bool {
        (self.0 >> letter as u32) & 1 != 0
    }
    pub const fn add(&mut self, letter: Letter) {
        self.0 |= 1 << letter as u32;
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
        use Letter::*;
        let mut set = f.debug_set();
        if self.has(A) { set.entry(&"a"); }
        if self.has(B) { set.entry(&"b"); }
        if self.has(C) { set.entry(&"c"); }
        if self.has(D) { set.entry(&"d"); }
        if self.has(E) { set.entry(&"e"); }
        if self.has(F) { set.entry(&"f"); }
        if self.has(G) { set.entry(&"g"); }
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
