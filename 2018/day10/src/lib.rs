aoc_tools::aoc_sol!(day10 2018: part1, part2);
aoc_tools::pos!(Scalar; +y => D);

type Scalar = i32;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Star {
    pos: Pos,
    vel: Pos,
}
impl Star {
    pub fn step(&mut self) {
        self.pos = self.pos.add(self.vel);
    }
    pub fn step_n(&mut self, n: usize) {
        self.pos = self.pos.add(self.vel.mul(n as Scalar));
    }
    pub fn average(stars: &[Star]) -> Pos {
        let x = stars.iter().map(|s| s.pos.x as i64).sum::<i64>();
        let y = stars.iter().map(|s| s.pos.y as i64).sum::<i64>();
        Pos { x: (x / stars.len() as i64) as Scalar, y: (y / stars.len() as i64) as Scalar }
    }
    pub fn range(stars: &[Star]) -> (Pos, Pos) {
        stars.iter()
            .map(|s| s.pos)
            .fold(
                (Pos { x: Scalar::MAX, y: Scalar::MAX }, Pos { x: Scalar::MIN, y: Scalar::MIN }),
                |(min, max), pos| (
                    Pos { x: min.x.min(pos.x), y: min.y.min(pos.y) },
                    Pos { x: max.x.max(pos.x), y: max.y.max(pos.y) },
                )
            )
    }
}
impl Debug for Star {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Star[ pos=<{:>6}, {:>6}> vel=<{:>6}, {:>6}> ]",
            self.pos.x,
            self.pos.y,
            self.vel.x,
            self.vel.y,
        )
    }
}

fn get_message_time(initial: &[Star]) -> usize {
    let mut stars = initial.to_vec();
    let mut max_vertical_line_score = (0, 0);
    for t in 0_usize.. {
        let avg = Star::average(&stars);
        let mut vertical_line_score = 0;
        for x in avg.x-10..=avg.x+10 {
            let mut vertical_lineness = 0_u32;
            for y in avg.y-4..=avg.y+4 {
                let pos = Pos { x, y };
                if stars.iter().any(|s| s.pos == pos) {
                    vertical_lineness += 1;
                }
            }
            vertical_line_score += vertical_lineness.pow(3);
        }
        if vertical_line_score >= max_vertical_line_score.0 {
            max_vertical_line_score = (vertical_line_score, t);
        }
        if t - max_vertical_line_score.1 > 100 { break }

        for star in &mut stars {
            star.step();
        }
    }
    max_vertical_line_score.1
}

pub fn part1(input: &str) -> String {
    let mut stars = parse_input(input);
    let message_time = get_message_time(&stars);
    for star in &mut stars {
        star.step_n(message_time);
    }

    let stars_range = Star::range(&stars);
    let mut output = "\n".to_string();
    for y in stars_range.0.y-1..=stars_range.1.y+1 {
        for x in stars_range.0.x-2..=stars_range.1.x+2 {
            let pos = Pos { x, y };
            if stars.iter().any(|s| s.pos == pos) {
                output.push('#');
            } else {
                output.push('.');
            }
        }
        output.push('\n');
    }
    output.pop();
    output
}

pub fn part2(input: &str) -> usize {
    let stars = parse_input(input);
    get_message_time(&stars)
}

fn parse_input(input: &str) -> Vec<Star> {
    input.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let rest = l.strip_prefix("position=<").unwrap();
            let (pos, vel) = rest.split_once("> velocity=<").unwrap();
            let vel = vel.strip_suffix(">").unwrap();
            let (pos_x, pos_y) = pos.split_once(",").unwrap();
            let (vel_x, vel_y) = vel.split_once(",").unwrap();
            let pos_x = pos_x.trim().parse::<Scalar>().unwrap();
            let pos_y = pos_y.trim().parse::<Scalar>().unwrap();
            let vel_x = vel_x.trim().parse::<Scalar>().unwrap();
            let vel_y = vel_y.trim().parse::<Scalar>().unwrap();
            let pos = Pos { x: pos_x, y: pos_y };
            let vel = Pos { x: vel_x, y: vel_y };

            Star { pos, vel }
        })
        .collect()
}
