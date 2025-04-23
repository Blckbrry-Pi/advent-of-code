aoc_tools::aoc_sol!(day24 2024: part1, part2);

pub fn part1(input: &str) -> usize {
    let abstract_exprs = parse_input(input);

    let mut zs: Vec<_> = abstract_exprs.iter()
        .filter(|(wire, _)| wire.chars()[0] == 'z')
        .collect();
    zs.sort_by_cached_key(|(w, _)| w.0);

    let mut cache = HashMap::new();
    let mut bits = 0;
    for (_, &expr) in zs.iter().rev() {
        bits <<= 1;
        bits |= expr.eval(&abstract_exprs, &mut cache) as usize;
    }

    bits
}

pub fn part2(input: &str) -> String {
    let abstract_exprs = parse_input(input);
    let mut uses: HashMap<Wire, ReferencedIn> = HashMap::with_capacity(abstract_exprs.len());
    for &expr in abstract_exprs.values() {
        match expr {
            AbstractWireExpr::Bit(_) => continue,
            AbstractWireExpr::And(a, b) => {
                uses.entry(a).or_default().and = true;
                uses.entry(b).or_default().and = true;
            },
            AbstractWireExpr::Or(a, b) => {
                uses.entry(a).or_default().or = true;
                uses.entry(b).or_default().or = true;
            }
            AbstractWireExpr::Xor(a, b) => {
                uses.entry(a).or_default().xor = true;
                uses.entry(b).or_default().xor = true;
            }
        };
    }

    let mut used_cout_or_x_xor_ys = HashSet::with_capacity(abstract_exprs.len() / 3);
    let mut used_outputs = HashSet::with_capacity(abstract_exprs.len() / 6);
    let mut used_ands = HashSet::with_capacity(abstract_exprs.len() / 3);
    for (wire, usage) in uses {
        match usage {
            ReferencedIn { and:  true, or: false, xor:  true } => {
                let wire_matches = wire.chars()[0] == 'x' || wire.chars()[0] == 'y';
                if !wire_matches { used_cout_or_x_xor_ys.insert(wire); }
            },
            ReferencedIn { and: false, or:  true, xor: false } => { used_ands.insert(wire); },
            ReferencedIn { and: false, or: false, xor: false } => { used_outputs.insert(wire); },
            _ => println!("Found oddly used wire: {wire:?}"),
        }
    }

    let mut apparent_cout_or_x_xor_ys = HashSet::with_capacity(abstract_exprs.len() / 3);
    let mut apparent_outputs = HashSet::with_capacity(abstract_exprs.len() / 6);
    let mut apparent_ands = HashSet::with_capacity(abstract_exprs.len() / 3);
    for (wire, expr) in abstract_exprs.iter() {
        match expr {
            AbstractWireExpr::Or(_, _) => { apparent_cout_or_x_xor_ys.insert(*wire); },
            AbstractWireExpr::Xor(a, _) => {
                let a_matches = a.chars()[0] == 'x' || a.chars()[0] == 'y';
                if a_matches {
                    // apparent_axorb.insert(*wire);
                    apparent_cout_or_x_xor_ys.insert(*wire);
                } else {
                    apparent_outputs.insert(*wire);
                }
            },
            AbstractWireExpr::And(_, _) => { apparent_ands.insert(*wire); },
            _ => (),
        }
    }


    let mut known_swaps = HashSet::with_capacity(16);

    // Add wires treated like C_out or x__ XOR y__ but not assigned as them (and vice versa)
    for diff in used_cout_or_x_xor_ys.difference(&apparent_cout_or_x_xor_ys) {
        known_swaps.insert(*diff);
    }
    for diff in apparent_cout_or_x_xor_ys.difference(&used_cout_or_x_xor_ys) {
        known_swaps.insert(*diff);
    }

    // Add wires treated like ___ AND ___ but not assigned as them (and vice versa)
    for diff in used_ands.difference(&apparent_ands) {
        known_swaps.insert(*diff);
    }
    for diff in apparent_ands.difference(&used_ands) {
        known_swaps.insert(*diff);
    }

    // Remove the "problematic" wire if the odd half-adder output is correct
    // because it gets flagged otherwise
    known_swaps.retain(|w| {
        let expr = abstract_exprs.get(w).unwrap();
        let is_sum_of_full_adder = expr == &AbstractWireExpr::Xor(
            Wire::from_str("x00"),
            Wire::from_str("y00"),
        );
        if is_sum_of_full_adder {
            return w != &Wire::from_str("z00");
        } else {
            true
        }
    });

    // !FIXME: Not rigorous, works for my input
    // Remove the "problematic" half adder carry wire
    known_swaps.retain(|w| {
        let expr = abstract_exprs.get(w).unwrap();
        expr != &AbstractWireExpr::And(
            Wire::from_str("x00"),
            Wire::from_str("y00"),
        )
    });

    // !FIXME: Not rigorous, works for my input
    // Remove the highest z wire
    let max_z = abstract_exprs.keys().filter(|k| k.chars()[0] == 'z').count() - 1;
    known_swaps.retain(|w| w != &Wire::from_str(&format!("z{max_z}")));

    // Sort invalid wires
    let mut known_swaps: Vec<_> = known_swaps.into_iter().collect();
    known_swaps.sort();
    assert_eq!(known_swaps.len(), 8);

    // Join with commas and output
    let mut output = known_swaps.into_iter().fold(String::new(), |mut s, w| {
        use std::fmt::Write;
        write!(&mut s, "{w:?},").unwrap();
        s
    });
    output.pop();
    output
}

