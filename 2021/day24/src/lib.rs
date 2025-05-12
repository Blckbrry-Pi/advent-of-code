#![feature(box_patterns)]

use std::{rc::Rc, str::FromStr, sync::RwLock};
aoc_tools::aoc_sol!(day24 2021: part1, part2);

type BoxType<T> = Rc<T>;

#[derive(Clone)]
struct ReductionReplacements(Vec<i64>);

#[derive(Clone)]
struct Range(Rc<RwLock<Option<Rc<HashSet<i64>>>>>);
impl Range {
    pub fn empty() -> Self {
        Self(Rc::new(RwLock::new(None)))
    }
    pub fn from(range: HashSet<i64>) -> Self {
        Self(Rc::new(RwLock::new(Some(Rc::new(range)))))
    }
    pub fn get_only(&self) -> Option<i64> {
        let guard = self.0.read().unwrap();
        guard.as_ref().and_then(|map| if map.len() == 1 {
            map.iter().next().copied()
        } else {
            None
        })
    }
    pub fn combine(a: &HashSet<i64>, b: &HashSet<i64>, combiner: impl Fn(i64, i64) -> i64) -> HashSet<i64> {
        let combiner_ref = &combiner;
        a.iter()
            .copied()
            .flat_map(
                |a_val| b.iter()
                    .copied()
                    .map(move |b_val| combiner_ref(a_val, b_val))
            )
            .collect()
    }
    pub fn get<'a>(&'a self) -> Option<Rc<HashSet<i64>>> {
        let guard = self.0.read().unwrap();
        guard.clone()
    }
}

#[derive(Clone)]
enum Computed {
    Binary(BoxType<BinaryOp>, Range),
    Literal(i64),
    Input(u8),
}
impl Computed {
    fn reduce(self: Self) -> (Self, bool) {
        let Self::Binary(binary_op, range) = self else { return (self, false) };
        if range.get().is_some() { return (Self::Binary(binary_op, range), false); }
        let (reduced, was_reduced) = binary_op.reduce();
        let Self::Binary(_, range) = &reduced else { return (reduced, was_reduced) };
        if let Some(l) = range.get_only() {
            (Self::Literal(l), true)
        } else {
            (reduced, was_reduced)
        }
    }
    fn reduce_2(l: Self, r: Self) -> (Self, Self, bool) {
        let (l, l_reduced) = if !l.has_range() { l.reduce() } else { (l, false) };
        let (r, r_reduced) = if !r.has_range() { r.reduce() } else { (r, false) };
        (l, r, l_reduced || r_reduced)
    }
    fn reduce_full(mut self) -> Self {
        loop {
            let (new, reduced) = self.reduce();
            if !reduced { return new }
            self = new;
        }
    }
    fn range(&self) -> Rc<HashSet<i64>> {
        match self {
            Self::Input(_) => (1..=9).collect::<HashSet<_>>().into(),
            Self::Literal(l) => [*l].into_iter().collect::<HashSet<_>>().into(),
            Self::Binary(_, range) => range.get().unwrap(),
        }
    }
    fn has_range(&self) -> bool {
        match self {
            Self::Binary(_, range) => range.get().is_some(),
            _ => true,
        }
    }
    fn clone_unfrozen(&self, replacements: &ReductionReplacements) -> (Self, bool) {
        match self {
            Self::Binary(op, _) => {
                let (unfrozen_op, needed_unfreezing) = op.clone_unfrozen(replacements);
                if needed_unfreezing {
                    (Self::Binary(BoxType::new(unfrozen_op), Range::empty()), true)
                } else {
                    (self.clone(), false)
                }
            },
            &Self::Input(i) if i < replacements.0.len() as u8 => (Self::Literal(replacements.0[i as usize]), true),
            _ => (self.clone(), false),
        }
    }
}
impl Debug for Computed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input(n) => write!(f, "<inp {n:2}>"),
            Self::Literal(l) => write!(f, "{l}"),
            Self::Binary(b, _) => write!(f, "{b:?}"),
        }
    }
}
// impl Hash for Computed {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         match self {
//             Self::Binary(b) => {
//                 state.write_i8(0);
//                 b.as_ref().hash()
//             }
//         }
//     }
// }

