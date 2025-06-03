use std::time::Instant;
use intcode_2019::{ parse_program, Machine };

fn main() {
    part1();
    part2();
}

fn perm5() -> Vec<[u8; 5]> {
    fn perm5_recursive(output: &mut Vec<[u8; 5]>, curr: &mut [u8; 5], i: u8) {
        for j in 0..5 {
            if curr[0..i as usize].contains(&j) { continue }
            curr[i as usize] = j;
            if i == 4 {
                output.push(*curr);
            } else {
                perm5_recursive(output, curr, i + 1);
            }
        }
    }
    let mut output = Vec::with_capacity(120);
    perm5_recursive(&mut output, &mut [0; 5], 0);
    output
}

fn run_value(program: &[isize], phase_settings: [u8; 5]) -> isize {
    let mut machines: [_; 5] = std::array::from_fn(|i| (
        Machine::new(vec![phase_settings[i] as isize]),
        program.to_vec(),
    ));

    let mut next = 0;
    for step in 0.. {
        let i = step % 5;
        let (machine, data) = machines.get_mut(i).unwrap();
        machine.input.0.push(next);
        while machine.output.len() <= step / 5 {
            if machine.step(data).is_err() {
                return next;
            }
        }
        next = *machine.output.last().unwrap();
    }
    unreachable!("We somehow counted to 2^64");
}

#[allow(dead_code)]
const TEST: &str = include_str!("../../../data/2019/day07/test.txt");
const INPUT: &str = include_str!("../../../data/2019/day07/input.txt");

fn part1() {
    let start = Instant::now();
    let data = parse_program(INPUT, 256);

    let mut max_output = 0;
    for perm in perm5() {
        let output = run_value(&data, perm);
        max_output = max_output.max(output);
    }

    println!("Part 1: {max_output} {:?}", start.elapsed());
}

fn part2() {
    let start = Instant::now();
    let data = parse_program(INPUT, 256);

    let mut max_output = 0;
    for perm in perm5() {
        let output = run_value(&data, std::array::from_fn(|i| perm[i] + 5));
        max_output = max_output.max(output);
    }

    println!("Part 2: {max_output} {:?}", start.elapsed());
}
