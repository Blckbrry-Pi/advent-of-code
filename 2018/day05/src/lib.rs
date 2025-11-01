aoc_tools::aoc_sol!(day05 2018: part1, part2);

fn len_without(units: impl Iterator<Item = PolymerUnit>, removed: Option<u8>) -> usize {
    let filtered = units.filter(|&u| {
        let Some(removed) = removed else { return true };
        let (PolymerUnit::Lower(t) | PolymerUnit::Upper(t)) = u;
        removed != t
    });
    PolymerStack::from_units(filtered).len()
}

pub fn part1(input: &str) -> usize {
    let units = parse_input(input);
    PolymerStack::from_units(units).len()
}

pub fn part2(input: &str) -> usize {
    let units: Vec<_> = parse_input(input).collect();
    (0..26).map(|v| len_without(units.iter().copied(), Some(v))).min().unwrap()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PolymerUnit {
    Upper(u8),
    Lower(u8),
}
impl PolymerUnit {
    pub fn reacts_with(&self, other: &Self) -> bool {
        match (self, other) {
            | (Self::Upper(a), Self::Lower(b))
            | (Self::Lower(a), Self::Upper(b)) => a == b,
            _ => false,
        }
    }
}
impl Debug for PolymerUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Upper(c) => write!(f, "{}", (*c + b'A') as char),
            Self::Lower(c) => write!(f, "{}", (*c + b'a') as char),
        }
    }
}

#[derive(Clone)]
struct PolymerStack(Vec<PolymerUnit>);
impl PolymerStack {
    pub fn from_units(units: impl IntoIterator<Item = PolymerUnit>) -> Self {
        units.into_iter()
            .fold(Self(vec![]), |mut stack, unit| {
                stack.push(unit);
                stack
            })
    }
    pub fn push(&mut self, unit: PolymerUnit) {
        if let Some(top) = self.0.last() {
            if unit.reacts_with(top) {
                self.0.pop();
                return;
            }
        }
        self.0.push(unit);
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
impl Debug for PolymerStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Poly<")?;
        for &unit in &self.0 {
            write!(f, "{unit:?}")?;
        }
        write!(f, ">")
    }
}

fn parse_input(input: &str) -> impl Clone + Iterator<Item = PolymerUnit> + '_ {
    input.trim()
        .chars()
        .map(|c| {
            assert!(c.is_ascii_alphabetic());
            if c.is_ascii_uppercase() {
                PolymerUnit::Upper(c as u8 - b'A')
            } else {
                PolymerUnit::Lower(c as u8 - b'a')
            }
        })
}
