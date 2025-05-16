use std::str::FromStr;

aoc_tools::aoc_sol!(day05 2020: part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ZoneFb { F, B }
impl FromStr for ZoneFb {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "F" => Ok(Self::F),
            "B" => Ok(Self::B),
            _ => Err("Invalid FB Zone".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ZoneLr { L, R }
impl FromStr for ZoneLr {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::L),
            "R" => Ok(Self::R),
            _ => Err("Invalid LR Zone".to_string()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pass([ZoneFb; 7], [ZoneLr; 3]);
impl Pass {
    pub fn row(&self) -> u8 {
        let mut row = 0;
        for fb in self.0 {
            row <<= 1;
            if fb == ZoneFb::B { row |= 1; }
        }
        row
    }
    pub fn col(&self) -> u8 {
        let mut col = 0;
        for lr in self.1 {
            col <<= 1;
            if lr == ZoneLr::R { col |= 1; }
        }
        col
    }
    pub fn id(&self) -> u16 {
        self.row() as u16 * 8 + self.col() as u16
    }
    pub const MIN: Self = Self([ZoneFb::F; 7], [ZoneLr::L; 3]);
    pub const MAX: Self = Self([ZoneFb::B; 7], [ZoneLr::R; 3]);
}
impl FromStr for Pass {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 { return Err("Invalid pass length".to_string()) }
        let mut fb = [ZoneFb::F; 7];
        let mut lr = [ZoneLr::L; 3];
        for i in 0..7 {
            fb[i] = s[i..i+1].parse()?;
        }
        for i in 0..3 {
            lr[i] = s[7+i..7+i+1].parse()?;
        }
        Ok(Self(fb, lr))
    }
}

pub fn part1(input: &str) -> u16 {
    let passes = parse_input(input);
    passes.iter().map(|p| p.id()).max().unwrap()
}

pub fn part2(input: &str) -> u16 {
    let passes = parse_input(input);
    let seen: HashSet<u16> = passes.iter().map(|p| p.id()).collect();
    for id in Pass::MIN.id()+1..=Pass::MAX.id() {
        if !seen.contains(&id) && seen.contains(&(id-1)) && seen.contains(&(id+1)) {
            return id
        }
    }
    unreachable!("Your seat was overbooked. We'll offer you a $10 voucher! Thx.")
}

fn parse_input(input: &str) -> Vec<Pass> {
    input.lines().filter(|l| !l.trim().is_empty()).map(|l| l.parse().unwrap()).collect()
}
