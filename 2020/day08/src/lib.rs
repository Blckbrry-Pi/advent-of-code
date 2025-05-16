use std::str::FromStr;

aoc_tools::aoc_sol!(day08 2020: part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Jmp(i32),
    Nop(i32),
    Acc(i32),
}
impl Instruction {
    pub fn swapped(self) -> Option<Self> {
        match self {
            Self::Acc(_) => None,
            Self::Nop(op) => Some(Self::Jmp(op)),
            Self::Jmp(op) => Some(Self::Nop(op)),
        }
    }
}
impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 5 {
            return Err("Instructions are at least 5 chars long".to_string());
        }
        let Some((opcode, operand)) = s.split_at_checked(4) else {
            return Err("All instructions are ascii".to_string());
        };
        let operand = operand.parse::<i32>().map_err(|e| e.to_string())?;
        let output = match opcode {
            "acc " => Self::Acc(operand),
            "nop " => Self::Nop(operand),
            "jmp " => Self::Jmp(operand),
            opcode => return Err(format!("Invalid opcode {:?}", opcode.trim()))
        };

        Ok(output)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Processor {
    acc: i32,
    pc: usize,
}
impl Processor {
    pub fn exec(&mut self, instructions: &[Instruction], sub: Option<(usize, Instruction)>) -> bool {
        let mut seen = HashSet::new();
        while self.pc < instructions.len() {
            if !seen.insert(self.pc) { return false; }
            let mut instr = instructions[self.pc];
            if let Some((pc, new_instr)) = sub {
                if self.pc == pc {
                    instr = new_instr;
                }
            }
            match instr {
                Instruction::Acc(op) => self.acc += op,
                Instruction::Nop(_) => (),
                Instruction::Jmp(op) => self.pc = (self.pc as i64 + op as i64 - 1) as usize,
            }
            self.pc += 1;
        }
        true
    }
}

pub fn part1(input: &str) -> i32 {
    let instructions = parse_input(input);
    let mut proc = Processor::default();
    proc.exec(&instructions, None);
    proc.acc
}

pub fn part2(input: &str) -> i32 {
    let instructions = parse_input(input);
    for i in 0..instructions.len() {
        let Some(new_inst) = instructions[i].swapped() else { continue };
        let mut proc = Processor::default();
        if proc.exec(&instructions, Some((i, new_inst))) {
            return proc.acc;
        }
    }
    panic!("Yeah idk how to help you, there doesn't seem to be a single instruction")
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}
