use std::{collections::HashMap, fmt::{Debug, Display}, str::FromStr};

use crate::algebra::AlgebraicOp;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Monkey {
    Number(i64),
    Op(Op, MonkeyIdent, MonkeyIdent),
    Human,
    Eq(MonkeyIdent, MonkeyIdent),
    Algebraic(AlgebraicOp),
}
impl FromStr for Monkey {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((first, rest)) = s.split_once(" ") else {
            return Ok(Self::Number(
                s.parse::<i64>().map_err(|e| e.to_string())?,
            ));
        };
        let Some((op, second)) = rest.split_once(" ") else {
            return Err("Invalid operation monkey".to_string());
        };
        let first: MonkeyIdent = first.parse()?;
        let second: MonkeyIdent = second.parse()?;
        let op: Op = op.parse()?;
        Ok(Self::Op(op, first, second))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MonkeyIdent([u8; 4]);
impl FromStr for MonkeyIdent {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 { return Err("All monkey idents must have 4 chars".to_string()) }
        Ok(Self([
            s.as_bytes()[0],
            s.as_bytes()[1],
            s.as_bytes()[2],
            s.as_bytes()[3],
        ]))
    }
}
impl Debug for MonkeyIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op { Add, Sub, Mul, Div }
impl Op {
    pub fn apply(&self, l: i64, r: i64) -> i64 {
        match self {
            Self::Add => l + r,
            Self::Sub => l - r,
            Self::Mul => l * r,
            Self::Div => l / r,
        }
    }
}
impl FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err("All operations have 1 character".to_string());
        }
        match s.as_bytes()[0] {
            b'+' => Ok(Self::Add),
            b'-' => Ok(Self::Sub),
            b'*' => Ok(Self::Mul),
            b'/' => Ok(Self::Div),
            e => Err(format!("Invalid operation {}", e as char))
        }
    }
}
impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonkeyEntry(pub MonkeyIdent, pub Monkey);
impl FromStr for MonkeyEntry {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((ident, monkey)) = s.split_once(": ") else {
            return Err("No colon in monkey description".to_string());
        };
        let ident: MonkeyIdent = ident.parse()?;
        let monkey: Monkey = monkey.parse()?;
        Ok(Self(ident, monkey))
    }
}

#[derive(Debug)]
pub struct Riddle {
    pub monkeys: HashMap<MonkeyIdent, Monkey>,
}
impl Riddle {
    pub fn reduce_from_root(&mut self) -> AlgebraicOp {
        self.reduce(MonkeyIdent::from_str("root").unwrap()).unwrap()
    }
    fn reduce(&mut self, ident: MonkeyIdent) -> Option<AlgebraicOp> {
        let mut output = match self.monkeys.remove(&ident)? {
            Monkey::Human => AlgebraicOp::Human,
            Monkey::Number(n) => AlgebraicOp::Number(n),
            Monkey::Algebraic(a) => a,
            Monkey::Eq(l, r) => AlgebraicOp::Eq(
                Box::new(self.reduce(l)?),
                Box::new(self.reduce(r)?),
            ),
            Monkey::Op(op, l, r) => AlgebraicOp::Op(
                op,
                Box::new(self.reduce(l)?),
                Box::new(self.reduce(r)?),
            ),
        };
        output.reduce();
        Some(output)
    }

    pub fn setup_for_part_2(&mut self) {
        let root = MonkeyIdent::from_str("root").expect("root should be a valid monkey identifier");
        let Monkey::Op(_op, l, r) = self.monkeys
            .get(&root)
            .expect("This riddle should have a root")
        else {
            panic!("Roots should always be operations in the input");
        };
        self.monkeys.insert(root, Monkey::Eq(*l, *r));

        let humn = MonkeyIdent::from_str("humn").expect("humn should be a valid monkey ident");
        self.monkeys.insert(humn, Monkey::Human).expect("This riddle should have a humn");
    }
}
