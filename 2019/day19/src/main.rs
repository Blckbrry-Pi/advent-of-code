use std::time::Instant;
use intcode_2019::{ parse_program, Machine };

fn main() {
    part1();
    part2();
}

#[allow(dead_code)]
const TEST: &str = include_str!("../../../data/2019/day19/test.txt");
const INPUT: &str = include_str!("../../../data/2019/day19/input.txt");

fn part1() {
    let start = Instant::now();
    let template_data = parse_program(INPUT, 256);

    let mut count = 0;
    for x in 0..50 {
        for y in 0..50 {
            let mut data = template_data.clone();
            let mut machine = Machine::new(vec![x, y]);

            while machine.step(&mut data).is_ok() {}
            if !machine.halt {
                println!("Encountered invalid instruction");
            }
            if *machine.output.last().unwrap() == 1 {
                count += 1;
            }
        }
    }

    println!("Part 1: {count} {:?}", start.elapsed());
}


fn part2() {
    let start = Instant::now();
    let template_data = parse_input(INPUT);

    let mut beam = vec![[false; 1500]; 1500];

    let mut leftmost = 0;
    let mut width = 0;
    let mut starting_candidate = -1;
    for y in 0..1500 {
        let mut new_leftmost = isize::MAX;
        let mut new_width = -1;
        for x in leftmost..1500 {
            if new_leftmost != isize::MAX && x - new_leftmost < width {
                beam[y as usize][x as usize] = true;
                continue
            }
            let mut data = template_data.clone();
            let mut machine = Machine::new(vec![x, y]);

            while machine.step(&mut data).is_ok() {}
            if !machine.halt {
                println!("Encountered invalid instruction");
            }
            if *machine.output.last().unwrap() == 1 {
                new_leftmost = new_leftmost.min(x);
                beam[y as usize][x as usize] = true;
            } else if new_leftmost != isize::MAX {
                new_width = x - new_leftmost;
                break
            }
        }
        if new_leftmost != isize::MAX {
            leftmost = new_leftmost;
        }
        if new_width != -1 {
            width = new_width;
            if starting_candidate == -1 && width >= 100 {
                starting_candidate = y;
            }
        }
    }

    let mut working = vec![];
    let mut leftmost = 0;
    for y in starting_candidate as usize..beam.len() - 100 {
        let mut new_leftmost = usize::MAX;
        for x in leftmost..beam[y].len() - 100 {
            if beam[y][x] {
                new_leftmost = new_leftmost.min(x);
            }
            let works = 'works: {
                // Just check the corners
                for dy in [0, 99] {
                    for dx in [0, 99] {
                        if !beam[y+dy][x+dx] {
                            break 'works false;
                        }
                    }
                }
                true
            };
            if works {
                working.push((x, y));
            }
        }
        if new_leftmost != usize::MAX {
            leftmost = new_leftmost;
        }
    }

    let out = working[0].0 * 10000 + working[0].1;

    println!("Part 2: {out} {:?}", start.elapsed());
}


