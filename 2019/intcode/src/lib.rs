#![feature(buf_read_has_data_left)]

pub use instruction::Instruction;
pub use machine::Machine;

pub mod instruction;
pub mod machine;

pub fn parse_program(input: &str, extend: usize) -> Vec<isize> {
    let mut mem: Vec<_> = input.trim_ascii()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    mem.extend(std::iter::repeat(0).take(extend));

    mem
}

pub fn disasm(prg: &[isize]) {
    let mut machine = Machine::new(vec![]);
    let mut seen = vec![false; prg.len()];
    let mut starts = vec![0x01e0, 0];
    let mut parts = vec![];
    while let Some(new_pc) = starts.pop() {
        machine.pc = new_pc;

        let mut new_part = vec![];
        while let Some(instruction) = Instruction::parse(&machine, prg) {
            if seen[machine.pc] { break }
            // println!("{instruction:?}");
            new_part.push(instruction);
            for i in machine.pc..machine.pc + instruction.size() {
                seen[i] = true;
            }
            if let Instruction::Jif { addr, .. } | Instruction::Jit { addr, .. } = instruction {
                starts.push(addr.get(prg, &machine) as usize);
            }
            machine.pc += instruction.size();
            if seen[machine.pc] { break }
            if starts.contains(&machine.pc) { break }
        }
        if new_part.len() > 0 {
            parts.push((new_pc, new_part));
        }
    }
    parts.sort_by_key(|p| p.0);
    for (mut offset, part) in parts {
        println!("0x{offset:04x}:");
        for instruction in part {
            println!("{offset:03x}     {instruction}");
            offset += instruction.size();
        }
        println!();
    }
}
