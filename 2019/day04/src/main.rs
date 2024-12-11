use std::time::Instant;

fn main() {
    part1();
    part2();
}

#[allow(dead_code)]
const TEST: &str = include_str!("../../../data/2019/day04/test.txt");
const INPUT: &str = include_str!("../../../data/2019/day04/input.txt");

fn part1() {
    let start = Instant::now();
    let (from, to) = parse_input(INPUT);

    let out = nondecreasing_number(6, 1)
        .filter(number_valid(from, to))
        .count();

    println!("Part 1: {out:?} {:?}", start.elapsed());
}

fn part2() {
    let start = Instant::now();
    let (from, to) = parse_input(INPUT);

    let out = nondecreasing_number(6, 1)
        .filter(number_valid_p2(from, to))
        .count();

    println!("Part 2: {out} {:?}", start.elapsed());
}


fn parse_input(input: &'static str) -> (usize, usize) {
    let (from, to) = input.split_once('-').unwrap();
    (from.parse().unwrap(), to.parse().unwrap())
}

pub fn nondecreasing_number(digits_left: u32, next_digit_at_least: usize) -> Box<dyn Iterator<Item = usize>> {
    if digits_left == 0 {
        Box::new([0].into_iter())
    } else {
        Box::new(
            (next_digit_at_least..10)
                .flat_map(
                    move |digit| nondecreasing_number(digits_left - 1, digit)
                        .map(move |v| v + digit * 10_usize.pow(digits_left - 1))
                )
        )
    }
}

pub fn number_valid(from: usize, to: usize) -> impl Fn(&usize) -> bool {
    move |&value| {
        if !(from <= value && value <= to) {
            return false;
        }
        for i in 0..5 {
            let digit_i = value / 10_usize.pow(i) % 10;
            let digit_ip1 = value / 10_usize.pow(i + 1) % 10;
            if digit_i == digit_ip1 {
                return true;
            }
        }
        false
    }
}

pub fn number_valid_p2(from: usize, to: usize) -> impl Fn(&usize) -> bool {
    move |&value| {
        if !(from <= value && value <= to) {
            return false;
        }
        for i in 0..5 {
            let digit_i = value / 10_usize.pow(i) % 10;
            let digit_ip1 = value / 10_usize.pow(i + 1) % 10;
            if digit_i != digit_ip1 { continue }
            let digit_im1 = value / 10_usize.pow(i.saturating_sub(1)) % 10;
            let digit_ip2 = value / 10_usize.pow(i.saturating_add(2)) % 10;

            if i > 0 && digit_im1 == digit_i { continue }
            if i < 4 && digit_ip2 == digit_i { continue }

            return true;
        }
        false
    }
}
