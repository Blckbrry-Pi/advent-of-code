use wrist_device::{Program, RegVal};

aoc_tools::aoc_sol!(day21 2018: part1, part2);

pub fn program_into_match_iter(p: &Program) -> impl Iterator<Item = RegVal> {
    let outer_v3_start = p.0[0x07].1.a as RegVal;
    let inner_v3_mul = p.0[0x0b].1.b as RegVal;
    let mut v3 = 0;
    std::iter::repeat(()).map(move |_| {
        let mut v5 = v3 | 0x10000;
        v3 = outer_v3_start;

        while v5 > 0 {
            v3 += v5 & 0xff;
            v3 &= 0xFFFFFF;

            v3 *= inner_v3_mul;
            v3 &= 0xFFFFFF;

            v5 >>= 8;
        }
        v3
    })
}

// This program functions as a rng machine with a right shift method that is so
// slow that I again could not get the executor to run fast enough (oh well)
//
// I disassembled, decompiled, and copied the logic of the program to the above
// function, pulling the user-specifc numbers from the input program
// I guess that's kind of cheating, but I Don't Care™ :3

pub fn part1(input: &str) -> RegVal {
    let program = parse_input(input);
    program_into_match_iter(&program).next().unwrap()
}

pub fn part2(input: &str) -> RegVal {
    let program = parse_input(input);
    let mut seen = HashSet::new();
    let mut last_good = 0;
    for halting_value in program_into_match_iter(&program) {
        if !seen.insert(halting_value) { break }
        last_good = halting_value;
    }
    last_good
}

fn parse_input(input: &str) -> Program {
    input.parse().unwrap()
}
