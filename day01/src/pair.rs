use std::{fmt::Debug, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pair {
    l: i32,
    r: i32,
}

impl Pair {
    pub fn unzip_lists(pairs: &[Pair]) -> (Vec<i32>, Vec<i32>) {
        let left = pairs.iter().map(|Pair { l, .. }| *l).collect();
        let right = pairs.iter().map(|Pair { r, .. }| *r).collect();

        (left, right)
    }
}

impl Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.l, self.r)
    }
}

impl FromStr for Pair {
    type Err = <i32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<i32> = s.split(' ')
            .filter(|p| !p.is_empty())
            .map(|p| p.parse::<i32>())
            .collect::<Result<_, _>>()?;

        Ok(Pair {
            l: parsed[0],
            r: parsed[1],
        })
    }
}
