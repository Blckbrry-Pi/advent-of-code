use std::{cell::LazyCell, hash::Hash, str::FromStr, sync::{atomic::AtomicBool, Arc}};
aoc_tools::aoc_sol!(day24 2021: part1, part2);
aoc_tools::fast_hash!();
aoc_tools::arena!();

type Scalar = i32;
type BoxType<T> = Arc<T>;

#[derive(Clone)]
struct ReductionReplacements(Vec<Scalar>);

#[derive(Debug, Clone, PartialEq)]
struct Range(Option<Arc<HashSet<Scalar>>>);
impl Range {
    pub fn empty() -> Self {
        Self(None)
    }
    pub fn from(range: HashSet<Scalar>) -> Self {
        Self(Some(Arc::new(range)))
    }
    pub fn get_only(&self) -> Option<Scalar> {
        let Some(set) = &self.0 else { return None };
        if set.len() == 1 {
            set.iter().next().copied()
        } else {
            None
        }
    }
    fn combine(a: &Range, b: &Range, combiner_fn: impl Fn(Scalar, Scalar) -> Scalar) -> Range {
        let a = a.0.as_ref().unwrap();
        let b = b.0.as_ref().unwrap();
        let capacity = a.len() * b.len().isqrt();
        let mut output = HashSet::with_capacity(capacity);
        for &a_val in a.as_ref() {
            for &b_val in b.as_ref() {
                output.insert(combiner_fn(a_val, b_val));
            }
        }
        Self::from(output)
    }
    fn is_some(&self) -> bool {
        self.0.is_some()
    }
    fn len(&self) -> usize {
        self.0
            .as_ref()
            .expect("Can't get the size of an empty range")
            .len()
    }
    fn is_disjoint(&self, other: &Self) -> bool {
        let a = self.0.as_ref().expect("Can't compare empty ranges");
        let b = other.0.as_ref().expect("Can't compare empty ranges");
        a.is_disjoint(b)
    }

    fn eq_range(&self, other: &Self) -> Range {
        let can_be_eq = !self.is_disjoint(other);
        let can_be_neq = self.len() > 1 || other.len() > 1 || !can_be_eq;
        match (can_be_eq, can_be_neq) {
            (true, true) => Self::zero_one(),
            (true, false) => Self::one(),
            (false, true) => Self::zero(),
            (false, false) => panic!("Something went wrong: {self:?} {other:?}"),
        }
    }
    fn neq_range(&self, other: &Self) -> Range {
        let range = self.eq_range(other);
        if range == Self::one() {
            Self::zero()
        } else if range == Self::zero() {
            Self::one()
        } else {
            Self::zero_one()
        }
    }
    fn contains(&self, value: Scalar) -> bool {
        let range = self.0.as_ref().expect("Can't check for value in an empty range");
        range.contains(&value)
    }
    fn literal(l: Scalar) -> Self {
        match l {
            0 => Self::zero(),
            1 => Self::one(),
            26 => Self::l26(),
            n => Self::from([n].into_iter().collect()),
        }
    }
}

macro_rules! memoized_common_range {
    ($name:ident => $contents:expr) => {
        impl Range {
            fn $name() -> Self {
                thread_local! {
                    static MEMOIZED_RANGE: LazyCell<Range> = LazyCell::new(|| {
                        Range(Some(Arc::new($contents)))
                    });
                }
                MEMOIZED_RANGE.with(|v| (**v).clone())
            }
        }
    };
}
memoized_common_range!(input => (1..=9).into_iter().collect());
memoized_common_range!(zero => [0].into_iter().collect());
memoized_common_range!(one => [1].into_iter().collect());
memoized_common_range!(zero_one => [0, 1].into_iter().collect());
memoized_common_range!(l26 => [26].into_iter().collect());

