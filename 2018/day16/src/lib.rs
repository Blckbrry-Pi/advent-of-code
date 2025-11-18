use wrist_device::{ Instruction, Opcode, Params, Program, RegVal };
aoc_tools::aoc_sol!(day16 2018: part1, part2);

type State = wrist_device::State<4>;

pub fn part1(input: &str) -> i64 {
    let (samples, _) = parse_input(input);
    let mut triple_ambiguous = 0;
    for (before, (_, params), after) in samples {
        let mut matching_opcodes = 0;
        for opcode in Opcode::ALL {
            let mut state = before;
            Instruction(opcode, params).apply(&mut state);
            if state == after { matching_opcodes += 1; }
        }
        if matching_opcodes >= 3 {
            triple_ambiguous += 1;
        }
    }
    triple_ambiguous
}

pub fn part2(input: &str) -> RegVal {
    let (samples, test_program) = parse_input(input);
    let mut known = [None; 16];
    let mut possibilities: Vec<HashSet<_>> = (0..16).map(|_| Opcode::ALL.into_iter().collect()).collect();
    for (before, (opcode_number, params), after) in samples {
        if known[opcode_number as usize].is_some() { continue }
        let mut matching_opcodes = HashSet::new();
        for opcode in Opcode::ALL {
            let mut state = before;
            Instruction(opcode, params).apply(&mut state);
            if state == after { matching_opcodes.insert(opcode); }
        }
        possibilities[opcode_number as usize].retain(|v| matching_opcodes.contains(v));
        if possibilities[opcode_number as usize].len() == 1 {
            let remove = *possibilities[opcode_number as usize].iter().next().unwrap();
            for i in 0..16 {
                if i == opcode_number { continue }
                possibilities[i as usize].remove(&remove);
            }
            known[opcode_number as usize] = Some(remove);
        }
    }

    let mut state = State::zeroed();
    let instructions: Vec<_> = test_program.into_iter()
        .map(|(opcode_num, params)| Instruction(known[opcode_num as usize].unwrap(), params))
        .collect();

    let program = Program(
        instructions,
        vec![],
    );
    program.execute(&mut state);

    state.0[0]
}

fn parse_input(input: &str) -> (Vec<(State, (u8, Params), State)>, Vec<(u8, Params)>) {
    fn parse_quadrouple(s: &str) -> (u8, Params) {
        let (inst, params) = s.split_once(' ').unwrap();
        let inst = inst.parse::<u8>().unwrap();
        (inst, params.parse().unwrap())
    }

    let (samples, test_program) = input.trim().split_once("\n\n\n\n").unwrap();
    let samples = samples.split("\n\n")
        .map(|sample| {
            let (before, rest) = sample.split_once('\n').unwrap();
            let (instruction, after) = rest.split_once('\n').unwrap();
            let before = before.strip_prefix("Before: ").unwrap();
            let after  = after .strip_prefix("After:  ").unwrap();
            (before.parse::<State>().unwrap(), parse_quadrouple(instruction), after.parse::<State>().unwrap())
        })
        .collect();
    let test_program = test_program.lines().map(parse_quadrouple).collect();
    (samples, test_program)
}
