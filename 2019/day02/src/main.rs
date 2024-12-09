use std::time::Instant;

fn main() {
    part1();
    part2();
}

#[allow(dead_code)]
const TEST: &str = include_str!("../../../data/2019/day05/test.txt");
const INPUT: &str = include_str!("../../../data/2019/day05/input.txt");

fn part1() {
    let start = Instant::now();
    let mut data = parse_input(INPUT);

    restore_gravity_assist(&mut data);
    println!("data: {data:?}");

    let mut pc = 0;

    while let Some((instruction, new_pc)) = Instruction::parse(&data, pc) {
        if instruction.exec(&mut data) { break }
        pc = new_pc;
    }

    let out = data[0];

    println!("Part 1: {out} {:?}", start.elapsed());
}

fn part2() {
    let start = Instant::now();
    let start_data = parse_input(INPUT);

    let (noun, verb) = 'out: {
        for noun in 0..99 {
            for verb in 0..99 {
                let mut working_data = start_data.clone();
                restore_gravity_assist_custom(&mut working_data, noun, verb);

                let mut pc = 0;
            
                while let Some((instruction, new_pc)) = Instruction::parse(&working_data, pc) {
                    if instruction.exec(&mut working_data) { break }
                    pc = new_pc;
                }
            
                let out = working_data[0];
                
                if out == 19690720 {
                    break 'out (noun, verb);
                }
            }
        }

        panic!("No solution found");
    };

    let out = 100 * noun + verb;

    println!("Part 2: {out} {:?}", start.elapsed());
}


fn parse_input(input: &'static str) -> Vec<usize> {
    input.split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    OpCode1 { a: usize, b: usize, to: usize },
    OpCode2 { a: usize, b: usize, to: usize },
    OpCode99,
}

impl Instruction {
    pub fn parse(data: &[usize], idx: usize) -> Option<(Self, usize)> {
        if idx >= data.len() { return None }
        match data[idx] {
            1 => Some((Self::OpCode1 { a: data[idx+1], b: data[idx+2], to: data[idx+3] }, idx+4)),
            2 => Some((Self::OpCode2 { a: data[idx+1], b: data[idx+2], to: data[idx+3] }, idx+4)),
            99 => Some((Self::OpCode99, idx+1)),
            _ => None,
        }
    }

    pub fn exec(&self, data: &mut [usize]) -> bool {
        match *self {
            Self::OpCode1 { a, b, to } => {
                data[to as usize] = data[a as usize] + data[b as usize];
                false
            },
            Self::OpCode2 { a, b, to } => {
                data[to as usize] = data[a as usize] * data[b as usize];
                false
            },
            Self::OpCode99 => true,
        }
    }
}

pub fn restore_gravity_assist(data: &mut [usize]) {
    data[1] = 12;
    data[2] = 2;
}

pub fn restore_gravity_assist_custom(data: &mut [usize], noun: usize, verb: usize) {
    data[1] = noun;
    data[2] = verb;
}
