use std::str::FromStr;

aoc_tools::aoc_sol!(day04 2020: part1, part2);

#[derive(Debug, Clone, Default)]
struct Passport {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<u64>,
}
impl Passport {
    pub fn valid_p1(&self) -> bool {
        true
        && self.byr.is_some()
        && self.iyr.is_some()
        && self.eyr.is_some()
        && self.hgt.is_some()
        && self.hcl.is_some()
        && self.ecl.is_some()
        && self.pid.is_some()
    }
    pub fn valid_p2(&self) -> bool {
        let mostly_good = true
            && self.byr.is_some_and(|y| 1920 <= y && y <= 2002)
            && self.iyr.is_some_and(|y| 2010 <= y && y <= 2020)
            && self.eyr.is_some_and(|y| 2020 <= y && y <= 2030)
            && self.ecl.as_ref().is_some_and(|c| matches!(c.as_str(), "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"))
            && self.pid.as_ref().is_some_and(|v| v.len() == 9 && v.as_bytes().iter().all(|&c| b'0' <= c && c <= b'9'))
            && self.hgt.is_some()
            && self.hcl.is_some();
        if !mostly_good { return false }

        let hcl = self.hcl.as_ref().unwrap();
        if !hcl.starts_with('#') { return false }
        if hcl.len() != 7 { return false }
        if !hcl[1..].as_bytes().iter().all(|b| (b'0'..=b'9').contains(b) || (b'a'..=b'f').contains(b)) { return false }

        let hgt = self.hgt.as_ref().unwrap();
        let Some((n, unit)) = hgt.split_at_checked(hgt.len() - 2) else { return false };
        let Ok(n) = n.parse::<u8>() else { return false };
        match unit {
            "in" => 59 <= n && n <= 76,
            "cm" => 150 <= n && n <= 193,
            _ => false,
        }
    }
}
impl FromStr for Passport {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut outputs = Self::default();
        let errs = s.trim().split_ascii_whitespace()
            .map(|field| {
                let (field, value) = field.split_once(':')?;
                match field {
                    "byr" => outputs.byr = Some(value.parse().ok()?),
                    "iyr" => outputs.iyr = Some(value.parse().ok()?),
                    "eyr" => outputs.eyr = Some(value.parse().ok()?),
                    "hgt" => outputs.hgt = Some(value.to_string()),
                    "hcl" => outputs.hcl = Some(value.to_string()),
                    "ecl" => outputs.ecl = Some(value.to_string()),
                    "pid" => outputs.pid = Some(value.to_string()),
                    "cid" => outputs.cid = Some(value.parse().ok()?),
                    _ => return None,
                }
                Some(())
            })
            .filter(|v| v.is_none())
            .count();
        if errs > 0 {
            Err(format!("{errs} fields failed to parse"))
        } else {
            Ok(outputs)
        }
    }
}

pub fn part1(input: &str) -> usize {
    let passports = parse_input(input);
    passports.iter().filter(|p| p.valid_p1()).count()
}

pub fn part2(input: &str) -> usize {
    let passports = parse_input(input);
    passports.iter().filter(|p| p.valid_p2()).count()
}

fn parse_input(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(Passport::from_str).map(Result::unwrap).collect()
}
