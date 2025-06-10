use std::str::FromStr;

aoc_tools::aoc_sol!(day22 2019: part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShuffleTechnique {
    NewStack,
    CutN(i64),
    DealWithInc(i64),
}
impl ShuffleTechnique {
    pub fn as_remappings(&self) -> impl Iterator<Item = Remapping> {
        let (a, b) = match self {
            Self::NewStack => (Remapping::Mul(-1), Some(Remapping::Add(-1))),
            Self::CutN(n) => (Remapping::Add(-*n), None),
            Self::DealWithInc(n) => (Remapping::Mul(*n), None),
        };
        [a].into_iter().chain(b)
    }
    pub fn many_as_remappings(techniques: &[Self], modulus: i64) -> Vec<Remapping> {
        let mut remappings = vec![];
        for technique in techniques {
            remappings.extend(technique.as_remappings());
        }
        Remapping::combine_many(&mut remappings, modulus);
        remappings
    }
}
impl FromStr for ShuffleTechnique {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "deal into new stack" {
            Ok(Self::NewStack)
        } else if let Some(n) = s.strip_prefix("deal with increment ") {
            let n = n.parse::<i64>().map_err(|e| e.to_string())?;
            Ok(Self::DealWithInc(n))
        } else if let Some(n) = s.strip_prefix("cut ") {
            let n = n.parse::<i64>().map_err(|e| e.to_string())?;
            Ok(Self::CutN(n))
        } else {
            Err("Invalid shuffle technique".to_string())
        }
    }
}

fn mul_mod(a: i64, b: i64, modulus: i64) -> i64 {
    (a as i128 * b as i128).rem_euclid(modulus as i128) as i64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Remapping {
    Add(i64),
    Mul(i64),
}
impl Remapping {
    pub fn repeat_n(remappings: &[Remapping], n: i64, modulus: i64) -> Vec<Remapping> {
        if n == 1 { remappings.to_vec() }
        else if n == 0 { vec![] }
        else {
            let mut output = Remapping::repeat_n(remappings, n / 2, modulus);
            output.extend_from_within(..);
            if n & 1 == 1 {
                output.extend_from_slice(remappings);
            }
            Self::combine_many(&mut output, modulus);
            output
        }
    }
    pub fn combine_many(remappings: &mut Vec<Remapping>, modulus: i64) {
        while remappings.len() > 3 {
            let a = remappings[0];
            let b = remappings[1];
            let combined = a.combine(&b, modulus);
            remappings[0] = combined.0;
            if let Some(second) = combined.1 {
                remappings[1] = second;
            } else {
                remappings.remove(1);
                continue
            }

            let a = remappings[1];
            let b = remappings[2];
            let combined = a.combine(&b, modulus);
            remappings[1] = combined.0;
            if let Some(second) = combined.1 {
                remappings[2] = second;
            } else {
                remappings.remove(2);
            }
        }
    }
    pub fn combine(&self, other: &Self, modulus: i64) -> (Self, Option<Self>) {
        match (self, other) {
            (Self::Add(a), Self::Add(b)) => (Self::Add((a + b).rem_euclid(modulus)), None),
            (Self::Mul(a), Self::Mul(b)) => (Self::Mul(mul_mod(*a, *b, modulus)), None),
            (Self::Add(a), Self::Mul(b)) => (Self::Mul(*b), Some(Self::Add(mul_mod(*a, *b, modulus)))),
            (Self::Mul(a), Self::Add(b)) => (Self::Mul(*a), Some(Self::Add(*b))),
        }
    }
    pub fn apply(&self, n: i64, modulus: i64) -> i64 {
        match self {
            Self::Add(a) => (n + a).rem_euclid(modulus),
            Self::Mul(m) => mul_mod(n, *m, modulus),
        }
    }
    pub fn apply_rev(&self, n: i64, modulus: i64) -> i64 {
        fn modular_inverse(a: i64, modulus: i64) -> i64 {
            let mut t = 0;
            let mut new_t = 1;
            let mut r = modulus;
            let mut new_r = a;
            while new_r != 0 {
                let quotient = r / new_r;
                (t, new_t) = (new_t, t - quotient * new_t);
                (r, new_r) = (new_r, r - quotient * new_r);
            }
            if r > 1 { panic!("a is not invertible") }

            t.rem_euclid(modulus)
        }
        assert_eq!(1, 2019 * modular_inverse(2019, modulus) % modulus);
        match self {
            Self::Add(a) => (n - a).rem_euclid(modulus),
            Self::Mul(m) => mul_mod(n, modular_inverse(*m, modulus), modulus),
        }
    }
    pub fn apply_many(remappings: &[Remapping], n: i64, modulus: i64) -> i64 {
        remappings
            .iter()
            .fold(n, |c, remap| remap.apply(c, modulus))
    }
    pub fn apply_many_rev(remappings: &[Remapping], n: i64, modulus: i64) -> i64 {
        remappings
            .iter()
            .rev()
            .fold(n, |c, remap| remap.apply_rev(c, modulus))
    }
}

pub fn part1(input: &str) -> i64 {
    const CARDS: i64 = 10007;
    let techniques = parse_input(input);
    let remappings = ShuffleTechnique::many_as_remappings(&techniques, CARDS);
    Remapping::apply_many(&remappings, 2019, CARDS)
}

pub fn part2(input: &str) -> i64 {
    const CARDS: i64 = 119315717514047;
    let techniques = parse_input(input);
    let remappings = ShuffleTechnique::many_as_remappings(&techniques, CARDS);
    let remappings = Remapping::repeat_n(&remappings, 101741582076661, CARDS);
    Remapping::apply_many_rev(&remappings, 2020, CARDS)
}

fn parse_input(input: &str) -> Vec<ShuffleTechnique> {
    input.lines().filter(|l| !l.is_empty()).map(|l| l.parse().unwrap()).collect()
}