#[derive(Clone)]
enum Computed {
    Binary(u64, BoxType<BinaryOp>, Range),
    Literal(Scalar),
    Input(u8),
}
impl Computed {
    fn reduce(mut self, rewrites: &mut HashMap<Computed, Computed>, distribute: bool) -> (Self, bool) {
        let mut was_reduced = false;
        loop {
            if let Some(rewrite) = rewrites.get(&self) {
                self = rewrite.clone();
                was_reduced = true;
                continue;
            }
            let Self::Binary(id, binary_op, range) = self else {
                return (self, was_reduced);
            };
            if range.is_some() {
                return (Self::Binary(id, binary_op, range), was_reduced);
            }
    
            let (reduced, was_reduced_again) = binary_op.clone().reduce(rewrites, distribute);
            self = Self::Binary(id, binary_op, range);
            was_reduced |= was_reduced_again;
            if was_reduced_again {
                rewrites.insert(self, reduced.clone());
            }
            let Self::Binary(_, _, range) = &reduced else {
                return (reduced, was_reduced)
            };
            self = if let Some(l) = range.get_only() {
                return (Self::Literal(l), true)
            } else if !was_reduced_again || distribute {
                return (reduced, was_reduced)
            } else {
                reduced
            };
        }
    }
    fn reduce_2(l: Self, r: Self, rewrites: &mut HashMap<Computed, Computed>, distribute: bool) -> (Self, Self, bool) {
        let (l, l_reduced) = if !l.has_range() { l.reduce(rewrites, distribute) } else { (l, false) };
        let (r, r_reduced) = if !r.has_range() && (!distribute || !l_reduced) { r.reduce(rewrites, distribute) } else { (r, false) };
        (l, r, l_reduced || r_reduced)
    }
    fn reduce_full(mut self, distribute: bool) -> Self {
        let mut rewrites = HashMap::new();
        let mut do_distribute = distribute;
        loop {
            let (new, reduced) = self.reduce(&mut rewrites, do_distribute);
            if reduced {
                do_distribute = false;
                
                // println!("{}", id_map.len());
            } else if do_distribute || !distribute {
                return new;
            } else {
                do_distribute = true;
                // println!("Distributing");
            }
            self = new;
        }
    }
    fn range(&self) -> Range {
        match self {
            Self::Input(_) => Range::input(),
            &Self::Literal(l) => Range::literal(l),
            Self::Binary(_, _, range) => range.clone(),
        }
    }
    fn has_range(&self) -> bool {
        match self {
            Self::Binary(_, _, range) => range.is_some(),
            _ => true,
        }
    }
    fn clone_unfrozen(&self, replacements: &ReductionReplacements) -> (Self, bool) {
        match self {
            Self::Binary(id, op, _) => {
                let (unfrozen_op, needed_unfreezing) = op.clone_unfrozen(replacements);
                if needed_unfreezing {
                    (Self::Binary(*id, BoxType::new(unfrozen_op), Range::empty()), true)
                } else {
                    (self.clone(), false)
                }
            },
            &Self::Input(i) if i < replacements.0.len() as u8 => (Self::Literal(replacements.0[i as usize]), true),
            _ => (self.clone(), false),
        }
    }
    fn clone_with_replacements(&self, replacements: &ReductionReplacements) -> Self {
        self.clone_unfrozen(replacements).0
    }
}
impl Debug for Computed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input(n) => write!(f, "<inp {n:2}>"),
            Self::Literal(l) => write!(f, "{l}"),
            Self::Binary(_, b, _) => write!(f, "{b:.*?}", f.precision().unwrap_or(0).saturating_sub(1)),
        }
    }
}
impl PartialEq for Computed {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Binary(_, a, _),
                Self::Binary(_, b, _),
            ) => a == b,
            (
                Self::Input(a),
                Self::Input(b),
            ) => a == b,
            (
                Self::Literal(a),
                Self::Literal(b),
            ) => a == b,
            _ => false,
        }
    }
}
impl Eq for Computed {}
impl Hash for Computed {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Binary(prehashed, _, _) => {
                state.write_i8(0);
                state.write_u64(*prehashed)
            },
            Self::Input(idx) => {
                state.write_i8(1);
                state.write_u8(*idx);
            },
            Self::Literal(l) => {
                state.write_i8(2);
                (*l).hash(state);
            },
        }
    }
}

