aoc_tools::aoc_sol!(day21: part1, part2);
aoc_tools::pos!(isize);

pub fn part1(input: &str) -> usize {
    let inputs = parse_input(input);

    let mut sum = 0;

    for (number, numerical) in inputs {
        let mut curr_numer = NumericalButton::AA;
        let mut dir0_buttons = Vec::with_capacity(numerical.len() * 5);
        for new_number in numerical {
            dir0_buttons.extend(curr_numer.navigate_to(new_number));
            dir0_buttons.push(DirectionalButton::A);
            curr_numer = new_number;
        }
    
        let mut curr_dir0 = DirectionalButton::A;
        let mut dir1_buttons = Vec::with_capacity(dir0_buttons.len() * 4);
        for new_dir in dir0_buttons {
            dir1_buttons.extend(curr_dir0.navigate_to(new_dir));
            dir1_buttons.push(DirectionalButton::A);
            curr_dir0 = new_dir;
        }
    
        let mut curr_dir1 = DirectionalButton::A;
        let mut dir2_buttons = Vec::with_capacity(dir1_buttons.len() * 4);
        for new_dir in dir1_buttons {
            dir2_buttons.extend(curr_dir1.navigate_to(new_dir));
            dir2_buttons.push(DirectionalButton::A);
            curr_dir1 = new_dir;
        }

        sum += number * dir2_buttons.len()
    }

    sum
}

pub fn part2(input: &str) -> usize {
    let inputs = parse_input(input);

    let mut sum = 0;

    let directionals = [DirectionalButton::A, DirectionalButton::U, DirectionalButton::D, DirectionalButton::L, DirectionalButton::R];
    let mut pairs = vec![];

    for from in directionals {
        for press in directionals {
            pairs.push((
                MovementIdent { from, press, primes: 0 },
                Movement::Pair { parts: from.navigate_to(press).into_iter().chain([DirectionalButton::A]).collect() }
            ));
        }
    }

    let mut cached_movements: HashMap<_, _> = pairs.into_iter().collect();

    let mut new_movements: HashMap<MovementIdent, Movement> = HashMap::new();
    for i in 0..24 {
        for (&id, movement) in cached_movements.iter() {
            if id.primes != i { continue; }

            let primed = movement.build_prime(&cached_movements);
            new_movements.insert(id.primed(), primed);
        }
        cached_movements.extend(new_movements.drain());
    }

    for (number, numerical) in inputs {
        let mut curr_numer = NumericalButton::AA;
        let mut dir0_buttons = vec![];
        for new_number in numerical {
            dir0_buttons.extend(curr_numer.navigate_to(new_number));
            dir0_buttons.push(DirectionalButton::A);
            curr_numer = new_number;
        }

        let dir25_len: usize = std::iter::once(DirectionalButton::A).chain(dir0_buttons.iter().copied()).zip(dir0_buttons.iter().copied())
            .map(|(from, press)| MovementIdent { from, press, primes: 24 })
            .map(|ident| cached_movements.get(&ident).unwrap())
            .map(|movement| movement.len())
            .sum();

        sum += dir25_len * number;
    }
    sum

}

fn parse_input(input: &str) -> Vec<(usize, Vec<NumericalButton>)> {
    input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| (l[..l.len()-1].parse().unwrap(), l.chars().map(NumericalButton::from_char).collect()))
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum NumericalButton {
    N7, N8, N9,
    N4, N5, N6,
    N1, N2, N3,
        N0, AA,
}
impl NumericalButton {
    fn pos(&self) -> Pos {
        use NumericalButton::*;
        match self {
            N7 => Pos { x: 0, y: 3 },
            N8 => Pos { x: 1, y: 3 },
            N9 => Pos { x: 2, y: 3 },

            N4 => Pos { x: 0, y: 2 },
            N5 => Pos { x: 1, y: 2 },
            N6 => Pos { x: 2, y: 2 },

            N1 => Pos { x: 0, y: 1 },
            N2 => Pos { x: 1, y: 1 },
            N3 => Pos { x: 2, y: 1 },

            // _ => Pos { x: 0, y: 1 },
            N0 => Pos { x: 1, y: 0 },
            AA => Pos { x: 2, y: 0 },
        }
    }
    fn navigate_to(&self, to: Self) -> impl Iterator<Item = DirectionalButton> {
        let dy = to.pos().y - self.pos().y;
        let dx = to.pos().x - self.pos().x;

        let vert = if dy > 0 { DirectionalButton::U } else { DirectionalButton::D };
        let hori = if dx > 0 { DirectionalButton::R } else { DirectionalButton::L };

        let vert_first = hori == DirectionalButton::R;
        let will_panic_with_hori_first = to.pos().x == 0 && self.pos().y == 0;
        let will_panic_with_vert_first = self.pos().x == 0 && to.pos().y == 0;

        let vert = std::iter::repeat(vert).take(dy.abs() as usize);
        let hori = std::iter::repeat(hori).take(dx.abs() as usize);

        // ^A^^<<A>>AvvvA
        // <A>A<AAv<AA>>^AvAA^Av<AAA>^A
        // v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA<^A>Av<A>^AA<A>Av<A<A>>^AAAvA<^A>A

        // ^A<<^^A>>AvvvA
        // <A>Av<<AA>^AA>AvAA^Av<AAA>^A
        // v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<A<A>>^AAAvA<^A>A
        // let vert_first = vert == DirectionalButton::U || hori == DirectionalButton::L;
        // if dy > 0 {
        if will_panic_with_hori_first {
            vert.chain(hori)
        } else if will_panic_with_vert_first {
            hori.chain(vert)
        } else if vert_first {
            vert.chain(hori)
        } else {
            hori.chain(vert)
        }
        // if vert_first {
        //     if to.pos().x == 0 && self.pos().y == 0 {
        //         vert.chain(hori)
        //     } else {
        //         hori.chain(vert)
        //     }
        // } else {
        //     hori.chain(vert)
        // }
    }

