aoc_tools::aoc_sol!(day23 2018: part1, part2);
aoc_tools::pos3!(Scalar);
type Scalar = i32;

pub fn part1(input: &str) -> Scalar {
    let mut bots = parse_input(input);
    bots.sort_by_key(|(_, radius)| *radius);
    let (pos, radius) = bots.pop().unwrap();
    let mut in_range_count = 1;
    for bot in bots {
        if bot.0.manhattan(pos) <= radius {
            in_range_count += 1;
        }
    }
    in_range_count
}

pub fn part2(input: &str) -> Scalar {
    let bots = parse_input(input);
    let (mut x_range, mut y_range, mut z_range) = bots.iter().fold(
        ((Scalar::MAX, Scalar::MIN), (Scalar::MAX, Scalar::MIN), (Scalar::MAX, Scalar::MIN)),
        |((x_min, x_max), (y_min, y_max), (z_min, z_max)), &(Pos3 { x, y, z }, _)| (
            (x_min.min(x), x_max.max(x)),
            (y_min.min(y), y_max.max(y)),
            (z_min.min(z), z_max.max(z)),
        ),
    );
    while x_range.1 != x_range.0 || y_range.1 != y_range.0 || z_range.1 != z_range.0 {
        let x_step = (x_range.1 - x_range.0) / 2;
        let y_step = (y_range.1 - y_range.0) / 2;
        let z_step = (z_range.1 - z_range.0) / 2;

        let mut best = ((0, 0), (0, 0), (0, 0));
        let mut best_count = 0;
        for xdiv in [0, 1] {
            let x = xdiv * x_step + x_step / 2 + x_range.0;
            for ydiv in [0, 1] {
                let y = ydiv * y_step + y_step / 2 + y_range.0;
                for zdiv in [0, 1] {
                    let z = zdiv * z_step + z_step / 2 + z_range.0;

                    let pos = Pos3 { x, y, z };
                    let radius_addition = (x_step + y_step + z_step + 3) / 2;

                    let mut in_range = 0;
                    for &bot in &bots {
                        if bot.0.manhattan(pos) <= bot.1 + radius_addition {
                            in_range += 1;
                        }
                    }
                    if in_range > best_count {
                        println!("{xdiv},{ydiv},{zdiv}, {in_range}");
                        if xdiv == 0 {
                            best.0 = (x_range.0, x_range.0 + x_step);
                        } else {
                            best.0 = (x_range.0 + x_step + 1, x_range.1);
                        }
                        if ydiv == 0 {
                            best.1 = (y_range.0, y_range.0 + y_step);
                        } else {
                            best.1 = (y_range.0 + y_step + 1, y_range.1);
                        }
                        if zdiv == 0 {
                            best.2 = (z_range.0, z_range.0 + z_step);
                        } else {
                            best.2 = (z_range.0 + z_step + 1, z_range.1);
                        }
                        best_count = in_range;
                    }
                }
            }
        }
        x_range = best.0;
        y_range = best.1;
        z_range = best.2;
    }

    let guess = Pos3 { x: x_range.0, y: y_range.0, z: z_range.0 };
    let center = Pos3 { x: 0, y: 0, z: 0 };
    let mut best = Pos3 { x: 0, y: 0, z: 0 };
    let mut best_count = 0;
    for x in x_range.0 - 5..=x_range.1 + 5 {
        for y in y_range.0 - 5..=y_range.1 + 5 {
            for z in z_range.0 - 5..=z_range.1 + 5 {
                let pos = Pos3 { x, y, z };
                if pos.manhattan(guess) > 5 { continue }
                let mut in_range = 0;
                for &bot in &bots {
                    if bot.0.manhattan(pos) <= bot.1 {
                        in_range += 1;
                    }
                }
                if in_range > best_count || (in_range == best_count && pos.manhattan(center) < best.manhattan(center)) {
                    best = pos;
                    best_count = in_range;
                }
            }
        }
    }

    best.manhattan(center)
}

fn parse_input(input: &str) -> Vec<(Pos3, Scalar)> {
    input.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (pos, radius) = l.split_once(">, r=").unwrap();
            let pos = pos.strip_prefix("pos=<").unwrap();
            let (x, yz) = pos.split_once(',').unwrap();
            let (y, z) = yz.split_once(',').unwrap();
            let x = x.parse::<Scalar>().unwrap();
            let y = y.parse::<Scalar>().unwrap();
            let z = z.parse::<Scalar>().unwrap();
            let radius = radius.parse::<Scalar>().unwrap();
            (Pos3 { x, y, z }, radius)
        })
        .collect()
}
