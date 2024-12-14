use std::time::Instant;
use intcode_2019::{ parse_program, Machine };

fn main() {
    part1();
    part2();
}

#[allow(dead_code)]
const TEST: &str = include_str!("../../../data/2019/day02/test.txt");
const INPUT: &str = include_str!("../../../data/2019/day02/input.txt");

fn part1() {
    let start = Instant::now();
    let mut data = parse_program(INPUT, 256);
    restore_gravity_assist(&mut data);

    let mut machine = Machine::new(vec![]);
    while machine.step(&mut data).is_ok() {}

    let out = data[0];

    println!("Part 1: {out} {:?}", start.elapsed());
}

fn part2() {
    let start = Instant::now();
    let start_data = parse_program(INPUT, 256);

    let (noun, verb) = 'out: {
        for noun in 0..99 {
            for verb in 0..99 {
                let mut working_data = start_data.clone();
                restore_gravity_assist_custom(&mut working_data, noun, verb);

                let mut machine = Machine::new(vec![]);
                while machine.step(&mut working_data).is_ok() {}

                if working_data[0] == 19690720 {
                    break 'out (noun, verb);
                }
            }
        }

        panic!("No solution found");
    };

    let out = 100 * noun + verb;

    println!("Part 2: {out} {:?}", start.elapsed());
}

pub fn restore_gravity_assist(data: &mut [isize]) {
    data[1] = 12;
    data[2] = 2;
}

pub fn restore_gravity_assist_custom(data: &mut [isize], noun: isize, verb: isize) {
    data[1] = noun;
    data[2] = verb;
}
