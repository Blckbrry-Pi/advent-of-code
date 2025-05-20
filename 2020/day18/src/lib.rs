aoc_tools::aoc_sol!(day18 2020: part1, part2);

type Scalar = i64;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Eq {
    terms: Vec<Term>,
}
impl Eq {
    pub fn parse(s: &str) -> (Eq, &str) {
        let mut remaining = s;
        let mut terms = vec![];
        while let Some((term, new_remaining)) = Term::parse(remaining) {
            terms.push(term);
            remaining = new_remaining;
        }
        (Eq { terms }, remaining)
    }
    fn _calculate(terms: &[Term], p2: bool) -> Scalar {
        let mut curr_term = terms[0].calculate(p2);
        let mut next = 1;
        while next < terms.len() {
            match terms[next] {
                Term::Op(Operator::Mul) => if p2 {
                    // Multiply by the value of the rest (basically, bind more loosely than addition)
                    curr_term *= Self::_calculate(&terms[next+1..], p2);
                    break;
                } else {
                    curr_term *= terms[next+1].calculate(p2)
                },
                Term::Op(Operator::Add) => curr_term += terms[next+1].calculate(p2),
                _ => panic!("Bad joining term"),
            }
            next += 2;
        }
        curr_term
    }
    pub fn calculate(&self, p2: bool) -> Scalar {
        Self::_calculate(&self.terms, p2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Mul,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Term {
    Num(Scalar),
    Op(Operator),
    Parenthesized(Eq),
}
impl Term {
    pub fn calculate(&self, p2: bool) -> Scalar {
        match self {
            &Self::Num(n) => n,
            &Self::Op(op) => panic!("Operator directly calculated {op:?}"),
            Self::Parenthesized(eq) => eq.calculate(p2),
        }
    }
    pub fn parse(s: &str) -> Option<(Term, &str)> {
        match *s.as_bytes().get(0)? {
            b'0'..=b'9' => {
                let mut rest = s;
                while !rest.is_empty() && (b'0'..=b'9').contains(&rest.as_bytes()[0]) {
                    rest = &rest[1..];
                }
                let (num, s) = (&s[..s.len() - rest.len()], rest.trim_start());
                Some((Term::Num(num.parse().unwrap()), s.trim()))
            },
            b'+' => Some((Term::Op(Operator::Add), s[1..].trim_start())),
            b'*' => Some((Term::Op(Operator::Mul), s[1..].trim_start())),
            b'(' => {
                let (eq, s) = Eq::parse(&s[1..]);
                Some((Term::Parenthesized(eq), s[1..].trim_start()))
            },
            b')' => None,
            _ => panic!(":("),
        }
    }
}


pub fn part1(input: &str) -> i64 {
    let equations = parse_input(input);
    equations.iter().map(|eq| eq.calculate(false)).sum()
}

pub fn part2(input: &str) -> i64 {
    let equations = parse_input(input);
    equations.iter().map(|eq| eq.calculate(true)).sum()
}

fn parse_input(input: &str) -> Vec<Eq> {
    input.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| Eq::parse(l).0)
        .collect()
}