#[derive(Clone)]
enum BinaryOp {
    Add(Computed, Computed),
    Mul(Computed, Computed),
    Div(Computed, Computed),
    Mod(Computed, Computed),
    Eql(Computed, Computed),
    Neq(Computed, Computed),
}
impl BinaryOp {
    fn sub_reduce(self: BoxType<Self>) -> (BoxType<Self>, bool) {
        match self.as_ref() {
            Self::Add(l, r) => {
                let (new_l, new_r, reduced) = Computed::reduce_2(l.clone(), r.clone());
                (BoxType::new(Self::Add(new_l, new_r)), reduced)
            },
            Self::Mul(l, r) => {
                let (new_l, new_r, reduced) = Computed::reduce_2(l.clone(), r.clone());
                (BoxType::new(Self::Mul(new_l, new_r)), reduced)
            },
            Self::Div(l, r) => {
                let (new_l, new_r, reduced) = Computed::reduce_2(l.clone(), r.clone());
                (BoxType::new(Self::Div(new_l, new_r)), reduced)
            },
            Self::Mod(l, r) => {
                let (new_l, new_r, reduced) = Computed::reduce_2(l.clone(), r.clone());
                (BoxType::new(Self::Mod(new_l, new_r)), reduced)
            },
            Self::Eql(l, r) => {
                let (new_l, new_r, reduced) = Computed::reduce_2(l.clone(), r.clone());
                (BoxType::new(Self::Eql(new_l, new_r)), reduced)
            },
            Self::Neq(l, r) => {
                let (new_l, new_r, reduced) = Computed::reduce_2(l.clone(), r.clone());
                (BoxType::new(Self::Neq(new_l, new_r)), reduced)
            },
        }
    }
    fn computed(self) -> Computed {
        Computed::Binary(BoxType::new(self), Range::empty())
    }
    fn computed_reduced(self) -> Computed {
        let range = self.range();
        Computed::Binary(BoxType::new(self), Range::from(range))
    }
    fn computed_maybe(self, is_reduced: bool) -> Computed {
        if is_reduced {
            self.computed_reduced()
        } else {
            self.computed()
        }
    }
    fn range(&self) -> HashSet<i64> {
        match self {
            Self::Add(l, r) => Range::combine(&l.range(), &r.range(), |l, r| l + r),
            Self::Mul(l, r) => Range::combine(&l.range(), &r.range(), |l, r| l * r),
            Self::Div(l, r) => Range::combine(&l.range(), &r.range(), |l, r| (l as f64 / r as f64).trunc() as i64),
            Self::Mod(l, r) => Range::combine(&l.range(), &r.range(), |l, r| l - (l as f64 / r as f64).trunc() as i64 * r),
            v @ (Self::Eql(l, r) | Self::Neq(l, r)) => {
                let is_neq = matches!(v, Self::Neq(..));
                let l_range = l.range();
                let r_range = r.range();
                let mut output_range = HashSet::new();
                if !l_range.is_disjoint(&r_range) || l_range.is_empty() || r_range.is_empty() {
                    output_range.insert(if is_neq { 0 } else { 1 });
                }
                if l_range.len() > 1 || r_range.len() > 1 || l_range.is_empty() || r_range.is_empty() {
                    output_range.insert(if is_neq { 1 } else { 0 });
                }
                output_range
            },
        }
    }
    fn clone_unfrozen(&self, replacements: &ReductionReplacements) -> (Self, bool) {
        fn collate(
            l: (Computed, bool),
            r: (Computed, bool),
            combiner: impl Fn(Computed, Computed) -> BinaryOp,
        ) -> (BinaryOp, bool) {
            (combiner(l.0, r.0), l.1 || r.1)
        }
        match self {
            Self::Add(l, r) => collate(l.clone_unfrozen(replacements), r.clone_unfrozen(replacements), Self::Add),
            Self::Mul(l, r) => collate(l.clone_unfrozen(replacements), r.clone_unfrozen(replacements), Self::Mul),
            Self::Div(l, r) => collate(l.clone_unfrozen(replacements), r.clone_unfrozen(replacements), Self::Div),
            Self::Mod(l, r) => collate(l.clone_unfrozen(replacements), r.clone_unfrozen(replacements), Self::Mod),
            Self::Eql(l, r) => collate(l.clone_unfrozen(replacements), r.clone_unfrozen(replacements), Self::Eql),
            Self::Neq(l, r) => collate(l.clone_unfrozen(replacements), r.clone_unfrozen(replacements), Self::Neq),
        }
    }
    fn reduce(mut self: BoxType<Self>) -> (Computed, bool) {
        use Computed::*;
        use BinaryOp::*;
        let (sub_reduced, sub_reduction_happened) = self.sub_reduce();
        self = sub_reduced;
        let (output, base_reducuction_happened) = match self.as_ref() {
            // Add zero
            Add(Literal(0), w) | Add(w, Literal(0)) => (
                w.clone(),
                true,
            ),
            // Reducing literal plus literal
            Add(Literal(l), Literal(r)) => (
                Literal(l + r),
                true,
            ),
            // Shunt literals to the right
            Add(l @ Literal(_), r) => (
                Add(r.clone(), l.clone()).computed(),
                true,
            ),
            // Right-fold additions
            Add(Binary(l, _), r) if matches!(l.as_ref(), Add(_, _)) => {
                let Add(l_l, l_r) = l.as_ref() else { unreachable!() };
                (
                    Add(
                        l_l.clone(),
                        Add(
                            l_r.clone(),
                            r.clone(),
                        ).computed(),
                    ).computed(),
                    true,
                )
            },
            
            // Reducing times 0 and times 1
            Mul(Literal(0), _) | Mul(_, Literal(0)) => (
                Literal(0),
                true,
            ),
            Mul(Literal(1), w) | Mul(w, Literal(1)) => (
                w.clone(),
                true,
            ),
            // Reducing literal times literal
            Mul(Literal(l), Literal(r)) => (
                Literal(l * r),
                true,
            ),
            // Shunt literals to the right
            // Mul(l @ Literal(_), r) => (
            Mul(l @ Literal(_), r) => (
                Mul(r.clone(), l.clone()).computed(),
                true,
            ),
            // Right-fold multiplies
            Mul(Binary(l, _), r) if matches!(l.as_ref(), Mul(_, _)) => {
                let Mul(l_l, l_r) = l.as_ref() else { unreachable!() };
                (
                    Mul(
                        l_l.clone(),
                        Mul(
                            l_r.clone(),
                            r.clone(),
                        ).computed(),
                    ).computed(),
                    true,
                )
            }
            // Distribute
            // | Mul(Binary(add, _), coeff)
            // | Mul(coeff, Binary(add, _))
            // if matches!(add.as_ref(), Add(_, _)) => {
            //     let Add(add_l, add_r) = add.as_ref() else { unreachable!() };
            //     (
            //         Add(
            //             Mul(add_l.clone(), coeff.clone()).computed(),
            //             Mul(add_r.clone(), coeff.clone()).computed(),
            //         ).computed(),
            //         true,
            //     )
            // },

            // 0 divided by anything is 0
            Div(Literal(0), _) => (
                Literal(0),
                true,
            ),
            // anything divided by 1 is itself
            Div(l, Literal(1)) => (
                l.clone(),
                true,
            ),
            Div(Literal(l), Literal(r)) => (
                Literal((*l as f64 / *r as f64).trunc() as i64),
                true,
            ),
            // Remove immediately nested divisions
            Div(Binary(l, _), r) if matches!(l.as_ref(), Div(_, _)) => {
                let Div(l_l, l_r) = l.as_ref() else { unreachable!() };
                (
                    Div(
                        l_l.clone(),
                        Mul(l_r.clone(), r.clone()).computed(),
                    ).computed(),
                    true,
                )
            },
            Div(l, Binary(r, _)) if matches!(r.as_ref(), Div(_, _)) => {
                let Div(r_l, r_r) = r.as_ref() else { unreachable!() };
                (
                    Div(
                        Mul(l.clone(), r_r.clone()).computed(),
                        r_l.clone(),
                    ).computed(),
                    true,
                )
            },

            Mod(_, Literal(1)) => (
                Literal(0),
                true,
            ),
            Mod(Literal(l), Literal(r)) => (
                Literal(l - (*l as f64 / *r as f64).trunc() as i64 * r),
                true,
            ),
            // Mod(Binary(mod_l, _), Literal(r)) if matches!(mod_l.as_ref(), Mod(_, Literal(_))) => {
            //     let Mod(l_l, Literal(l_r)) = mod_l.as_ref() else { unreachable!() };
            //     let (l_r, r) = (*l_r, *r);
            //     if l_r % r == 0 || r % l_r == 0 {
            //         (
            //             Mod(l_l.clone(), Literal(l_r.min(r))).computed(),
            //             true,
            //         )
            //     } else {
            //         // (
            //         //     Mod(
            //         //         Mod(l_l.clone(), Literal(l_r)).computed(),
            //         //         Literal(r),
            //         //     ).computed_reduced(),
            //         //     false,
            //         // )
            //         panic!("a");
            //     }
            // },
            // Mod(Binary(binop, range), Literal(r_val)) if range.get().is_some() => {
            //     if range.get().unwrap().iter().all(|v| (0..*r_val).contains(v)) {
            //         (Binary(binop.clone(), range.clone()), true)
            //     } else {
            //         ((*self).clone().computed_maybe(!sub_reduction_happened), false)
            //     }
            // }
            // Mod(Binary(box Mod(l_l, Literal(l_r)), _), Literal(r)) => {
            //     if l_r % r == 0 || r % l_r == 0 {
            //         (
            //             Mod(l_l, Literal(l_r.min(r))).computed(),
            //             true,
            //         )
            //     } else {
            //         (
            //             Mod(
            //                 Mod(l_l, Literal(l_r)).computed(),
            //                 Literal(r),
            //             ).computed_reduced(),
            //             false,
            //         )
            //     }
            // },


            // Reducing literal eql literal
            Eql(Literal(l), Literal(r)) => (
                Literal(if l == r { 1 } else { 0 }),
                true,
            ),

            // Reducing neq
            | Eql(Binary(cmp, _), Literal(0))
            | Eql(Literal(0), Binary(cmp, _))
            if matches!(cmp.as_ref(), Eql(..)) => {
                let Eql(cmp_l, cmp_r) = cmp.as_ref() else { unreachable!() };
                (Neq(cmp_l.clone(), cmp_r.clone()).computed(), true)
            },

            // Sub-Reductions
            // c @ Self::Add(_, _) => (c.computed_reduced(), false),
            // c @ Self::Mul(_, _) => (c.computed_reduced(), false),
            // c @ Self::Div(_, _) => (c.computed_reduced(), false),
            // c @ Self::Mod(_, _) => (c.computed_reduced(), false),
            // c @ Self::Eql(_, _) => (c.computed_reduced(), false),
            _ => ((*self).clone().computed_maybe(!sub_reduction_happened), false),
        };
        let reduction_happened = base_reducuction_happened || sub_reduction_happened;
        (output, reduction_happened)
        // if reduction_happened {
        // } else {
        //     return (output, false);
        // }
    }
}
impl Debug for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Self::Input(n) => write!(f, "<inp {n:2}>"),
            // Self::Literal(l) => write!(f, "{l}"),
            Self::Add(a, b) => write!(f, "({a:?} + {b:?})"),
            Self::Mul(a, b) => write!(f, "({a:?} * {b:?})"),
            Self::Div(a, b) => write!(f, "({a:?} / {b:?})"),
            Self::Mod(a, b) => write!(f, "({a:?} % {b:?})"),
            Self::Eql(a, b) => write!(f, "({a:?} == {b:?})"),
            Self::Neq(a, b) => write!(f, "({a:?} != {b:?})"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reg { W, X, Y, Z }
impl FromStr for Reg {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err("All registers are 1 char long".to_string())
        }
        let res = match s {
            "w" => Self::W,
            "x" => Self::X,
            "y" => Self::Y,
            "z" => Self::Z,
            c => return Err(format!("Invalid register {c:?}"))
        };
        Ok(res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op { Reg(Reg), Literal(i64) }
impl FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = if s.as_bytes()[0].is_ascii_alphabetic() {
            Self::Reg(Reg::from_str(s)?)
        } else {
            Self::Literal(i64::from_str(s).map_err(|e| e.to_string())?)
        };
        Ok(res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Inp(Reg),
    Add(Reg, Op),
    Mul(Reg, Op),
    Div(Reg, Op),
    Mod(Reg, Op),
    Eql(Reg, Op),
}
impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 5 {
            return Err("All instructions are at least 5 chars long".to_string());
        }
        let inst = match s.get(0..3) {
            Some(instruction_type) => instruction_type,
            None => return Err("All instructions must be ASCII".to_string()),
        };
        let reg = match s.get(4..5) {
            Some(register) => register.parse(),
            None => Err("All instructions must be ASCII".to_string()),
        }?;
        let op = s.get(6..).map(Op::from_str).transpose()?;
        match inst {
            "inp" => if let Some(op) = op {
                Err(format!("Unexpected operand {op:?}"))
            } else {
                Ok(Self::Inp(reg))
            },
            "add" => if let Some(op) = op {
                Ok(Self::Add(reg, op))
            } else { Err("Missing 2nd operand".to_string()) },
            "mul" => if let Some(op) = op {
                Ok(Self::Mul(reg, op))
            } else { Err("Missing 2nd operand".to_string()) },
            "div" => if let Some(op) = op {
                Ok(Self::Div(reg, op))
            } else { Err("Missing 2nd operand".to_string()) },
            "mod" => if let Some(op) = op {
                Ok(Self::Mod(reg, op))
            } else { Err("Missing 2nd operand".to_string()) },
            "eql" => if let Some(op) = op {
                Ok(Self::Eql(reg, op))
            } else { Err("Missing 2nd operand".to_string()) },
            code => Err(format!("Invalid opcode {code:?}")),
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    w: Computed,
    x: Computed,
    y: Computed,
    z: Computed,
    inputs_received: u8,
}
impl State {
    pub fn get_reg(&self, reg: Reg) -> Computed {
        match reg {
            Reg::W => self.w.clone(),
            Reg::X => self.x.clone(),
            Reg::Y => self.y.clone(),
            Reg::Z => self.z.clone(),
        }
    }
    pub fn set_reg(&mut self, reg: Reg, val: Computed) {
        match reg {
            Reg::W => self.w = val,
            Reg::X => self.x = val,
            Reg::Y => self.y = val,
            Reg::Z => self.z = val,
        }
    }
    pub fn get_op(&self, op: Op) -> Computed {
        match op {
            Op::Reg(reg) => self.get_reg(reg),
            Op::Literal(l) => Computed::Literal(l),
        }
    }
    pub fn exec_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Inp(a) => {
                let input_num = self.inputs_received;
                self.inputs_received += 1;
                self.set_reg(a, Computed::Input(input_num));
            },
            Instruction::Add(a, b) => {
                let a_val = self.get_reg(a);
                let b_val = self.get_op(b);
                let new_val = BinaryOp::Add(a_val, b_val).computed();
                self.set_reg(a, new_val);
            }
            Instruction::Mul(a, b) => {
                let a_val = self.get_reg(a);
                let b_val = self.get_op(b);
                let new_val = BinaryOp::Mul(a_val, b_val).computed();
                self.set_reg(a, new_val);
            }
            Instruction::Div(a, b) => {
                let a_val = self.get_reg(a);
                let b_val = self.get_op(b);
                let new_val = BinaryOp::Div(a_val, b_val).computed();
                self.set_reg(a, new_val);
            }
            Instruction::Mod(a, b) => {
                let a_val = self.get_reg(a);
                let b_val = self.get_op(b);
                let new_val = BinaryOp::Mod(a_val, b_val).computed();
                self.set_reg(a, new_val);
            }
            Instruction::Eql(a, b) => {
                let a_val = self.get_reg(a);
                let b_val = self.get_op(b);
                let new_val = BinaryOp::Eql(a_val, b_val).computed();
                self.set_reg(a, new_val);
            }
        }
    }
    pub fn reduce(&mut self) {
        self.w = self.w.clone().reduce_full();
        self.x = self.x.clone().reduce_full();
        self.y = self.y.clone().reduce_full();
        self.z = self.z.clone().reduce_full();
    }
    pub fn init() -> Self {
        Self {
            w: Computed::Literal(0),
            x: Computed::Literal(0),
            y: Computed::Literal(0),
            z: Computed::Literal(0),
            inputs_received: 0,
        }
    }
}