    fn from_char(ch: char) -> Self {
        match ch {
            'A' => Self::AA,
            '0' => Self::N0,
            '1' => Self::N1,
            '2' => Self::N2,
            '3' => Self::N3,
            '4' => Self::N4,
            '5' => Self::N5,
            '6' => Self::N6,
            '7' => Self::N7,
            '8' => Self::N8,
            '9' => Self::N9,
            _ => panic!("Invalid char"),
        }
    }
}
impl Debug for NumericalButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use NumericalButton::*;
        let ch = match self {
            AA => 'A',
            N0 => '0',
            N1 => '1',
            N2 => '2',
            N3 => '3',
            N4 => '4',
            N5 => '5',
            N6 => '6',
            N7 => '7',
            N8 => '8',
            N9 => '9',
        };
        write!(f, "{ch}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum DirectionalButton {
       U, A,
    L, D, R,
}
impl DirectionalButton {
    fn pos(&self) -> Pos {
        use DirectionalButton::*;
        match self {
            // _ => Pos { x: 0, y: 1 },
            U => Pos { x: 1, y: 1 },
            A => Pos { x: 2, y: 1 },

            L => Pos { x: 0, y: 0 },
            D => Pos { x: 1, y: 0 },
            R => Pos { x: 2, y: 0 },
        }
    }
    fn navigate_to(&self, to: Self) -> impl Iterator<Item = DirectionalButton> {
        let dy = to.pos().y - self.pos().y;
        let dx = to.pos().x - self.pos().x;

        let vert = if dy > 0 { Self::U } else { Self::D };
        let hori = if dx > 0 { Self::R } else { Self::L };

        let vert_first = hori == DirectionalButton::R;
        let will_panic_with_hori_first = to.pos().x == 0 && self.pos().y == 1;
        let will_panic_with_vert_first = self.pos().x == 0 && to.pos().y == 1;

        let vert = std::iter::repeat(vert).take(dy.abs() as usize);
        let hori = std::iter::repeat(hori).take(dx.abs() as usize);

        if will_panic_with_hori_first {
            vert.chain(hori)
        } else if will_panic_with_vert_first {
            hori.chain(vert)
        } else if vert_first {
            vert.chain(hori)
        } else {
            hori.chain(vert)
        }
        // if dy > 0 {
        //     hori.chain(vert)
        // } else {
        //     vert.chain(hori)
        // }.collect()
    }
}
impl Debug for DirectionalButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DirectionalButton::*;
        let ch = match self {
            U => '^',
            D => 'v',
            L => '<',
            R => '>',
            A => 'A',
        };
        write!(f, "{ch}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MovementIdent { from: DirectionalButton, press: DirectionalButton, primes: usize }
impl MovementIdent {
    pub fn primed(&self) -> Self {
        Self { from: self.from, press: self.press, primes: self.primes + 1 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Movement {
    Pair { parts: Vec<DirectionalButton> },
    Primed { parts: Vec<MovementIdent>, len: usize },
}

impl Movement {
    pub fn len(&self) -> usize {
        match self {
            Self::Pair { parts, .. } => parts.len(),
            Self::Primed { len, .. } => *len,
        }
    }

    pub fn build_prime(&self, cached: &HashMap<MovementIdent, Self>) -> Self {
        match self {
            Self::Pair { parts } => {
                let parts: Vec<_> = std::iter::once(DirectionalButton::A)
                    .chain(parts.iter().copied())
                    .zip(parts.iter().copied())
                    .map(|(from, press)| MovementIdent { from, press, primes: 0 })
                    .collect();

                let len = parts.iter().map(|ident| cached.get(ident).unwrap().len()).sum();

                Self::Primed { parts, len }
            },
            Self::Primed { parts, .. } => {
                let primed_parts: Vec<_> = parts.iter()
                    .map(|ident| ident.primed())
                    .collect();
                let len = primed_parts.iter().map(|ident| cached.get(ident).unwrap().len()).sum();

                Self::Primed { parts: primed_parts, len }
            }
        }
    }
}

// struct Movement { press: DirectionalButton, from: DirectionalButton, len: usize }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// struct DerivedMovement { movement: Movement, derivations: usize, len: usize }
