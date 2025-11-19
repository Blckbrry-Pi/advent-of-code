use wrist_device::{Instruction, Opcode, Params, Program, RegVal, State};

aoc_tools::aoc_sol!(day19 2018: part1, part2);

pub fn sum_of_factors(n: RegVal) -> RegVal {
    let mut sum: RegVal = 0;

    for i in 1..=n.isqrt() {
        if n % i == 0 {
            sum += i;
            sum += n / i;
        }
    }
    sum
}

// After decompiling the program, it became clear that it was just
// calculating (very inefficiently, mind you) the sum of the factors of a
// number. This is kind of a hack, but I couldn't figure out how to
// effectively enough optimize the program executor to use the original
// program.
//
// I just wrote this kinda cheat-ey solution :)

pub fn part1(input: &str) -> RegVal {
    let mut program = parse_input(input);
    let mut state = State::<6>::zeroed();

    // Set program to halt immediately after calculating the target value
    program.0[1] = Instruction(Opcode::SetI, Params { a: 255, b: 0, c: program.get_ip().unwrap() });

    program.execute(&mut state);
    sum_of_factors(state.0[5])
}

pub fn part2(input: &str) -> RegVal {
    let mut program = parse_input(input);
    let mut state = State::<6>::zeroed();
    state.0[0] = 1;

    // Set program to halt immediately after calculating the target value
    program.0[1] = Instruction(Opcode::SetI, Params { a: 255, b: 0, c: program.get_ip().unwrap() });

    program.execute(&mut state);
    sum_of_factors(state.0[5])
}

fn parse_input(input: &str) -> Program {
    input.parse().unwrap()
}
