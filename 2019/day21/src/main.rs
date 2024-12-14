use std::time::Instant;
use intcode_2019::{ parse_program, Machine };

fn main() {
    part1();
    part2();
}

#[allow(dead_code)]
const TEST: &str = include_str!("../../../data/2019/day21/test.txt");
const INPUT: &str = include_str!("../../../data/2019/day21/input.txt");

const SPRINGDROID_PROGRAM_P1: &str = r#"
NOT A J
NOT B T
AND T J
NOT C T
AND T J
NOT D T
AND T J
NOT A T
OR A T
AND A T
AND B T
AND C T
NOT T T
AND D T
OR T J
WALK
"#;

const SPRINGDROID_PROGRAM_P2: &str = r#"
NOT A J
NOT A T
OR A T
AND A T
AND B T
AND C T
NOT T T
AND D T
AND H T
OR T J
RUN
"#;

fn part1() {
    let start = Instant::now();
    let mut data = parse_program(INPUT, 256);

    let mut machine = Machine::new_ascii(SPRINGDROID_PROGRAM_P1.trim_start());

    while machine.step(&mut data).is_ok() {}
    if !machine.halt {
        println!("Encountered invalid instruction");
    }

    println!("Part 1: {} {:?}", machine.output.last().unwrap(), start.elapsed());
}


fn part2() {
    let start = Instant::now();
    let mut data = parse_program(INPUT, 256);

    let mut machine = Machine::new_ascii(SPRINGDROID_PROGRAM_P2.trim_start());

    while machine.step(&mut data).is_ok() {}
    if !machine.halt {
        println!("Encountered invalid instruction");
    }

    println!("Part 2: {} {:?}", machine.output.last().unwrap(), start.elapsed());
}
