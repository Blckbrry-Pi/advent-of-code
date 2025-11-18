use std::{fmt::Debug, str::FromStr};

pub type ParamType = u32;
pub type RegVal = u64;

pub struct Program(pub Vec<Instruction>, pub Vec<Directive>);
impl Program {
    #[allow(irrefutable_let_patterns)]
    pub fn get_ip(&self) -> Option<ParamType> {
        for directive in &self.1 {
            if let Directive::InstructionPointer(ip_val) = directive {
                return Some(*ip_val);
            }
        }
        None
    }
    /// Returns whether or not the program definitely doesn't halt
    pub fn execute<const REG_COUNT: usize>(self, state: &mut State<REG_COUNT>) -> bool {
        for directive in self.1 {
            directive.exec(state);
        }
        if let Some(binding) = state.1 {
            while state.2 < self.0.len() {
                state.0[binding as usize] = state.2 as RegVal;
                self.0[state.2].apply(state);
                state.2 = state.0[binding as usize] as usize;
                state.2 += 1;
            }
        } else {
            while state.2 < self.0.len() {
                self.0[state.2].apply(state);
                state.2 += 1;
            }
        }
        false
    }
}
impl FromStr for Program {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut output = Program(vec![], vec![]);
        for (i, l) in s.lines().enumerate() {
            if l.trim().is_empty() { continue }
            let directive_error = match l.parse::<Directive>() {
                Ok(d) => {
                    output.1.push(d);
                    continue;
                }
                Err(e) => e,
            };
            let instruction_error = match l.parse::<Instruction>() {
                Ok(i) => {
                    output.0.push(i);
                    continue;
                }
                Err(e) => e,
            };
            return Err(format!("Error on line {}: {} OR {}", i+1, directive_error, instruction_error))
        }
        Ok(output)
    }
}
impl Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in &self.1 {
            writeln!(f, "{d:?}")?;
        }
        let ip = self.get_ip();
        for (i, instruction) in self.0.iter().enumerate() {
            if f.alternate() {
                write!(f, "0x{i:02x}: ")?;
                instruction.display_with(i, ip);
                writeln!(f)?;
            } else {
                writeln!(f, "{instruction:?}")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Directive {
    InstructionPointer(ParamType),
}
impl Directive {
    pub fn exec<const REG_COUNT: usize>(&self, state: &mut State<REG_COUNT>) {
        match self {
            Self::InstructionPointer(v) => state.set_ip(*v),
        }
    }
}
impl FromStr for Directive {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix('#') else { return Err("Missing hash for directive".to_string()); };
        let Some((directive, value)) = s.split_once(' ') else {
            return Err("Missing value for directive".to_string());
        };
        match directive {
            "ip" => {
                let value = value.parse::<ParamType>().map_err(|e| format!("Failed to parse instruction point value: {e}"))?;
                Ok(Self::InstructionPointer(value))
            },
            v => Err(format!("Invalid directive: {v:?}")),
        }
    }
}
impl Debug for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InstructionPointer(v) => write!(f, "#ip {v}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State<const REG_COUNT: usize>(pub [RegVal; REG_COUNT], Option<ParamType>, usize);
impl<const REG_COUNT: usize> State<REG_COUNT> {
    pub fn zeroed() -> Self {
        Self([0; REG_COUNT], None, 0)
    }
    pub fn set_ip(&mut self, v: ParamType) {
        self.1 = Some(v);
    }
}
impl<const REG_COUNT: usize> FromStr for State<REG_COUNT> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix('[') else { return Err("Missing opening square bracket".to_string()) };
        let Some(s) = s.strip_suffix(']') else { return Err("Missing closing square bracket".to_string()) };
        let mut output = Self([0; REG_COUNT], None, 0);
        let mut remaining = s;
        for i in 0..REG_COUNT-1 {
            let Some((v, rest)) = remaining.split_once(", ") else {
                return Err(format!("Expected {REG_COUNT} values, got only {}", i+1));
            };
            let Ok(v) = v.parse::<RegVal>() else {
                return Err(format!("Invalid number {v:?} at idx {i}"));
            };
            output.0[i] = v;
            remaining = rest;
        }
        let Ok(last) = remaining.parse::<RegVal>() else {
            return Err(format!("Invalid number {remaining:?} at idx {}", REG_COUNT-1));
        };
        output.0[REG_COUNT-1] = last;
        Ok(output)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Params { pub a: ParamType, pub b: ParamType, pub c: ParamType }
impl FromStr for Params {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let Some((a, rest)) = s.split_once(' ') else {
            return Err("Missing params B and C".to_string());
        };
        let Some((b, c)) = rest.split_once(' ') else {
            return Err("Missing param C".to_string());
        };
        let Ok(a) = a.parse::<ParamType>() else {
            return Err(format!("Invalid param A {a:?} "));
        };
        let Ok(b) = b.parse::<ParamType>() else {
            return Err(format!("Invalid param B {b:?} "));
        };
        let Ok(c) = c.parse::<ParamType>() else {
            return Err(format!("Invalid param C {c:?} "));
        };
        Ok(Self { a, b, c })
    }
}
impl Debug for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.a, self.b, self.c)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Instruction(pub Opcode, pub Params);
impl Instruction {
    pub fn display_with(&self, idx: usize, binding: Option<ParamType>) {
        match self.0 {
            Opcode::SetI => if Some(self.1.c) == binding {
                print!("goto 0x{:02x};", self.1.a + 1);
            } else {
                if Some(self.1.a) == binding {
                    print!("v{} = {};", self.1.c, idx as RegVal);
                } else {
                    print!("v{} = {};", self.1.c, self.1.a as RegVal);
                }
            },
            Opcode::SetR => if Some(self.1.c) == binding {
                if Some(self.1.a) == binding {
                    print!("goto v{} + 1;", self.1.a);
                } else {
                    print!("goto 0x{:02x};", idx as RegVal + 1);
                }
            } else {
                if Some(self.1.a) == binding {
                    print!("v{} = {};", self.1.c, idx as RegVal);
                } else {
                    print!("v{} = v{};", self.1.c, self.1.a);
                }
            },
            Opcode::AddI => if Some(self.1.c) == binding {
                if Some(self.1.a) == binding {
                    print!("goto 0x{:02x};", idx as RegVal + self.1.b as RegVal + 1);
                } else {
                    print!("goto v{} + 0x{:02x};", self.1.a, self.1.b + 1);
                }
            } else {
                if Some(self.1.a) == binding {
                    print!("v{} = {}", self.1.c, self.1.b as RegVal + idx as RegVal);
                } else if self.1.a == self.1.c {
                    print!("v{} += {}", self.1.c, self.1.b);
                } else {
                    print!("v{} = v{} + {}", self.1.c, self.1.a, self.1.b);
                }
            },
            Opcode::AddR => if self.1.a == self.1.c || self.1.b == self.1.c {
                let other = if self.1.a == self.1.c { self.1.b } else { self.1.a };
                if Some(other) == binding {
                    print!("v{} += {}", self.1.c, idx);
                } else {
                    print!("v{} += v{}", self.1.c, other);
                }
            } else {
                print!("v{} = v{} + v{}", self.1.c, self.1.a, self.1.b);
            }
            Opcode::MulI | Opcode::BanI | Opcode::BorI => {
                let (op_char, hex) = match self.0 {
                    Opcode::MulI => ('*', false),
                    Opcode::BanI => ('&', true),
                    Opcode::BorI => ('|', true),
                    _ => unreachable!(),
                };
                if self.1.a == self.1.c {
                    print!("v{} {op_char}= ", self.1.c);
                        if hex {
                            print!("0x{:02x}", self.1.b);
                        } else {
                            print!("{}", self.1.b);
                        }
                } else {
                    if Some(self.1.a) == binding {
                        let value = match self.0 {
                            Opcode::MulI => self.1.b as RegVal * idx as RegVal,
                            Opcode::BanI => self.1.b as RegVal & idx as RegVal,
                            Opcode::BorI => self.1.b as RegVal | idx as RegVal,
                            _ => unreachable!(),
                        };
                        print!("v{} = ", self.1.c);
                        if hex {
                            print!("0x{value:02x}");
                        } else {
                            print!("{value}");
                        }
                    } else {
                        print!("v{} = v{} {op_char} ", self.1.c, self.1.a);
                        if hex {
                            print!("0x{:02x}", self.1.b);
                        } else {
                            print!("{}", self.1.b);
                        }
                    }
                }
            }
            Opcode::MulR | Opcode::BanR | Opcode::BorR => {
                let (op_char, hex) = match self.0 {
                    Opcode::MulI => ('*', false),
                    Opcode::BanI => ('&', true),
                    Opcode::BorI => ('|', true),
                    _ => unreachable!(),
                };
                if self.1.a == self.1.c || self.1.b == self.1.c {
                    let other = if self.1.a == self.1.c { self.1.b } else { self.1.a };
                    if Some(other) == binding {
                        print!("v{} {op_char}= ", self.1.c);
                        if hex {
                            print!("0x{idx:02x}");
                        } else {
                            print!("{idx}");
                        }
                    } else {
                        print!("v{} {op_char}= v{}", self.1.c, other);
                    }
                } else {
                    print!("v{} = v{} {op_char} v{}", self.1.c, self.1.a, self.1.b);
                }
            },
            | Opcode::GtIR | Opcode::GtRI | Opcode::GtRR
            | Opcode::EqIR | Opcode::EqRI | Opcode::EqRR => {
                if Some(self.1.c) == binding {
                    print!("goto ");
                } else {
                    print!("v{} = ", self.1.c);
                }
                let op = if matches!(self.0, Opcode::GtIR | Opcode::GtRI | Opcode::GtRR) { ">" } else { "==" };
                if self.0.a_is_reg() {
                    if Some(self.1.a) == binding {
                        print!("{idx}");
                    } else {
                        print!("v{}", self.1.a);
                    }
                } else {
                    print!("{}", self.1.a);
                }
                print!(" {op} ");
                if self.0.b_is_reg() {
                    if Some(self.1.b) == binding {
                        print!("{idx}");
                    } else {
                        print!("v{}", self.1.b);
                    }
                } else {
                    print!("{}", self.1.b);
                }
            }
        }
    }
    pub fn apply<const REG_COUNT: usize>(&self, state: &mut State<REG_COUNT>) {
        let a = if self.0.a_is_reg() { state.0[self.1.a as usize] } else { self.1.a as RegVal };
        let b = if self.0.b_is_reg() { state.0[self.1.b as usize] } else { self.1.b as RegVal };
        let store = self.0.apply(a, b);
        state.0[self.1.c as usize] = store;
    }
}
impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((opcode, params)) = s.split_once(' ') else {
            return Err("Missing params after opcode".to_string());
        };
        let opcode = opcode.parse::<Opcode>()?;
        let params = params.parse::<Params>()?;
        Ok(Instruction(opcode, params))
    }
}
impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.0, self.1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    AddR,
    AddI,

    MulR,
    MulI,

    BanR,
    BanI,

    BorR,
    BorI,

    SetR,
    SetI,

    GtIR,
    GtRI,
    GtRR,

    EqIR,
    EqRI,
    EqRR,
}
impl Opcode {
    pub const ALL: [Self; 16] = [
        Self::AddR, Self::AddI,
        Self::MulR, Self::MulI,
        Self::BanR, Self::BanI,
        Self::BorR, Self::BorI,
        Self::SetR, Self::SetI,
        Self::GtIR, Self::GtRI, Self::GtRR,
        Self::EqIR, Self::EqRI, Self::EqRR,
    ];

