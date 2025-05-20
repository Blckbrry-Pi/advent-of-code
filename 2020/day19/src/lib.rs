use std::str::FromStr;

aoc_tools::aoc_sol!(day19 2020: part1, part2);

#[derive(Clone, PartialEq, Eq)]
struct Concat {
    parts: Vec<u8>
}
impl Concat {
    pub fn parse<'a>(&self, s: &'a str, rules: &RuleSet) -> Vec<&'a str> {
        let mut curr_results = vec![s];
        for part in &self.parts {
            let mut new_results = Vec::new();
            for remaining in curr_results {
                for new_remaining in rules.rules.get(part).unwrap().parse(remaining, rules) {
                    new_results.push(new_remaining);
                }
            }
            curr_results = new_results;
        }
        curr_results
    }
}
impl FromStr for Concat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ')
            .map(|p| p.parse::<u8>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { parts })
    }
}
impl Debug for Concat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.parts.len() {
            if i != 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", self.parts[i])?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq)]
enum IndirectRule {
    Char(char),
    Concat(Concat),
    Or(Concat, Concat),
}
impl IndirectRule {
    pub fn parse<'a>(&self, s: &'a str, rules: &RuleSet) -> Vec<&'a str> {
        match self {
            Self::Char(c) => s.strip_prefix(*c).into_iter().collect(),
            Self::Concat(c) => c.parse(s, rules),
            Self::Or(a, b) => a.parse(s, rules).into_iter().chain(b.parse(s, rules)).collect(),
        }
    }
}
impl FromStr for IndirectRule {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.as_bytes()[0] == b'"' {
            return Ok(Self::Char(s.as_bytes()[1] as char));
        }
        if let Some((a, b)) = s.split_once(" | ") {
            Ok(Self::Or(a.parse()?, b.parse()?))
        } else {
            Ok(Self::Concat(s.parse()?))
        }
    }
}
impl Debug for IndirectRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Char(c) => write!(f, "{c:?}"),
            Self::Concat(concat) => write!(f, "{concat:?}"),
            Self::Or(a, b) => write!(f, "{a:?} | {b:?}"),
        }
    }
}

#[derive(Clone)]
struct RuleSet {
    rules: HashMap<u8, IndirectRule>,
}
impl RuleSet {
    pub fn parse<'a>(&self, s: &'a str, rule: u8) -> Vec<&'a str> {
        self.rules.get(&rule).unwrap().parse(s, self).to_vec()
    }
    pub fn update_for_part_2(&mut self) {
        // 8: 42 | 42 8
        // 11: 42 31 | 42 11 31
        self.rules.insert(8, IndirectRule::from_str("42 | 42 8").unwrap());
        self.rules.insert(11, IndirectRule::from_str("42 31 | 42 11 31").unwrap());
    }
}
impl FromStr for RuleSet {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules = s.lines().map(|l| {
            let Some((rule_num, rule)) = l.split_once(": ") else {
                return Err("Missing colon in ruleset line".to_string());
            };
            let rule_num = rule_num.parse::<u8>().map_err(|e| e.to_string())?;
            Ok((rule_num, rule.parse::<IndirectRule>()?))
        }).collect::<Result<_, _>>()?;
        Ok(Self { rules })
    }
}
impl Debug for RuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in u8::MIN..=u8::MAX {
            if let Some(rule) = self.rules.get(&i) {
                writeln!(f, "{i:3}: {rule:?}")?;
            }
        }
        Ok(())
    }
}

pub fn part1(input: &str) -> i64 {
    let (ruleset, lines) = parse_input(input);
    let mut matches = 0;
    // println!("{ruleset:?}");
    for line in lines {
        if ruleset.parse(&line, 0).contains(&"") {
            matches += 1;
        }
    }
    matches
}

pub fn part2(input: &str) -> i64 {
    let (mut ruleset, lines) = parse_input(input);
    ruleset.update_for_part_2();
    let mut matches = 0;
    for line in lines {
        if ruleset.parse(&line, 0).contains(&"") {
            matches += 1;
        }
    }
    matches
}

fn parse_input(input: &str) -> (RuleSet, Vec<String>) {
    let (ruleset, lines) = input.split_once("\n\n").unwrap();
    let lines = lines.lines().filter(|l| !l.trim().is_empty()).map(|s| s.to_string()).collect();
    (ruleset.parse().unwrap(), lines)
}
