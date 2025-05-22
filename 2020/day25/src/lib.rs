aoc_tools::aoc_sol!(day25 2020: part1);

const fn pow_mod(b: u64, e: u64, m: u64) -> u64 {
    if e == 0 { 1 }
    else if e == 1 { b % m }
    else {
        let half = pow_mod(b, e/2, m);
        let double_half = half * half % m;
        double_half * (if e % 2 == 0 { 1 } else { b }) % m
    }
}

const MODULUS: u64 = 20201227;
const PUBLIC_KEY_GEN_SUBJECT: u64 = 7;

// Realistically, I should use big-step little-step but I'm kinda lazy ngl
pub fn discrete_log(b: u64, b_to_x: u64, m: u64) -> u64 {
    let mut curr = 1;
    for i in 0..m-1 {
        if curr == b_to_x { return i; }
        curr = curr * b % m;
    }
    panic!(":(")
}

pub fn part1(input: &str) -> u64 {
    let (card_pub, lock_pub) = parse_input(input);

    let card_loop_count = discrete_log(PUBLIC_KEY_GEN_SUBJECT, card_pub, MODULUS);
    // let lock_loop_count = discrete_log(PUBLIC_KEY_GEN_SUBJECT, lock_pub, MODULUS);

    // pow_mod(7, card_loop_count * lock_loop_count, MODULUS)
    pow_mod(lock_pub, card_loop_count, MODULUS)
}

pub fn part2(_input: &str) -> () {}

fn parse_input(input: &str) -> (u64, u64) {
    let (card_pub_key, lock_pub_key) = input.trim().split_once('\n').unwrap();
    (card_pub_key.parse().unwrap(), lock_pub_key.parse().unwrap())
}