    pub fn a_is_reg(&self) -> bool {
        matches!(
            self,
            | Self::AddR | Self::AddI
            | Self::MulR | Self::MulI
            | Self::BanR | Self::BanI
            | Self::BorR | Self::BorI
            | Self::SetR
            | Self::GtRI | Self::GtRR
            | Self::EqRI | Self::EqRR
        )
    }
    pub fn b_is_reg(&self) -> bool {
        matches!(
            self,
            | Self::AddR
            | Self::MulR
            | Self::BanR
            | Self::BorR
            | Self::GtIR | Self::GtRR
            | Self::EqIR | Self::EqRR
        )
    }
    pub fn apply(&self, a: RegVal, b: RegVal) -> RegVal {
        match self {
            Self::AddR | Self::AddI => a + b,
            Self::MulR | Self::MulI => a * b,
            Self::BanR | Self::BanI => a & b,
            Self::BorR | Self::BorI => a | b,
            Self::SetR | Self::SetI => a,
            Self::GtIR | Self::GtRI | Self::GtRR => if a > b { 1 } else { 0 },
            Self::EqIR | Self::EqRI | Self::EqRR => if a == b { 1 } else { 0 },
        }
    }
}
impl FromStr for Opcode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let o = match s {
            "addr" => Self::AddR,
            "addi" => Self::AddI,