fn parse_input(input: &'static str) -> Vec<isize> {
    let mut mem: Vec<_> = input.split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    mem.extend([0; 256]);

    mem
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Instruction {
//     OpCode1 { a: Addr, b: Addr, to: Addr },
//     OpCode2 { a: Addr, b: Addr, to: Addr },
//     OpCode3 { to: Addr },
//     OpCode4 { output: Addr },
//     OpCode5 { cond: Addr, to: Addr },
//     OpCode6 { cond: Addr, to: Addr },
//     OpCode7 { l: Addr, r: Addr, to: Addr },
//     OpCode8 { l: Addr, r: Addr, to: Addr },
//     OpCode9 { l: Addr, r: Addr, to: Addr },
//     OpCode99,
// }

// impl Instruction {
//     pub fn parse(data: &[isize], idx: usize) -> Option<Self> {
//         if idx >= data.len() { return None }
//         let opcode = data[idx];
//         println!("opcode: {opcode}");
//         let mode_0 = Mode::from_int(opcode / 100 % 10);
//         let mode_1 = Mode::from_int(opcode / 1000 % 10);
//         let mode_2 = Mode::from_int(opcode / 10000 % 10);
//         match data[idx] % 100 {
//             1 => Some(Self::OpCode1 {
//                 a: Addr { mode: mode_0, arg: data[idx+1] },
//                 b: Addr { mode: mode_1, arg: data[idx+2] },
//                 to: Addr { mode: mode_2, arg: data[idx+3] },
//             }),
//             2 => Some(Self::OpCode2 {
//                 a: Addr { mode: mode_0, arg: data[idx+1] },
//                 b: Addr { mode: mode_1, arg: data[idx+2] },
//                 to: Addr { mode: mode_2, arg: data[idx+3] },
//             }),
//             3 => Some(Self::OpCode3 { to: Addr { mode: mode_0, arg: data[idx+1] } }),
//             4 => Some(Self::OpCode4 { output: Addr { mode: mode_0, arg: data[idx+1] } }),
//             5 => Some(Self::OpCode5 {
//                 cond: Addr { mode: mode_0, arg: data[idx+1] },
//                 to: Addr { mode: mode_1, arg: data[idx+2] },
//             }),
//             6 => Some(Self::OpCode6 {
//                 cond: Addr { mode: mode_0, arg: data[idx+1] },
//                 to: Addr { mode: mode_1, arg: data[idx+2] },
//             }),
//             7 => Some(Self::OpCode7 {
//                 l: Addr { mode: mode_0, arg: data[idx+1] },
//                 r: Addr { mode: mode_1, arg: data[idx+2] },
//                 to: Addr { mode: mode_2, arg: data[idx+3] },
//             }),
//             8 => Some(Self::OpCode8 {
//                 l: Addr { mode: mode_0, arg: data[idx+1] },
//                 r: Addr { mode: mode_1, arg: data[idx+2] },
//                 to: Addr { mode: mode_2, arg: data[idx+3] },
//             }),
//             99 => Some(Self::OpCode99),
//             _ => None,
//         }
//     }

//     pub fn exec(
//         &self,
//         pc: usize,
//         data: &mut [isize],
//         input: &mut impl Iterator<Item = isize>,
//     ) -> (usize, Option<isize>, bool) {
//         match *self {
//             Self::OpCode1 { a, b, to } => {
//                 to.set(a.get(data) + b.get(data), data);
//                 (pc+4, None, false)
//             },
//             Self::OpCode2 { a, b, to } => {
//                 to.set(a.get(data) * b.get(data), data);
//                 (pc+4, None, false)
//             },
//             Self::OpCode3 { to } => {
//                 to.set(input.next().unwrap(), data);
//                 (pc+2, None, false)
//             },
//             Self::OpCode4 { output } => {
//                 (pc+2, Some(output.get(data)), false)
//             },
//             Self::OpCode5 { cond, to } => {
//                 let new_pc = if cond.get(data) != 0 {
//                     to.get(data) as usize
//                 } else {
//                     pc+3
//                 };
//                 (new_pc, None, false)
//             },
//             Self::OpCode6 { cond, to } => {
//                 let new_pc = if cond.get(data) == 0 {
//                     to.get(data) as usize
//                 } else {
//                     pc+3
//                 };
//                 (new_pc, None, false)
//             },
//             Self::OpCode7 { l, r, to } => {
//                 let store = l.get(data) < r.get(data);
//                 let store = if store { 1 } else { 0 };
//                 to.set(store, data);
//                 (pc+4, None, false)
//             },
//             Self::OpCode8 { l, r, to } => {
//                 let store = l.get(data) == r.get(data);
//                 let store = if store { 1 } else { 0 };
//                 to.set(store, data);
//                 (pc+4, None, false)
//             },
//             Self::OpCode99 => (pc+1, None, true),
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// enum Mode {
//     Position,
//     Immediate,
// }
// impl Mode {
//     fn from_int(int: isize) -> Self {
//         match int {
//             0 => Self::Position,
//             1 => Self::Immediate,
//             _ => panic!("Invalid integer"),
//         }
//     }
//     pub fn get(&self, arg: isize, data: &[isize]) -> isize {
//         match self {
//             Self::Position => { data[arg as usize] },
//             Self::Immediate => arg,
//         }
//     }
//     pub fn set(&self, arg: isize, val: isize, data: &mut [isize]) {
//         match self {
//             Self::Position => { data[arg as usize] = val; },
//             Self::Immediate => unimplemented!("Cannot set an immediate value"),
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Addr { arg: isize, mode: Mode }

// impl Addr {
//     pub fn get(&self, data: &[isize]) -> isize {
//         self.mode.get(self.arg, data)
//     }
//     pub fn set(&self, val: isize, data: &mut [isize]) {
//         self.mode.set(self.arg, val, data)
//     }
// }

// pub fn restore_gravity_assist(data: &mut [isize]) {
//     data[1] = 12;
//     data[2] = 2;
// }

// pub fn restore_gravity_assist_custom(data: &mut [isize], noun: isize, verb: isize) {
//     data[1] = noun;
//     data[2] = verb;
// }
