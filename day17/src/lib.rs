use std::fmt::Write;

aoc_tools::aoc_sol!(day17: part1, part2);

// Instruction { opcode: Bst, operand: Operand(4) }
// Instruction { opcode: Bxl, operand: Operand(3) }
// Instruction { opcode: Cdv, operand: Operand(5) }
// Instruction { opcode: Bxl, operand: Operand(5) }
// Instruction { opcode: Adv, operand: Operand(3) }
// Instruction { opcode: Bxc, operand: Operand(1) }
// Instruction { opcode: Out, operand: Operand(5) }
// Instruction { opcode: Jnz, operand: Operand(0) }

// while a != 0 {
//     b = a & 0b111;
//     b ^= 0b011;
//     c = a >> b;
//     b ^= 0b101;
//     a = a >> 3;
//     b = b ^ c
//     println(b & 0b111);
// }



pub fn part1(input: &str) -> String {
    let (mut machine, program) = parse_input(input);

    while let Some(instruction) = machine.decode(&program) {
        instruction.exec(&mut machine);
    }

    let first = machine.output[0];
    let mut output = format!("{first}");
    for &v in &machine.output[1..] {
        output.push_str(&format!(",{v}"));
    }
    output
}

pub fn part2(input: &str) -> isize {
    let (template, program) = parse_input(input);

    fn solve_recursive(template: &Machine, i: isize, program: &[u8]) -> Option<isize> {
        let mut machine = template.clone();
        machine.a = i;
        machine.run(program);

        let program_match_start = program.len() - machine.output.len();
        if machine.output == program[program_match_start..] {
            if program_match_start == 0 { return Some(i) }

            for new in 0..8 {
                if let Some(v) = solve_recursive(template, i * 8 + new, program) {
                    return Some(v);
                }
            }
        }

        None
    }

    for i in 0..8 {
        if let Some(v) = solve_recursive(&template, i, &program) {
            return v;
        }
    }

    panic!("No matches found")
}

fn parse_input(input: &str) -> (Machine, Vec<u8>) {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let (reg_a, registers) = registers.trim_start_matches("Register A: ").split_once('\n').unwrap();
    let (reg_b, reg_c) = registers.trim_start_matches("Register B: ").split_once('\n').unwrap();
    let reg_c = reg_c.trim_start_matches("Register C: ");

    let program = program.trim_start_matches("Program: ");

    let pc = 0;
    let a = reg_a.parse().unwrap();
    let b = reg_b.parse().unwrap();
    let c = reg_c.parse().unwrap();
    let output = vec![];
    let machine = Machine { pc, a, b, c, output };

    let program = program.split(',').map(|v| v.parse().unwrap()).collect();

    (machine, program)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
    pc: usize,
    a: isize,
    b: isize,
    c: isize,
    output: Vec<u8>,
}
impl Machine {
    pub fn decode(&self, data: &[u8]) -> Option<Instruction> {
        if self.pc > data.len() - 2 { return None }
        let opcode = match data[self.pc] {
            0 => Opcode::Adv,
            1 => Opcode::Bxl,
            2 => Opcode::Bst,
            3 => Opcode::Jnz,
            4 => Opcode::Bxc,
            5 => Opcode::Out,
            6 => Opcode::Bdv,
            7 => Opcode::Cdv,
            _ => panic!("Invalid opcode @ 0x{:x}: {}", self.pc, data[self.pc]),
        };
        let operand = Operand(data[self.pc + 1]);
        Some(Instruction { opcode, operand })
    }

    pub fn run(&mut self, program: &[u8]) {
        while let Some(instruction) = self.decode(&program) {
            instruction.exec(self);
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction { opcode: Opcode, operand: Operand }
impl Instruction {
    pub fn exec(&self, machine: &mut Machine) {
        match self.opcode {
            Opcode::Adv => {
                let combo = self.operand.get_combo(machine) as u32;
                machine.a = machine.a / 2_isize.pow(combo);
            },
            Opcode::Bdv => {
                let combo = self.operand.get_combo(machine) as u32;
                machine.b = machine.a / 2_isize.pow(combo);
            },
            Opcode::Cdv => {
                let combo = self.operand.get_combo(machine) as u32;
                machine.c = machine.a / 2_isize.pow(combo);
            },
            Opcode::Bxl => {
                let literal = self.operand.get_literal(machine);
                machine.b = machine.b ^ literal;
            },
            Opcode::Bst => {
                let combo = self.operand.get_combo(machine);
                machine.b = combo.rem_euclid(8);
            },
            Opcode::Bxc => {
                // let combo = self.operand.get_literal(machine);
                machine.b = machine.b ^ machine.c;
            },

            // Special instructions
            Opcode::Out => {
                let combo = self.operand.get_combo(machine);
                machine.output.push(combo.rem_euclid(8) as u8);
            },
            Opcode::Jnz => {
                let literal = self.operand.get_literal(machine);
                if machine.a != 0 {
                    machine.pc = literal as usize;
                    return;
                }
            }
        }
        machine.pc += 2;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Operand(u8);
impl Operand {
    pub fn get_combo(&self, machine: &Machine) -> isize {
        match self.0 {
            0..4 => self.0 as isize,
            4 => machine.a,
            5 => machine.b,
            6 => machine.c,
            _ => panic!("Invalid operand"),
        }
    }
    pub fn get_literal(&self, _machine: &Machine) -> isize {
        match self.0 {
            0..8 => self.0 as isize,
            _ => panic!("Invalid operand"),
        }
    }
}