fn input_replacements(
    base: &Computed,
    input_num: u8,
    replacements: &mut ReductionReplacements,
    guesses: &(impl Iterator<Item = i64> + Clone),
) -> Option<Vec<i64>> {
        if input_num == 14 { return Some(Vec::with_capacity(14)); }
        for guess in guesses.clone() {
            // println!("Testing inp {input_num} = {guess}");
            // let mut test_replacements = replacements.clone();
            replacements.0.push(guess);
            let test_base = base.clone_unfrozen(&replacements).0.reduce_full();
            // println!("{:?}", z.range());
            if test_base.range().contains(&0) {
                for &n in replacements.0.iter() {
                    print!("{n} ");
                }
                println!();
                // println!("Input {input_num:2} ?= {guess}");
                if let Some(mut output) = input_replacements(
                    &test_base,
                    input_num+1,
                    replacements,
                    guesses,
                ) {
                    println!("Input {input_num:2}  = {guess}");
                    output.push(guess);
                    return Some(output)
                }
            }
            replacements.0.pop();
        }
        None
    }

pub fn part1(input: &str) -> i64 {
    let instructions = parse_input(input);
    let mut state = State::init();
    for instruction in instructions {
        state.exec_instruction(instruction);
        state.reduce();
    }
    println!("{state:?}");
    let solution = input_replacements(
        &state.z,
        0,
        &mut ReductionReplacements(vec![]),
        &(1..=9).rev(),
    );
    solution.unwrap().into_iter().rev().fold(0, |acc, n| acc * 10 + n)
}

pub fn part2(input: &str) -> i64 {
    let instructions = parse_input(input);
    let mut state = State::init();
    for instruction in instructions {
        state.exec_instruction(instruction);
        state.reduce();
    }
    let solution = input_replacements(
        &state.z,
        0,
        &mut ReductionReplacements(vec![]),
        &(1..=9),
    );
    solution.unwrap().into_iter().rev().fold(0, |acc, n| acc * 10 + n)
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect()
}
