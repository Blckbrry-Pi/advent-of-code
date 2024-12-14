#![feature(buf_read_has_data_left)]

pub use instruction::Instruction;
pub use machine::Machine;

pub mod instruction;
pub mod machine;

pub fn parse_program(input: &'static str, extend: usize) -> Vec<isize> {
    let mut mem: Vec<_> = input.split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    mem.extend(std::iter::repeat(0).take(extend));

    mem
}
