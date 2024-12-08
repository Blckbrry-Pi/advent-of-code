use std::{ collections::HashMap, str::FromStr };

fn main() {
    part1();
    part2();
}

const TEST: &str = include_str!("../../data/day7/test.txt");
const INPUT: &str = include_str!("../../data/day7/input.txt");

fn part1() {
    let start = std::time::Instant::now();
    let equations = parse_input(INPUT);

    let mut sum = 0;
    for equation in equations {
        if equation.solve().is_some() {
            sum += equation.target;
        }
    }

    println!("Part 1: {sum} ({:?})", start.elapsed());
}

fn part2() {
    let start = std::time::Instant::now();
    let equations = parse_input(INPUT);

    let mut sum = 0;
    for equation in equations {
        if equation.solve_concat().is_some() {
            sum += equation.target;
        }
    }

    println!("Part 2: {sum} ({:?})", start.elapsed());
}

fn parse_input(input: &str) -> Vec<Equation> {
    input.lines()
        .map(|l| {
            let (target, values) = l.split_once(": ").unwrap();
            let target = target.parse().unwrap();
            let values: Vec<isize> = values.split(' ').map(str::parse).map(Result::unwrap).collect();

            Equation { target, values }
        })
        .collect()
}

#[derive(Clone, PartialEq, Eq)]
struct Equation {
    target: isize,
    values: Vec<isize>,
}

impl Equation {
    fn solve_recursive(target: isize, curr: isize, remaining: &[isize], operators: &[Operator]) -> Option<Vec<Operator>> {
        if remaining.is_empty() {
            if target == curr {
                return Some(vec![]);
            } else {
                return None;
            }
        }

        let next = remaining[0];
        let remaining = &remaining[1..];
        for &op in operators {
            let curr = op.apply(curr, next);
            if let Some(mut sol) = Self::solve_recursive(target, curr, remaining, operators) {
                sol.push(op);
                return Some(sol)
            }
        }

        None
    }
    pub fn solve(&self) -> Option<Vec<Operator>> {
        Self::solve_recursive(
            self.target,
            self.values[0],
            &self.values[1..],
            &[Operator::Mult, Operator::Plus],
        ).map(|mut v| { v.reverse(); v })
    }
    pub fn solve_concat(&self) -> Option<Vec<Operator>> {
        Self::solve_recursive(
            self.target,
            self.values[0],
            &self.values[1..],
            &[Operator::Mult, Operator::Plus, Operator::Concat],
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator { Plus, Mult, Concat }
impl Operator {
    pub fn apply(&self, a: isize, b: isize) -> isize {
        match self {
            Self::Plus => a + b,
            Self::Mult => a * b,
            Self::Concat => {
                let b10_size = (b as f64).log10() + 1.0;
                let b10_size = b10_size.floor() as u32;
                let a_coeff = 10_isize.pow(b10_size);
                a * a_coeff + b
                // format!("{a}{b}").parse().unwrap()
            }
        }
    }
}
