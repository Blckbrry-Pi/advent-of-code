aoc_tools::aoc_sol!(day06 2018: part1, part2);
aoc_tools::pos!(Scalar; +y => D);

type Scalar = i32;

fn closest(pos: Pos, points: &[Pos]) -> (Option<(usize, Pos)>, Scalar) {
    let mut closest = None;
    let mut min_dist = Scalar::MAX;
    let mut margin = 0;

    for i in 0..points.len() {
        let point = points[i];
        let dist = point.manhattan(pos);
        if dist == min_dist {
            closest = None;
            margin = 0;
        } else if dist < min_dist {
            closest = Some((i, point));
            margin = min_dist - dist;
            min_dist = dist;
        } else if min_dist + margin > dist {
            margin = dist - min_dist;
        }
    }
    (closest, margin)
}

fn total_dist(pos: Pos, points: &[Pos]) -> u32 {
    points.iter().map(|p| p.manhattan(pos) as u32).sum()
}

pub fn part1(input: &str) -> u32 {
    let points = parse_input(input);

    let (x_range, y_range) = points.iter()
        .fold(
            ((points[0].x, points[0].x), (points[0].y, points[0].y)),
            |(x_range, y_range), p| (
                (x_range.0.min(p.x), x_range.1.max(p.x)),
                (y_range.0.min(p.y), y_range.1.max(p.y)),
            ),
        );
    let x_iter = x_range.0 - (x_range.1 - x_range.0)..=x_range.1 + (x_range.1 - x_range.0);
    let y_iter = y_range.0 - (y_range.1 - y_range.0)..=y_range.1 + (y_range.1 - y_range.0);
    let mut counts = vec![0; points.len()];
    let mut infinite = HashSet::new();

    for y in y_iter.clone() {
        let mut x = *x_iter.start();
        while x_iter.contains(&x) {
            let pos = Pos { x, y };
            let (Some((closest, closest_pos)), margin) = closest(pos, &points) else { x += 1; continue; };
            if x == *x_iter.start() || x == *x_iter.end() || y == *y_iter.start() || y == *y_iter.end() {
                infinite.insert(closest);
            }
            let step = (closest_pos.x - x).max(0) + (margin / 2).max(1);
            counts[closest] += step as u32;
            x += step;
        }
    }

    for i in infinite {
        counts[i] = 0;
    }

    counts.into_iter().max().unwrap()
}

pub fn part2(input: &str) -> u32 {
    const MAX_DIST: u32 = 10_000;

    let points = parse_input(input);
    let (x_range, y_range) = points.iter()
        .fold(
            ((points[0].x, points[0].x), (points[0].y, points[0].y)),
            |(x_range, y_range), p| (
                (x_range.0.min(p.x), x_range.1.max(p.x)),
                (y_range.0.min(p.y), y_range.1.max(p.y)),
            ),
        );
    let offset = MAX_DIST as Scalar / points.len() as Scalar;
    let x_iter = x_range.0 - offset..=x_range.1 + offset;
    let y_iter = y_range.0 - offset..=y_range.1 + offset;

    let mut count = 0;
    for y in y_iter.clone() {
        let mut x = *x_iter.start();
        while x_iter.contains(&x) {
            let pos = Pos { x, y };
            let dist = total_dist(pos, &points);
            let within = dist < MAX_DIST;
            let step = MAX_DIST.abs_diff(dist) / points.len() as u32;
            let step = step.max(1);
            if within {
                count += step;
            }
            x += step as Scalar;
        }
    }
    count
}

fn parse_input(input: &str) -> Vec<Pos> {
    input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (x, y) = l.split_once(", ").unwrap();
            let x: Scalar = x.parse().unwrap();
            let y: Scalar = y.parse().unwrap();
            Pos { x, y }
        })
        .collect()
}
