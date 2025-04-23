aoc_tools::aoc_sol!(day07 2024: part1, part2);

type Scalar = i64;

pub fn part1(input: &str) -> Scalar {
    let equations = parse_input(input);

    let mut sum = 0;
    for equation in equations {
        if equation.solve() {
            sum += equation.target;
        }
    }

    sum
}

pub fn part2(input: &str) -> Scalar {
    let equations = parse_input(input);

    let mut sum = 0;
    for equation in equations {
        if equation.solve_concat() {
            sum += equation.target;
        }
    }

    sum
}

fn parse_input(input: &str) -> Vec<Equation> {
    let mut equations = Vec::with_capacity(input.len() / 8);
    input.lines()
        .map(|l| {
            let (target, values_str) = l.split_once(": ").unwrap();
            let target = parse_unsigned(target);
            let mut values = Vec::with_capacity(values_str.len() / 2);
            values_str.split(' ')
                .map(parse_unsigned)
                .inspect(|v| assert!(*v < 10_000)) // Unsafe condition asserted here
                .for_each(|v| values.push(v));

            Equation { target, values }
        })
        .for_each(|eq| equations.push(eq));

    equations
}

fn parse_unsigned(s: &str) -> i64 {
    let mut curr = 0;
    for &digit in s.as_bytes() {
        curr *= 10;
        curr += (digit - b'0') as i64;
    }
    curr
}

#[derive(Clone, PartialEq, Eq)]
struct Equation {
    target: Scalar,
    values: Vec<Scalar>,
}

impl Equation {
    // fn solve_recursive(target: Scalar, curr: Scalar, remaining: &[Scalar], operators: &[Operator]) -> Option<OpVec> {        
    fn solve_recursive<'a, T: IntoIterator<Item = &'a Operator> + Copy>(
        target: Scalar,
        curr: Scalar,
        remaining: &[Scalar],
        operators: T,
    ) -> bool {
        let next = remaining[0];
        let remaining = &remaining[1..];
        for &op in operators {
            let curr = unsafe { op.apply_unchecked(curr, next) };
            if curr > target && next > 1 { break }
            if remaining.is_empty() {
                if curr == target { return true }
                else { continue }
            }

            if Self::solve_recursive(target, curr, remaining, operators) { return true }
        }

        false
    }
    pub fn solve(&self) -> bool {
        Self::solve_recursive(
            self.target,
            self.values[0],
            &self.values[1..],
            &[Operator::Plus, Operator::Mult],
        )
    }
    pub fn solve_concat(&self) -> bool {
        Self::solve_recursive(
            self.target,
            self.values[0],
            &self.values[1..],
            &[Operator::Plus, Operator::Mult, Operator::Concat],
        )
    }

    #[allow(dead_code)]
    pub fn valid(&self, solution: OpVec) -> bool {
        let output = self.values[1..].iter()
            .copied()
            .zip(solution.iter())
            .fold(self.values[0], |a, (b, op)| unsafe { op.apply_unchecked(a, b) });

        output == self.target
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator { Plus = 0b00, Mult = 0b01, Concat = 0b10 }
impl Operator {
    pub fn from_u8(v: u8) -> Self {
        match (v & 0b10 == 0b10, v & 0b1 == 1) {
            (false, false) => Self::Plus,
            (false, true ) => Self::Mult,
            (true , _    ) => Self::Concat,
        }
    }
    pub unsafe fn apply_unchecked(&self, a: Scalar, b: Scalar) -> Scalar {
        match self {
            Self::Plus => a + b,
            Self::Mult => a * b,
            Self::Concat => {
                let a_coeff = match b {
                    ..10 => 10,
                    ..100 => 100,
                    ..1_000 => 1_000,
                    ..10_000 => 10_000,
                    // ..100_000 => 100_000,
                    // ..1_000_000 => 1_000_000,
                    // ..10_000_000 => 10_000_000,
                    // ..100_000_000 => 100_000_000,
                    // ..1_000_000_000 => 1_000_000_000,
                    // ..10_000_000_000 => 10_000_000_000,
                    // ..100_000_000_000 => 100_000_000_000,
                    // ..1_000_000_000_000 => 1_000_000_000_000,
                    // ..10_000_000_000_000 => 10_000_000_000_000,
                    // ..100_000_000_000_000 => 100_000_000_000_000,
                    // ..1_000_000_000_000_000 => 1_000_000_000_000_000,
                    // ..10_000_000_000_000_000 => 10_000_000_000_000_000,
                    // ..100_000_000_000_000_000 => 100_000_000_000_000_000,
                    // ..1_000_000_000_000_000_000 => 1_000_000_000_000_000_000,
                    _ => unsafe { std::hint::unreachable_unchecked() }, // Asserted in parse_input
                };
                a * a_coeff + b
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct OpVec {
    data: u64,
    len_bits: u32,
}

#[allow(dead_code)]
impl OpVec {
    pub fn new_one(v: Operator) -> Self  {
        Self { data: v as u64, len_bits: 2 }
    }
    pub fn push(&mut self, op: Operator) {
        assert!(self.len_bits < u64::BITS / 2);

        self.data |= (op as u64) << self.len_bits;
        self.len_bits += 2;
    }

    pub fn len(&self) -> u32 {
        self.len_bits >> 1
    }

    pub fn reverse(&mut self) {
        for i in 0..self.len_bits >> 2 {
            let shift_a = i * 2;
            let shift_b = self.len_bits - shift_a - 2;
            let temp_a = (self.data >> shift_a) & 0b11;
            let temp_b = (self.data >> shift_b) & 0b11;
            let xor = temp_a ^ temp_b;
            self.data ^= xor << shift_a;
            self.data ^= xor << shift_b;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Operator> + '_ {
        (0..self.len()).map(|i| (self.data >> (i*2)) as u8).map(Operator::from_u8)
    }
}
