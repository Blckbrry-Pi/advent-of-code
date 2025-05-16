use std::str::FromStr;

aoc_tools::aoc_sol!(day02 2020: part1, part2);

#[derive(Clone, Copy, PartialEq, Eq)]
struct Rule {
    n1: u8,
    n2: u8,
    chr: u8,
}
impl Rule {
    fn check_p1(&self, pass: &[u8]) -> bool {
        let occurences = pass.iter().filter(|c| **c == self.chr).count() as u8;
        self.n1 <= occurences && occurences <= self.n2
    }
    fn check_p2(&self, pass: &[u8]) -> bool {
        let n1_contains = pass[self.n1 as usize - 1] == self.chr;
        let n2_contains = pass[self.n2 as usize - 1] == self.chr;
        n1_contains != n2_contains
    }
}
impl Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{} {}", self.n1, self.n2, self.chr as char)
    }
}
impl FromStr for Rule {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((nums, chr)) = s.split_once(' ') else {
            return Err("Missing space between numbers and char".to_string());
        };
        let Some((n1, n2)) = nums.split_once('-') else {
            return Err("Missing hyphen between numbers".to_string());
        };
        let n1 = n1.parse::<u8>().map_err(|e| e.to_string())?;
        let n2 = n2.parse::<u8>().map_err(|e| e.to_string())?;
        if chr.len() != 1 {
            return Err("Expected a single ASCII character after the space".to_string());
        }
        let chr = chr.as_bytes()[0];
        Ok(Self {
            n1,
            n2,
            chr,
        })
    }
}

pub fn part1(input: &str) -> u16 {
    let entries = parse_input(input);
    let mut valid_count = 0;
    for (rule, pass) in entries {
        if rule.check_p1(&pass) {
            valid_count += 1;
        }
    }
    valid_count
}

pub fn part2(input: &str) -> u16 {
    let entries = parse_input(input);
    let mut valid_count = 0;
    for (rule, pass) in entries {
        if rule.check_p2(&pass) {
            valid_count += 1;
        }
    }
    valid_count
}

fn parse_input(input: &str) -> Vec<(Rule, Box<[u8]>)> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once(": ").unwrap())
        .map(|(rule, pass)| (rule.parse().unwrap(), Box::from(pass.as_bytes())))
        .collect()
}
