use std::str::FromStr;

aoc_tools::aoc_sol!(day06 2020: part1, part2);

#[derive(Clone, Copy, PartialEq, Eq)]
struct Questions(u32);
impl Questions {
    pub fn or(&self, other: &Self) -> Self {
        Self(self.0 | other.0)
    }
    pub fn and(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }
    pub fn len(&self) -> u32 {
        self.0.count_ones()
    }
}
impl Debug for Questions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..26 {
            let c = b'a' + i;
            if self.0 & (1 << i) != 0 {
                write!(f, "{}", c as char)?;
            }
        }
        Ok(())
    }
}
impl FromStr for Questions {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut output = Self(0);
        for c in s.chars() {
            if 'a' > c || c > 'z' {
                return Err(format!("Invalid char {c:?}"));
            }
            let i = c as u8 - b'a';
            output.0 |= 1 << i;
        }
        Ok(output)
    }
}

pub fn part1(input: &str) -> u32 {
    let questions = parse_input(input);
    let mut group_counts = 0;
    for group in questions {
        group_counts += group.into_iter().reduce(|a, b| a.or(&b)).unwrap().len();
    }
    group_counts
}

pub fn part2(input: &str) -> u32 {
    let questions = parse_input(input);
    let mut group_counts = 0;
    for group in questions {
        group_counts += group.into_iter().reduce(|a, b| a.and(&b)).unwrap().len();
    }
    group_counts
}

fn parse_input(input: &str) -> Vec<Vec<Questions>> {
    input.split("\n\n")
        .map(|l| l.trim())
        .map(|l| l.lines().map(|l| l.parse().unwrap()).collect())
        .collect()
}