fn parse_input(input: &str) -> HashMap<Wire, AbstractWireExpr> {
    let mut wires = HashMap::new();

    let (bit_defs, expr_defs) = input.split_once("\n\n").unwrap();
    for bit_def in bit_defs.lines() {
        let (name, bit) = bit_def.split_once(": ").unwrap();
        let bit = bit.as_bytes()[0] - b'0';
        wires.insert(Wire::from_str(name), AbstractWireExpr::Bit(bit));
    }
    for expr_def in expr_defs.trim().lines() {
        let (expr, name) = expr_def.split_once(" -> ").unwrap();
        let (a, rest) = expr.split_once(' ').unwrap();
        let (op, b) = rest.split_once(' ').unwrap();
        let (a, b) = (Wire::from_str(a), Wire::from_str(b));
        let (a, b) = (a.min(b), a.max(b));
        let expr = match op {
            "AND" => AbstractWireExpr::And(a, b),
            "OR" => AbstractWireExpr::Or(a, b),
            "XOR" => AbstractWireExpr::Xor(a, b),
            _ => unreachable!(),
        };
        wires.insert(Wire::from_str(name), expr);
    }

    wires
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Wire(u16);
impl Wire {
    pub fn from_str(s: &str) -> Self {
        let b0 = s.as_bytes()[0] as u8;
        let b1 = s.as_bytes()[1] as u8;
        let b2 = s.as_bytes()[2] as u8;

        let b0 = if (b'a'..=b'z').contains(&b0) { b0 - b'a' } else { b0 - b'0' + 26 };
        let b1 = if (b'a'..=b'z').contains(&b1) { b1 - b'a' } else { b1 - b'0' + 26 };
        let b2 = if (b'a'..=b'z').contains(&b2) { b2 - b'a' } else { b2 - b'0' + 26 };

        Self(b0 as u16 * 36 * 36 + b1 as u16 * 36 + b2 as u16)
    }

    pub fn chars(&self) -> [char; 3] {
        let b0 = self.0 / 36 / 36;
        let b1 = self.0 / 36 % 36;
        let b2 = self.0 % 36;

        let b0 = if b0 < 26 { b0 as u8 + b'a' } else { b0 as u8 - 26 + b'0' };
        let b1 = if b1 < 26 { b1 as u8 + b'a' } else { b1 as u8 - 26 + b'0' };
        let b2 = if b2 < 26 { b2 as u8 + b'a' } else { b2 as u8 - 26 + b'0' };

        [b0 as char, b1 as char, b2 as char]
    }
}

impl Debug for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [b0, b1, b2] = self.chars();
        write!(f, "{b0}{b1}{b2}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum AbstractWireExpr {
    Bit(u8),
    And(Wire, Wire),
    Or(Wire, Wire),
    Xor(Wire, Wire),
}

impl AbstractWireExpr {
    pub fn eval(&self, wires: &HashMap<Wire, AbstractWireExpr>, cached: &mut HashMap<AbstractWireExpr, u8>) -> u8 {
        if let Some(&cached) = cached.get(self) { return cached; }
        let output = match self {
            Self::Bit(b) => *b,
            Self::And(a, b) => wires.get(a).unwrap().eval(wires, cached) & wires.get(b).unwrap().eval(wires, cached),
            Self::Or (a, b) => wires.get(a).unwrap().eval(wires, cached) | wires.get(b).unwrap().eval(wires, cached),
            Self::Xor(a, b) => wires.get(a).unwrap().eval(wires, cached) ^ wires.get(b).unwrap().eval(wires, cached),
        };
        cached.insert(*self, output);
        output
    }
}

impl Debug for AbstractWireExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bit(b) => write!(f, "{b}"),
            Self::And(a, b) => write!(f, "{a:?} AND {b:?}"),
            Self::Or(a, b) => write!(f, "{a:?} OR {b:?}"),
            Self::Xor(a, b) => write!(f, "{a:?} XOR {b:?}"),
        }
    }
}

// #[derive(Clone, PartialEq, Eq, Hash)]
// enum ConcreteWireExpr {
//     Bit(u8),
//     And(Box<ConcreteWireExpr>, Box<ConcreteWireExpr>),
//     Or(Box<ConcreteWireExpr>, Box<ConcreteWireExpr>),
//     Xor(Box<ConcreteWireExpr>, Box<ConcreteWireExpr>),
// }
// impl ConcreteWireExpr {
//     pub fn eval(&self) -> u8 {
//         match self {
//             Self::Bit(b) => *b,
//             Self::And(a, b) => a.eval() & b.eval(),
//             Self::Or(a, b) => a.eval() | b.eval(),
//             Self::Xor(a, b) => a.eval() ^ b.eval(),
//         }
//     }
// }
// impl Debug for ConcreteWireExpr {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Bit(b) => write!(f, "{b}"),
//             Self::And(a, b) => write!(f, "({a:?} AND {b:?})"),
//             Self::Or(a, b) => write!(f, "({a:?} OR {b:?})"),
//             Self::Xor(a, b) => write!(f, "({a:?} XOR {b:?})"),
//         }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct ReferencedIn { and: bool, or: bool, xor: bool }