            "mulr" => Self::MulR,
            "muli" => Self::MulI,

            "banr" => Self::BanR,
            "bani" => Self::BanI,

            "borr" => Self::BorR,
            "bori" => Self::BorI,

            "setr" => Self::SetR,
            "seti" => Self::SetI,

            "gtir" => Self::GtIR,
            "gtri" => Self::GtRI,
            "gtrr" => Self::GtRR,

            "eqir" => Self::EqIR,
            "eqri" => Self::EqRI,
            "eqrr" => Self::EqRR,

            _ => return Err(format!("Invalid opcode {s:?}")),
        };
        Ok(o)
    }
}
impl Debug for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddR => write!(f, "addr"),
            Self::AddI => write!(f, "addi"),

            Self::MulR => write!(f, "mulr"),
            Self::MulI => write!(f, "muli"),

            Self::BanR => write!(f, "banr"),
            Self::BanI => write!(f, "bani"),

            Self::BorR => write!(f, "borr"),
            Self::BorI => write!(f, "bori"),

            Self::SetR => write!(f, "setr"),
            Self::SetI => write!(f, "seti"),

            Self::GtIR => write!(f, "gtir"),
            Self::GtRI => write!(f, "gtri"),
            Self::GtRR => write!(f, "gtrr"),

            Self::EqIR => write!(f, "eqir"),
            Self::EqRI => write!(f, "eqri"),
            Self::EqRR => write!(f, "eqrr"),
        }
    }
}
