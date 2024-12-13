aoc_tools::aoc_sol!(day03: part1, part2);

fn part1(input: &str) -> i64 {
    let instructions = parse_input(input);

    let mult = instructions.iter()
        .copied()
        .filter_map(|f| match f {
            Instruction::Mul(l, r) => Some((l, r)),
            _ => None,
        });
    mult.map(|(l, r)| l * r).sum()
}

fn part2(input: &str) -> i64 {
    let instructions = parse_input(input);

    let (sum, _enabled) = instructions.into_iter()
        .fold((0, true), |(sum, enabled), instruction| {
            match instruction {
                Instruction::Do => (sum, true),
                Instruction::Dont => (sum, false),
                Instruction::Mul(l, r) => (sum + if enabled { l * r } else { 0 }, enabled),
            }
        });

    sum
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut remaining = input;
    
    let mut instructions = vec![];
    while !remaining.is_empty() {
        if let Some(new_remaining) = is_do(remaining) {
            instructions.push(Instruction::Do);
            remaining = new_remaining;
        } else if let Some(new_remaining) = is_dont(remaining) {
            instructions.push(Instruction::Dont);
            remaining = new_remaining;
        } else if let Some((new_remaining, (left, right))) = is_mul(remaining) {
            instructions.push(Instruction::Mul(left, right));
            remaining = new_remaining;
        } else {
            remaining = &remaining[1..];
        }
    }

    instructions
}

fn is_do(s: &str) -> Option<&str> {
    s.strip_prefix("do()")
}
fn is_dont(s: &str) -> Option<&str> {
    s.strip_prefix("don't()")
}
fn is_mul(s: &str) -> Option<(&str, (i64, i64))> {
    let s = s.strip_prefix("mul(")?;
    let paren_idx = s.chars()
        .take(8)
        .enumerate()
        .find_map(|(i, c)| (c == ')').then_some(i))?;
    let (args, remaining) = s.split_at(paren_idx);
    let (left, right) = args.split_once(',')?;
    let left = parse_arg(left)?;
    let right = parse_arg(right)?;
    Some((remaining, (left, right)))
}
fn parse_arg(s: &str) -> Option<i64> {
    if !s.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if s.len() > 3 {
        return None;
    }
    if s.is_empty() {
        return None;
    }
    s.parse().ok()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Do,
    Dont,
    Mul(i64, i64),
}
