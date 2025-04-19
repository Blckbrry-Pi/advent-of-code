#![feature(debug_closure_helpers)]
#![feature(btree_extract_if)]

use std::{collections::HashSet, fmt::Debug};


fn main() {
    part1();
    part2();
}

#[allow(dead_code)]
const TEST: &str = include_str!("../../../data/2022/day18/test.txt");
const INPUT: &str = include_str!("../../../data/2022/day18/input.txt");

fn part1() {
    let cubes = parse_input(INPUT);
    let mut surface_area = 0;
    for cube in cubes.iter().copied() {
        let test_locations = [
            cube.xm1(),
            cube.xp1(),
            cube.ym1(),
            cube.yp1(),
            cube.zm1(),
            cube.zp1(),
        ];
        
        for cube in test_locations {
            if !cubes.contains(&cube) {
                surface_area += 1;
            }
        }
    }
    println!("Part 1: {surface_area}");
}

fn part2() {
    let cubes = parse_input(INPUT);

    // We know that all of the spaces bordering the cubes will be within a
    // certain bounding box.
    // This calculates a good enough approximation of that box
    let (max_x, max_y, max_z) = cubes
        .iter()
        .fold((0, 0, 0), |(max_x, max_y, max_z), cube| {
            (
                max_x.max(cube.x),
                max_y.max(cube.y),
                max_z.max(cube.z),
            )
        });
    let (max_x, max_y, max_z) = (max_x + 1, max_y + 1, max_z + 1);
    let bounding_volume = max_x as usize * max_y as usize * max_z as usize;
    
    // Queue contains the cubes that are reachable and maybe haven't been checked
    // Seen contains the cubes that have been checked
    let mut queue = vec![Cube { x: 0, y: 0, z: 0 }];
    let mut seen = HashSet::with_capacity(bounding_volume - cubes.len());
    let mut exterior_surface_area = 0;
    while let Some(next) = queue.pop() {
        if seen.contains(&next) { continue }
        seen.insert(next);
        let test_cubes = [
            (next.x > 0).then(|| next.xm1()),
            (next.x < max_x).then(|| next.xp1()),
            (next.y > 0).then(|| next.ym1()),
            (next.y < max_y).then(|| next.yp1()),
            (next.z > 0).then(|| next.zm1()),
            (next.z < max_z).then(|| next.zp1()),
        ];
        for test in test_cubes {
            let Some(test) = test else { continue };
            if cubes.contains(&test) {
                exterior_surface_area += 1;
            } else {
                queue.push(test);
            }
        }
    }

    println!("Part 2: {exterior_surface_area}");
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Cube { x: u8, y: u8, z: u8 }
impl Cube {
    fn xm1(self) -> Self { Self { x: self.x - 1, ..self } }
    fn xp1(self) -> Self { Self { x: self.x + 1, ..self } }
    fn ym1(self) -> Self { Self { y: self.y - 1, ..self } }
    fn yp1(self) -> Self { Self { y: self.y + 1, ..self } }
    fn zm1(self) -> Self { Self { z: self.z - 1, ..self } }
    fn zp1(self) -> Self { Self { z: self.z + 1, ..self } }
}
impl Debug for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}


fn parse_input(input: &'static str) -> HashSet<Cube> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (x, l) = l.split_once(',').unwrap();
            let (y, z) = l.split_once(',').unwrap();
            let (x, y, z) = (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap());
            Cube { x, y, z }
        })
        .map(|Cube { x, y, z }| Cube { x: x + 1, y: y + 1, z: z + 1 })
        .collect()
}
