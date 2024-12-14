#![feature(buf_read_has_data_left)]

use std::time::Instant;
use intcode_2019::{ parse_program, Machine };

fn main() {
    part1();
}

#[allow(dead_code)]
const TEST: &str = include_str!("../../../data/2019/day25/test.txt");
const INPUT: &str = include_str!("../../../data/2019/day25/input.txt");

const PROGRAM_INPUT: &str = "
east
take jam
east
take fuel cell
west
south
take shell
north
west
south
west
take easter egg
north
east
take space heater
west
south
west
west
take monolith
south
west
north
take coin
south
east
north
west
take mug
north
";

fn part1() {
    let start = Instant::now();
    let mut data = parse_program(INPUT, 256);

    let mut commands: String = PROGRAM_INPUT.trim_start().to_string();

    let items = ["jam", "mug", "space heater", "fuel cell", "shell", "monolith", "easter egg", "coin"];
    for i in 0..2_usize.pow(items.len() as u32) {
        for bit in 0..items.len() {
            if (i >> bit) & 1 == 1 {
                commands.push_str("take ");
            } else {
                commands.push_str("drop ");
            }
            commands.push_str(items[bit]);
            commands.push('\n');
        }
        commands.push_str("inv\n");
        commands.push_str("north\n");
    }

    let mut machine = Machine::new_ascii(&commands);
    
    let mut lines = vec![];

    while machine.step(&mut data).is_ok() {
        if let Some(b'\n') = machine.output.last().map(|&v| v as u8) {
            let line: Vec<_> = machine.output.into_iter().map(|v| v as u8).collect();
            let line = String::from_utf8_lossy(&line).into_owned();
            if line.starts_with("\"Oh, hello!") {
                lines.push(line);
                break;
            } else {
                lines.push(line);
                machine.output = vec![];
            }
        }
    }

    let last_line = lines.last().unwrap();
    let (_, password) = last_line.split_once(" typing ").unwrap();
    let (password, _) = password.split_once(" on ").unwrap();

    println!("Part 1: {password} {:?}", start.elapsed());
}