#[derive(Clone, PartialEq, Hash)]
enum BinaryOp {
    Add(Computed, Computed),
    Mul(Computed, Computed),
    Div(Computed, Computed),
    Mod(Computed, Computed),
    Eql(Computed, Computed),
    Neq(Computed, Computed),
}
impl BinaryOp {
    // #[inline(always)]
    fn sub_reduce(self: BoxType<Self>, rewrites: &mut HashMap<Computed, Computed>, distribute: bool) -> (BoxType<Self>, bool) {
        macro_rules! sub_reduce_binary {
            ($($variant:ident),+ $(,)?) => {
                match self.as_ref() {
                    $(
                        Self::$variant(l, r) => {
                            let (new_l, new_r, reduced) = Computed::reduce_2(l.clone(), r.clone(), rewrites, distribute);
                            (BoxType::new(Self::$variant(new_l, new_r)), reduced)
                        },
                    )+
                }
            };
        }
        sub_reduce_binary!(Add, Mul, Div, Mod, Eql, Neq)
    }
    fn computed(self) -> Computed {
        Computed::Binary(self.simple_hash(), BoxType::new(self), Range::empty())
    }
    fn computed_reduced(self) -> Computed {
        let range = self.range();
        Computed::Binary(self.simple_hash(), BoxType::new(self), range)
    }
    fn computed_maybe(self, is_reduced: bool) -> Computed {
        if is_reduced {
            self.computed_reduced()
        } else {
            self.computed()
        }
    }
    fn range(&self) -> Range {
        match self {
            Self::Add(l, r) => Range::combine(&l.range(), &r.range(), |l, r| l + r),
            Self::Mul(l, r) => Range::combine(&l.range(), &r.range(), |l, r| l * r),
            Self::Div(l, r) => Range::combine(&l.range(), &r.range(), |l, r| (l as f64 / r as f64).trunc() as Scalar),
            Self::Mod(l, r) => Range::combine(&l.range(), &r.range(), |l, r| l - (l as f64 / r as f64).trunc() as Scalar * r),
            Self::Eql(l, r) => l.range().eq_range(&r.range()),
            Self::Neq(l, r) => l.range().neq_range(&r.range()),
        }
    }
    fn clone_unfrozen(&self, replacements: &ReductionReplacements) -> (Self, bool) {
        macro_rules! collate {
            ($l:ident, $r:ident, $combiner:expr) => {
                {
                    let l = $l.clone_unfrozen(replacements);
                    let r = $r.clone_unfrozen(replacements);
                    (($combiner)(l.0, r.0), l.1 || r.1)
                }
            };
        }
        match self {
            Self::Add(l, r) => collate!(l, r, Self::Add),
            Self::Mul(l, r) => collate!(l, r, Self::Mul),
            Self::Div(l, r) => collate!(l, r, Self::Div),
            Self::Mod(l, r) => collate!(l, r, Self::Mod),
            Self::Eql(l, r) => collate!(l, r, Self::Eql),
            Self::Neq(l, r) => collate!(l, r, Self::Neq),
        }
    }
    // #[inline(always)]
    fn reduce(mut self: BoxType<Self>, rewrites: &mut HashMap<Computed, Computed>, distribute: bool) -> (Computed, bool) {
        use Computed::*;
        use BinaryOp::*;
        let mut sub_reduction_happened = false;
        loop {
            let (sub_reduced, new_sub_reduction_happened) = self.sub_reduce(rewrites, distribute);
            self = sub_reduced;
            if new_sub_reduction_happened {
                sub_reduction_happened = true;
                if distribute { break }
            } else {
                break
            }
        }
        if sub_reduction_happened && distribute { return (Binary(self.simple_hash(), self, Range::empty()), true) }
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
            Add(Binary(_, l, _), r) if matches!(l.as_ref(), Add(_, _)) => {
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
            Mul(l @ Literal(_), r) => (
                Mul(r.clone(), l.clone()).computed(),
                true,
            ),
            // Right-fold multiplies
            Mul(Binary(_, l, _), r) if matches!(l.as_ref(), Mul(_, _)) => {
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
            },
            // Distribute
            | Mul(Binary(_, add, _), coeff)
            | Mul(coeff, Binary(_, add, _))
            if distribute && matches!(add.as_ref(), Add(_, _)) => {
                // println!("a");
                let Add(add_l, add_r) = add.as_ref() else { unreachable!() };
                (
                    Add(
                        Mul(add_l.clone(), coeff.clone()).computed(),
                        Mul(add_r.clone(), coeff.clone()).computed(),
                    ).computed(),
                    true,
                )
            },

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
                Literal((*l as f64 / *r as f64).trunc() as Scalar),
                true,
            ),
            // Remove immediately nested divisions
            // Div(Binary(_, l, _), r) if matches!(l.as_ref(), Div(_, _)) => {
            //     let Div(l_l, l_r) = l.as_ref() else { unreachable!() };
            //     (
            //         Div(
            //             l_l.clone(),
            //             Mul(l_r.clone(), r.clone()).computed(),
            //         ).computed(),
            //         true,
            //     )
            // },
            // Div(l, Binary(_, r, _)) if matches!(r.as_ref(), Div(_, _)) => {
            //     let Div(r_l, r_r) = r.as_ref() else { unreachable!() };
            //     (
            //         Div(
            //             Mul(l.clone(), r_r.clone()).computed(),
            //             r_l.clone(),
            //         ).computed(),
            //         true,
            //     )
            // },

            Mod(_, Literal(1)) => (
                Literal(0),
                true,
            ),
            Mod(Literal(l), Literal(r)) => (
                Literal(l - (*l as f64 / *r as f64).trunc() as Scalar * r),
                true,
            ),


            // Reducing literal eql literal
            Eql(Literal(l), Literal(r)) => (
                Literal(if l == r { 1 } else { 0 }),
                true,
            ),

            // Converting equality into "subtractions"
            Eql(l, r) if !matches!(r, Literal(0)) => (
                Eql(
                    Add(l.clone(), Mul(r.clone(), Literal(-1)).computed()).computed(),
                    Literal(0),
                ).computed(),
                true,
            ),

            // Reducing neq
            | Eql(Binary(_, cmp, _), Literal(0))
            | Eql(Literal(0), Binary(_, cmp, _))
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
    }
    pub fn simple_hash(&self) -> u64 {
        use std::hash::{ Hasher, BuildHasher };
        let mut hasher = NoRandomState::default().build_hasher();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
impl Debug for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let multiline_recurse = f.precision().unwrap_or(0);
        let (a, b) = match self {
            | Self::Add(a, b)
            | Self::Mul(a, b)
            | Self::Div(a, b)
            | Self::Mod(a, b)
            | Self::Eql(a, b)
            | Self::Neq(a, b) => (a, b),
        };
        if multiline_recurse > 0 && matches!(a, Computed::Binary(..)) && matches!(b, Computed::Binary(..)) {
            writeln!(f, "(")?;
            for line in format!("{a:.*?}", multiline_recurse).lines() {
                writeln!(f, "    {line}")?;
            }
            write!(f, "    ")?;
            match self {
                Self::Add(..) => writeln!(f, "+"),
                Self::Mul(..) => writeln!(f, "*"),
                Self::Div(..) => writeln!(f, "/"),
                Self::Mod(..) => writeln!(f, "%"),
                Self::Eql(..) => writeln!(f, "=="),
                Self::Neq(..) => writeln!(f, "!="),
            }?;
            for line in format!("{b:.*?}", multiline_recurse).lines() {
                writeln!(f, "    {line}")?;
            }
            write!(f, ")")
        } else {
            match self {
                Self::Add(a, b) => write!(f, "({a:.*?} + {b:.*?})", multiline_recurse, multiline_recurse),
                Self::Mul(a, b) => write!(f, "({a:.*?} * {b:.*?})", multiline_recurse, multiline_recurse),
                Self::Div(a, b) => write!(f, "({a:.*?} / {b:.*?})", multiline_recurse, multiline_recurse),
                Self::Mod(a, b) => write!(f, "({a:.*?} % {b:.*?})", multiline_recurse, multiline_recurse),
                Self::Eql(a, b) => write!(f, "({a:.*?} == {b:.*?})", multiline_recurse, multiline_recurse),
                Self::Neq(a, b) => write!(f, "({a:.*?} != {b:.*?})", multiline_recurse, multiline_recurse),
            }
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
enum Op { Reg(Reg), Literal(Scalar) }
impl FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = if s.as_bytes()[0].is_ascii_alphabetic() {
            Self::Reg(Reg::from_str(s)?)
        } else {
            Self::Literal(Scalar::from_str(s).map_err(|e| e.to_string())?)
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
        self.w = self.w.clone().reduce_full(false);
        self.x = self.x.clone().reduce_full(false);
        self.y = self.y.clone().reduce_full(false);
        self.z = self.z.clone().reduce_full(false);
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
    guesses: impl Iterator<Item = Scalar> + Clone + Send,
    stop: Arc<AtomicBool>,
) -> Option<Vec<Scalar>> {
        if input_num == 14 { return Some(Vec::with_capacity(14)); }
        let mut thread_fns = vec![];
        for guess in guesses.clone() {
            let mut replacements = ReductionReplacements(replacements.0.clone());
            replacements.0.push(guess);
            let guesses = guesses.clone();
            let stop = stop.clone();
            let thread_fn = move || {
                let test_base = base.clone_with_replacements(&replacements).reduce_full(input_num <= 1);
                if test_base.range().contains(0) {
                    use std::io::Write;
                    print!("\x1b[0K\r");
                    for n in replacements.0.iter().copied().chain([guesses.clone().next().unwrap(); 14]).take(14) {
                        print!("{n}");
                    }
                    std::io::stdout().flush().unwrap();

                    if let Some(mut output) = input_replacements(
                        &test_base,
                        input_num+1,
                        &mut replacements,
                        guesses,
                        stop,
                    ) {
                        output.push(guess);
                        return Some(output)
                    }
                }
                replacements.0.pop();
                None
            };
            thread_fns.push(thread_fn);
        }
        let output = std::thread::scope(|t| {
            let mut threads = vec![];
            let mut outputs = vec![];
            for thread_fn in thread_fns {
                // if stop.load(std::sync::atomic::Ordering::Relaxed) { break }
                if input_num == 1 {
                    threads.push(t.spawn(thread_fn));
                } else {
                    let output = thread_fn();
                    if output.is_some() { return output }
                    outputs.push(output);
                }
            }
            while !threads.is_empty() {
                let output = threads.remove(0).join().unwrap();
                outputs.push(output.clone());
                if output.is_some() {
                    stop.store(true, std::sync::atomic::Ordering::Relaxed);
                    return output;
                }
            }
            None
        });
        if input_num == 0 { println!(); }
        output
    }

pub fn part1(input: &str) -> i64 {
    let instructions = parse_input(input);
    let mut state = State::init();
    for instruction in instructions {
        state.exec_instruction(instruction);
        state.reduce();
    }
    // println!("{state:?}");
    let solution = input_replacements(
        &state.z.clone().reduce_full(true),
        0,
        &mut ReductionReplacements(vec![]),
        (1..=9).rev(),
        Arc::new(AtomicBool::new(false)),
    );
    solution.unwrap().into_iter().rev().fold(0, |acc, n| acc * 10 + n as i64)
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
        1..=9,
        Arc::new(AtomicBool::new(false)),
    );
    solution.unwrap().into_iter().rev().fold(0, |acc, n| acc * 10 + n as i64)
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect()
}
