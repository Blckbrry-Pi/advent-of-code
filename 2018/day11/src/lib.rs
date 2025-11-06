use std::collections::VecDeque;

aoc_tools::aoc_sol!(day11 2018: part1, part2);

pub fn vals_y(x: u32, s: u32) -> impl Iterator<Item = i32> {
    let rack_id = x + 10;
    let offset = rack_id * s % 1000;
    let rack_id_sq = rack_id * rack_id % 1000;
    (1..)
        .map(move |y| y % 1000)
        .map(move |y| rack_id_sq * y + offset)
        .map(|pow_lev| ((pow_lev / 100) % 10) as i32 - 5)
}

pub fn sums_y(x: u32, s: u32, d: u32) -> impl Iterator<Item = i32> {
    let mut iter = vals_y(x, s);
    let mut sum = 0;
    let mut cached = VecDeque::with_capacity(d as usize);
    for _ in 1..d {
        let next_val = iter.next().unwrap();
        cached.push_back(next_val);
        sum += next_val;
    }
    iter.map(move |v| {
        let out = sum + v;
        cached.push_back(v);
        sum = out - cached.pop_front().unwrap();
        out
    })
}

pub fn max_xy(s: u32, grid_size: u32, d: u32) -> (u32, u32, i32) {
    let mut sums = vec![(
        VecDeque::with_capacity(d as usize),
        0,
    ); grid_size as usize - d as usize + 1];

    for x in 1..d {
        for (y, s) in sums_y(x, s, d).take(grid_size as usize - d as usize + 1).enumerate() {
            sums[y].0.push_back(s);
            sums[y].1 += s;
        }
    }
    (1..).map(move |x|
        sums_y(x + d - 1, s, d)
            .take(grid_size as usize - d as usize + 1)
            .enumerate()
            .map(|(y, s)| {
                let out = sums[y].1 + s;
                sums[y].0.push_back(s);
                sums[y].1 = out - sums[y].0.pop_front().unwrap();
                (x, y as u32 + 1, out)
            })
            .max_by_key(|(_, _, s)| *s)
            .unwrap()
    )
    .take((grid_size - d + 1) as usize)
    .max_by_key(|(_, _, s)| *s)
    .unwrap()
}

const GRID_DIM: usize = 300;

pub fn part1(input: &str) -> String {
    let serial = parse_input(input);
    let (x, y, _) = max_xy(serial, GRID_DIM as u32, 3);
    // println!("{:?}", );
    format!("{x},{y}")
}

pub fn part2(input: &str) -> String {
    let serial = parse_input(input);
    let ((x, y, _), d) = (1..=GRID_DIM)
        .map(|d| (max_xy(serial, GRID_DIM as u32, d as u32), d))
        .max_by_key(|&((_, _, s), _)| s)
        .unwrap();
    format!("{x},{y},{d}")
}

fn parse_input(input: &str) -> u32 {
    input.trim().parse::<u32>().unwrap()
}
