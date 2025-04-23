use std::fmt::{Display, Formatter};
use std::str::FromStr;

aoc_tools::aoc_sol!(day25 2022: part1);

pub fn part1(input: &str) -> Number {
    let numbers = parse_input(input);
    let sum: i64 = numbers.into_iter().sum();
    Number(to_balanced_quinary(sum))
}

pub fn part2(input: &str) -> usize { input.len() }

fn from_balanced_quinary(balanced: impl Iterator<Item = Digit>) -> i64 {
    let mut value = 0;
    for digit in balanced {
        value *= 5;
        value += digit as i64;
    }
    value
}
fn to_balanced_quinary(input: i64) -> Vec<Digit> {
    let mut digits = vec![];
    let digit_count = (input * 2 - 1).ilog(5) + 1;
    let offset = from_balanced_quinary(std::iter::repeat(Digit::Plu2).take(digit_count as usize));
    let mut remaining = input + offset;
    while remaining > 0 {
        let digit = match remaining % 5 {
            0 => Digit::Min2,
            1 => Digit::Min1,
            2 => Digit::Zero,
            3 => Digit::Plu1,
            4 => Digit::Plu2,
            _ => unreachable!(),
        };
        digits.push(digit);
        remaining /= 5;
    }
    digits.reverse();
    digits
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Digit {
    Min2 = -2,
    Min1 = -1,
    Zero = 0,
    Plu1 = 1,
    Plu2 = 2,
}
impl Debug for Digit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Digit::Min2 => write!(f, "="),
            Digit::Min1 => write!(f, "-"),
            Digit::Zero => write!(f, "0"),
            Digit::Plu1 => write!(f, "1"),
            Digit::Plu2 => write!(f, "2"),
        }
    }
}
impl FromStr for Digit {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "=" => Ok(Digit::Min2),
            "-" => Ok(Digit::Min1),
            "0" => Ok(Digit::Zero),
            "1" => Ok(Digit::Plu1),
            "2" => Ok(Digit::Plu2),
            _ => Err(format!("Invalid digit '{}'", s)),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Number(Vec<Digit>);
impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for &digit in &self.0 {
            write!(f, "{digit:?}")?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| (0..line.len()).map(|i| &line[i..i+1]).map(|line| line.parse::<Digit>().unwrap()))
        .map(|digits| from_balanced_quinary(digits))
        .collect()
}
