#![feature(isqrt)]

use num_bigint::BigInt;
use projectile::{Projectile, Vec3};

mod projectile;

fn main() {
    part2();
}

const TEST: &str = include_str!("../../../data/2023/day24/test.txt");
const PART_2: &str = include_str!("../../../data/2023/day24/p2.txt");

fn part2() {
    let hailstones = parse_input(PART_2);

    let a = hailstones[0];
    let b = hailstones[1];
    let c = hailstones[2];
    let d = hailstones[3];

    let pa_m_pc = a.pos() - c.pos();
    let abc_numer_coeff_a0 = a.vel().cross(c.vel()).dot(pa_m_pc); // (va x vc) • (pa - pc)
    let abc_numer_coeff_a1 = (a.pos() - b.pos()).cross(c.vel()).dot(a.vel()); // ((pa - pb) x vc) • va
    let num_coeff_c = abc_numer_coeff_a0 + abc_numer_coeff_a1;
    let num_const_c = (a.pos() - b.pos()).cross(c.vel()).dot(pa_m_pc); // ((pa - pb) x vc) • (pa - pc)
    let den_coeff_c = b.vel().cross(c.vel()).dot(a.vel()); // (vb x vc) • va
    let den_const_c = b.vel().cross(c.vel()).dot(pa_m_pc); // (vb x vc) • (pa - pc)

    let pa_m_pd = a.pos() - d.pos();
    let abd_numer_coeff_a0 = a.vel().cross(d.vel()).dot(pa_m_pd); // (va x vd) • (pa - pd)
    let abd_numer_coeff_a1 = (a.pos() - b.pos()).cross(d.vel()).dot(a.vel()); // ((pa - pb) x vd) • va
    let num_coeff_d = abd_numer_coeff_a0 + abd_numer_coeff_a1;
    let num_const_d = (a.pos() - b.pos()).cross(d.vel()).dot(pa_m_pd); // ((pa - pb) x vd) • (pa - pd)
    let den_coeff_d = b.vel().cross(d.vel()).dot(a.vel()); // (vb x vd) • va
    let den_const_d = b.vel().cross(d.vel()).dot(pa_m_pd); // (vb x vd) • (pa - pd)

    let quadratic_a = BigInt::from(num_coeff_c * den_coeff_d - num_coeff_d * den_coeff_c);
    let quadratic_b_a0 = den_coeff_d * num_const_c + num_coeff_c * den_const_d;
    let quadratic_b_a1 = den_coeff_c * num_const_d + den_const_c * num_coeff_d;
    let quadratic_b = BigInt::from(quadratic_b_a0 - quadratic_b_a1);
    let quadratic_c = BigInt::from(num_const_c) * den_const_d - BigInt::from(num_const_d) * den_const_c;

    let discriminant = &quadratic_b * &quadratic_b - BigInt::from(4) * &quadratic_a * &quadratic_c;
    let discriminant_sqrt = discriminant.sqrt();

    let t0 = (-&quadratic_b + &discriminant_sqrt) / (BigInt::from(2) * &quadratic_a);
    let t1 = (-&quadratic_b - &discriminant_sqrt) / (BigInt::from(2) * &quadratic_a);

    let ta = t0.max(t1).iter_u64_digits().next().unwrap() as i128;

    let tb_numer = ta * num_coeff_c + num_const_c;
    let tb_denom = ta * den_coeff_c + den_const_c;
    let tb = tb_numer / tb_denom;

    let vel = (b.at_time(tb) - a.at_time(ta)) / (tb - ta);
    let pos = a.at_time(ta) - vel * ta;

    println!("Part 2: {}", pos.dot(Vec3::new(1, 1, 1)));
}


fn parse_input(input: &'static str) -> Vec<Projectile> {
    input.split('\n')
        .map(std::str::FromStr::from_str)
        .map(Result::unwrap)
        .collect()
}
