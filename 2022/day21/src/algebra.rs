use std::fmt::Debug;
use super::monkeys::Op;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AlgebraicOp {
    Number(i64),
    Human,
    Op(Op, Box<AlgebraicOp>, Box<AlgebraicOp>),
    Eq(Box<AlgebraicOp>, Box<AlgebraicOp>),
}
impl AlgebraicOp {
    pub fn reduce(&mut self) {
        self.reduce_get_num();
        let value = std::mem::replace(self, AlgebraicOp::Human);
        let (op, l, r) = match value {
            // Not actually given, but we know that 
            Self::Eq(l, r) => {
                let (output, was_reduced) = Self::reduce_eq(l, r);
                *self = output;
                if was_reduced {
                    self.reduce();
                }
                return;
            },
            Self::Op(op, l, r) => (op, l, r),
            value => {
                *self = value;
                return;
            },
        };
        match (op, l, r) {
            (
                Op::Mul,
                box AlgebraicOp::Op(
                    op @ (Op::Add | Op::Sub),
                    d_l,
                    d_r
                ),
                over,
            ) | (
                Op::Mul,
                over,
                box AlgebraicOp::Op(
                    op @ (Op::Add | Op::Sub),
                    d_l,
                    d_r
                ),
            ) => {
                *self = Self::Op(
                    op,
                    Box::new(Self::Op(
                        Op::Mul,
                        d_l.clone(),
                        over.clone(),
                    )),
                    Box::new(Self::Op(
                        Op::Mul,
                        d_r.clone(),
                        over,
                    )),
                );
                self.reduce();
            },
            // Base case reduction
            (op, mut l, mut r) => {
                l.reduce();
                r.reduce();
                *self = Self::Op(op, l, r);
            },
        }
    }
    pub fn reduce_get_num(&mut self) -> Option<i64> {
        match self {
            Self::Number(n) => Some(*n),
            Self::Op(op, l, r) => {
                let (Some(l), Some(r)) = (l.reduce_get_num(), r.reduce_get_num()) else {
                    return None;
                };
                let output = op.apply(l, r);
                *self = Self::Number(output);
                Some(output)
            },
            Self::Human => None,
            Self::Eq(l, r) => {
                l.reduce_get_num();
                r.reduce_get_num();
                None
            }
        }
    }
    fn reduce_eq(l: Box<AlgebraicOp>, r: Box<AlgebraicOp>) -> (AlgebraicOp, bool) {
        let (op, mut l_l, mut l_r, r) = match (l, r) {
            (box AlgebraicOp::Op(op, l_l, l_r), r) => (op, l_l, l_r, r),
            (l, r) => return (AlgebraicOp::Eq(l, r), false),
        };
        if let Some(l_l) = l_l.reduce_get_num() {
            let l_l = Box::new(AlgebraicOp::Number(l_l));
            let r = match op {
                // l_l + l_r = r  --> l_r = r - l_l
                Op::Add => AlgebraicOp::Op(Op::Sub, r, l_l),
                // l_l - l_r = r  --> l_r = l_l - r
                Op::Sub => AlgebraicOp::Op(Op::Sub, l_l, r),
                // l_l * l_r = r  --> l_r = r / l_l
                Op::Mul => AlgebraicOp::Op(Op::Div, r, l_l),
                // l_l / l_r = r  --> l_r = l_l / r
                Op::Div => AlgebraicOp::Op(Op::Div, l_l, r),
            };
            (AlgebraicOp::Eq(l_r.clone(), Box::new(r)), true)
        } else if let Some(l_r) = l_r.reduce_get_num() {
            let l_r = Box::new(AlgebraicOp::Number(l_r));
            let r = match op {
                // l_l + l_r = r  --> l_l = r - l_r
                Op::Add => AlgebraicOp::Op(Op::Sub, r, l_r),
                // l_l - l_r = r  --> l_l = r + l_r
                Op::Sub => AlgebraicOp::Op(Op::Add, r, l_r),
                // l_l * l_r = r  --> l_l = r / l_r
                Op::Mul => AlgebraicOp::Op(Op::Div, r, l_r),
                // l_l / l_r = r  --> l_l = r * l_r
                Op::Div => AlgebraicOp::Op(Op::Mul, r, l_r),
            };
            (AlgebraicOp::Eq(l_l.clone(), Box::new(r)), true)
        } else {
            todo!("This is too complicated help");
        }
    }

    pub fn as_part1_sol(&self) -> i64 {
        let Self::Number(n) = self else { panic!("Invalid reduced result for part 1"); };
        *n
    }

    pub fn as_part2_sol(&self) -> i64 {
        let Self::Eq(box Self::Human, box Self::Number(n)) = self else { panic!("Invalid reduced result for part 2"); };
        *n
    }
}
impl Debug for AlgebraicOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Human => write!(f, "humn"),
            Self::Number(n) => write!(f, "{n}"),
            Self::Op(op, l, r) => write!(f, "({l:?}) {op} ({r:?})"),
            Self::Eq(l, r) => write!(f, "{l:?} = {r:?}"),
        }
    }
}
