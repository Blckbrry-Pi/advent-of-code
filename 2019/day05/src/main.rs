use std::time::Instant;
use intcode_2019::{ parse_program, Machine };

fn main() {
    part1();
    part2();
}

#[allow(dead_code)]
const TEST: &str = include_str!("../../../data/2019/day05/test.txt");
const INPUT: &str = include_str!("../../../data/2019/day05/input.txt");

fn part1() {
    let start = Instant::now();
    let mut data = parse_program(INPUT, 256);
    let mut machine = Machine::new(vec![1]);

    while machine.step(&mut data).is_ok() {}

    let out = machine.output.last().unwrap();

    println!("Part 1: {out} {:?}", start.elapsed());
}


fn part2() {
    let start = Instant::now();
    let mut data = parse_program(INPUT, 256);
    let mut machine = Machine::new(vec![5]);

    while machine.step(&mut data).is_ok() {}
    
    let out = machine.output.last().unwrap();

    println!("Part 2: {out} {:?}", start.elapsed());
}
