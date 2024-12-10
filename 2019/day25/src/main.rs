#![feature(buf_read_has_data_left)]

use std::time::Instant;

use instruction::Instruction;
use machine::Machine;
mod instruction;
mod machine;

fn main() {
    part1();
    part2();
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
    let mut data = parse_input(INPUT);
    // data.extend([0]);

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
    let mut machine = Machine::new(commands.trim_start().as_bytes().iter().copied().map(|b| b as isize).collect());



    // let instructions = {
    //     let mut instructions = vec![];
    //     machine.pc = 2683 - 4 * 5;
    //     while machine.pc < data.len() - 4 {
    //         let Some(instruction) = Instruction::parse(&machine, &data) else {
    //             println!("Bad instruction: {}", machine.pc);
    //             break;
    //         };
    //         instructions.push((machine.pc, instruction));
    //         machine.pc += instruction.size();
    //     }
    //     instructions
    // };
    // for (addr, instruction) in instructions {
    //     println!("0x{addr:04X}: {instruction:?}");
    // }

    // let mut machine = Machine::new(vec![]);

    machine.run(
        &mut data,
        std::io::stdin(),
        std::io::stdout(),
    ).unwrap();

    println!("Part 1: {:?} {:?}", 0, start.elapsed());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IdleState {
    idle_read_count: u32,
}

fn part2() {
    let start = Instant::now();
    let data = parse_input(INPUT);

    let mut machines: Vec<_> = (0..50).map(|v| (
        Machine::new(vec![v]),
        data.clone(),
        IdleState { idle_read_count: 0 },
    )).collect();

    let mut seen_nat_packets = std::collections::HashSet::new();
    let mut nat_packet = None;

    loop {
        let mut packets = vec![];
        for (machine, data, idle_state) in machines.iter_mut() {
            if machine.input.is_empty() {
                machine.input.push(-1);
                idle_state.idle_read_count += 1;
            }

            if machine.halt { continue }

            if let Some(instruction) = Instruction::parse(&machine, &data) {
                instruction.exec(machine, data);
            }

            if !machine.output.is_empty() {
                idle_state.idle_read_count = 0;
            }
            if machine.output.len() == 3 {
                packets.push((machine.output[0], machine.output[1], machine.output[2]));
                machine.output.clear();
            }
        }
        for (addr, x, y) in packets {
            if addr == 255 {
                nat_packet = Some((x, y));
            } else {
                machines[addr as usize].0.input.push(x);
                machines[addr as usize].0.input.push(y);
                machines[addr as usize].2.idle_read_count = 0;
            }
        }

        if machines.iter().all(|(_, _, idle)| idle.idle_read_count > 5) {
            if let Some(nat_packet) = nat_packet {
                if seen_nat_packets.contains(&nat_packet) {
                    break;
                }
                machines[0].0.input.push(nat_packet.0);
                machines[0].0.input.push(nat_packet.1);
                machines[0].2.idle_read_count = 0;

                seen_nat_packets.insert(nat_packet);
            } else {
                break
            }
            nat_packet = None;
        }
    }

    println!("Part 2: {:?} {:?}", nat_packet.unwrap().1, start.elapsed());
}


fn parse_input(input: &'static str) -> Vec<isize> {
    let mut mem: Vec<_> = input.split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    mem.extend([0; 256]);

    mem
}
