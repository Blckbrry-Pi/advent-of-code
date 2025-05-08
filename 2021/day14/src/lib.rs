use std::str::FromStr;

aoc_tools::aoc_sol!(day14 2021: part1, part2);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Element(u8);
impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}
impl FromStr for Element {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(format!("Invalid string length: {}", s.len()));
        }
        Ok(Element(s.as_bytes()[0]))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pair(Element, Element);
impl Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
struct Replacements(HashMap<Pair, Element>);
impl FromStr for Replacements {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|line| {
                let Some((pair, inserted)) = line.split_once(" -> ") else {
                    return Err(format!("Couldn't find `->` in input"));
                };
                let (a, b) = (Element::from_str(&pair[..1])?, Element::from_str(&pair[1..])?);
                let pair = Pair(a, b);
                let inserted = Element::from_str(inserted)?;
                Ok((pair, inserted))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        Ok(Self(map))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PolymerStats(HashMap<Pair, usize>, Element);
impl PolymerStats {
    pub fn do_step(self, replacements: &Replacements) -> Self {
        let mut new_stats = HashMap::with_capacity(self.0.len() * 2);
        for (pair, count) in self.0 {
            if let Some(&insert) = replacements.0.get(&pair) {
                let pair_l = Pair(pair.0, insert);
                let pair_r = Pair(insert, pair.1);
                *new_stats.entry(pair_l).or_default() += count;
                *new_stats.entry(pair_r).or_default() += count;
            } else {
                *new_stats.entry(pair).or_default() += count;
            }
        }
        Self(new_stats, self.1)
    }
    pub fn counts(&self) -> HashMap<Element, usize> {
        let mut output = HashMap::new();
        self.0.iter()
            .for_each(|(pair, count)| *output.entry(pair.0).or_default() += count);
        *output.entry(self.1).or_default() += 1;
        output
    }
    pub fn min_count(&self) -> usize {
        self.counts().into_values().min().unwrap()
    }
    pub fn max_count(&self) -> usize {
        self.counts().into_values().max().unwrap()
    }
}
impl FromStr for PolymerStats {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut prev = None;
        let mut counts = HashMap::new();
        for i in 0..s.len() {
            let new = Element::from_str(&s[i..i+1])?;
            if let Some(prev) = prev {
                *counts.entry(Pair(prev, new)).or_default() += 1;
            }
            prev = Some(new);
        }
        Ok(Self(counts, prev.unwrap()))
    }
}

pub fn part1(input: &str) -> usize {
    let (mut polymer, replacements) = parse_input(input);
    for _ in 0..10 { polymer = polymer.do_step(&replacements); }
    polymer.max_count() - polymer.min_count()
}

pub fn part2(input: &str) -> usize {
    let (mut polymer, replacements) = parse_input(input);
    for _ in 0..40 { polymer = polymer.do_step(&replacements); }
    polymer.max_count() - polymer.min_count()
}

fn parse_input(input: &str) -> (PolymerStats, Replacements) {
    let (polymer, replacements) = input.split_once("\n\n").unwrap();
    (polymer.parse().unwrap(), replacements.parse().unwrap())
}
