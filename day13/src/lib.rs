aoc_tools::aoc_sol!(day13: part1, part2);

aoc_tools::pos!(isize);

pub fn part1(input: &str) -> usize {
    let machines = parse_input(input);

    let mut tokens = 0;
    for machine in machines {
        let (a, b) = machine.solve();
        if machine.solve_valid(a, b) {
            tokens += 3 * a + b;
        }
    }

    tokens as usize
}

pub fn part2(input: &str) -> usize {
    let machines = parse_input(input);

    let mut tokens = 0;
    for machine in machines {
        let machine = machine.adjust_p2();
        let (a, b) = machine.solve();
        if machine.solve_valid(a, b) {
            tokens += 3 * a + b;
        }
    }

    tokens as usize
}

fn parse_input(input: &str) -> Vec<Machine> {
    input.split("\n\n")
        .map(|machine| {
            let (a, rest) = machine.split_once('\n').unwrap();
            let (b, prize) = rest.split_once('\n').unwrap();
            let a = a.trim_start_matches("Button A: X+");
            let b = b.trim_start_matches("Button B: X+");
            let prize = prize.trim_start_matches("Prize: X=");

            let (a_x, a_y) = a.split_once(", Y+").unwrap();
            let (b_x, b_y) = b.split_once(", Y+").unwrap();
            let (prize_x, prize_y) = prize.split_once(", Y=").unwrap();

            let (a_x, a_y) = (a_x.parse().unwrap(), a_y.parse().unwrap());
            let (b_x, b_y) = (b_x.parse().unwrap(), b_y.parse().unwrap());
            let (prize_x, prize_y) = (prize_x.parse().unwrap(), prize_y.parse().unwrap());

            Machine {
                a: Pos { x: a_x, y: a_y },
                b: Pos { x: b_x, y: b_y },
                prize: Pos { x: prize_x, y: prize_y },
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Machine {
    a: Pos,
    b: Pos,
    prize: Pos,
}

impl Machine {
    pub fn adjust_p2(&self) -> Self {
        let offset = 10_000_000_000_000;
        let offset = Pos { x: offset, y: offset };
        Self {
            prize: self.prize.add(offset),
            ..*self
        }
    }

    pub fn solve(&self) -> (isize, isize) {
        // A * a.x + B * b.x = p.x
        // A * a.y + B * b.y = p.y

        // A * a.x = p.x - B * b.x
        // A * a.y = p.y - B * b.y

        // A = (p.x - B * b.x) / a.x
        // A = (p.y - B * b.y) / a.y

        // (p.x - B * b.x) / a.x = (p.y - B * b.y) / a.y
        // (p.x - B * b.x) * a.y = (p.y - B * b.y) * a.x
        // p.x * a.y - B * b.x * a.y = p.y * a.x - B * b.y * a.x
        // p.x * a.y - p.y * a.x = B * b.x * a.y - B * b.y * a.x
        // p.x * a.y - p.y * a.x = B * (b.x * a.y - b.y * a.x)
        // B = (p.x * a.y - p.y * a.x) / (b.x * a.y - b.y * a.x)

        let b_coeff_num = self.prize.x * self.a.y - self.prize.y * self.a.x;
        let b_coeff_den = self.b.x * self.a.y - self.b.y * self.a.x;
        let b_coeff = b_coeff_num / b_coeff_den;

        let a_coeff = (self.prize.x - b_coeff * self.b.x) / self.a.x;

        (a_coeff, b_coeff)
    }

    pub fn solve_valid(&self, a: isize, b: isize) -> bool {
        a >= 0 && b >= 0 && self.a.mul(a).add(self.b.mul(b)) == self.prize
    }
}
