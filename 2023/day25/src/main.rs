use system::System;
use wire::Wire;

mod component;
mod wire;
mod system;

fn main() {
    part1();
}

const PART_1: &str = include_str!("../../../data/2023/day25/p1.txt");
// const PART_2: &str = include_str!("../../data/day25/p2.txt");

fn part1() {
    let system = parse_input(PART_1);

    let from = system.components().next().unwrap();
    let path = 'path: {
        for to in system.components() {
            if from == to { continue }
    
            let (needed, ignored) = system.wire_disconnects_needed(from, to);
            if needed <= 3 {
                break 'path ignored;
            }
        }

        panic!("No path found");
    };

    let path: Vec<_> = path.into_iter().collect();

    let wires = 'wires: {
        for i in 0..path.len() {
            for j in i+1..path.len() {
                for k in j+1..path.len() {
                    let ignored = vec![path[i], path[j], path[k]];
                    if system.partitions(ignored).is_some() {
                        break 'wires (path[i], path[j], path[k]);
                    }
                }
            }
        }
        unreachable!("No group of 3 wires found");
    };


    let [a, b] = system.partitions(vec![wires.0, wires.1, wires.2]).unwrap();
    println!("Part 1: {}", a.len() * b.len());
}


fn parse_input(input: &'static str) -> System {
    let mut system = System::new();

    input.split('\n')
        .for_each(|line| {
            let (from, tos) = line.split_once(':').unwrap();
            let from = from.trim();
            for to in tos.split(' ').filter(|t| !t.is_empty()) {
                system.add_wire(Wire::new(from, to.trim()));
            }
        });

    system
}
