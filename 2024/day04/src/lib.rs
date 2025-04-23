aoc_tools::aoc_sol!(day04 2024: part1, part2);
aoc_tools::pos!(i16; +y=>D);

pub fn part1(input: &str) -> usize {
    let field = parse_input(input);
    match_xmas(&field)
}

pub fn part2(input: &str) -> usize {
    let field = parse_input(input);
    match_x_mas(&field)
}

fn parse_input(input: &str) -> Vec<Vec<Letter>> {
    input.lines()
        .map(|line| line.chars().map(Letter::new).collect())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Letter { X, M, A, S }

impl Letter {
    fn new(c: char) -> Self {
        match c {
            'x' | 'X' => Self::X,
            'm' | 'M' => Self::M,
            'a' | 'A' => Self::A,
            's' | 'S' => Self::S,
            _ => panic!("Invalid letter: {}", c),
        }
    }
}

fn match_xmas(search: &[Vec<Letter>]) -> usize {
    let mut matches = 0;
    for y in 0..search.len() {
        for x in 0..search[y].len() {
            let pos = Pos { x: x as i16, y: y as i16 };
            if search[y][x] != Letter::X { continue }
            let dirs = [
                Pos::U.add(Pos::L),
                Pos::U,
                Pos::U.add(Pos::R),

                Pos::L,
                Pos::R,

                Pos::D.add(Pos::L),
                Pos::D,
                Pos::D.add(Pos::R),
            ];
            for dir in dirs {
                let m_pos = pos.add(dir);
                let a_pos = m_pos.add(dir);
                let s_pos = a_pos.add(dir);

                let s_x_in_range = 0 <= s_pos.x && s_pos.x < search[y].len() as i16;
                let s_y_in_range = 0 <= s_pos.y && s_pos.y < search.len() as i16;
                if !s_x_in_range || !s_y_in_range { continue }

                let m = search[m_pos.y as usize][m_pos.x as usize];
                let a = search[a_pos.y as usize][a_pos.x as usize];
                let s = search[s_pos.y as usize][s_pos.x as usize];

                if m == Letter::M && a == Letter::A && s == Letter::S {
                    matches += 1;
                }
            }
        }
    }
    matches
}

fn match_x_mas(search: &[Vec<Letter>]) -> usize {
    let mut matches = 0;
    for y in 1..search.len()-1 {
        for x in 1..search[y].len()-1 {
            let pos = Pos { x: x as i16, y: y as i16 };
            if search[y][x] != Letter::A { continue }
            let mut corner_ms = 0;
            let mut corner_ss = 0;
            let mut opposite_ms = 0;

            let parts = [
                (Pos::U.add(Pos::L), true),
                (Pos::U.add(Pos::R), false),
                (Pos::D.add(Pos::L), false),
                (Pos::D.add(Pos::R), true),
            ];
            for (corner, is_bottom) in parts {
                let corner = pos.add(corner);
                match search[corner.y as usize][corner.x as usize] {
                    Letter::M => {
                        corner_ms += 1;
                        if is_bottom { opposite_ms += 1 }
                    },
                    Letter::S => corner_ss += 1,
                    _ => break,
                }
            }
            if corner_ms == 2 && corner_ss == 2 && opposite_ms == 1 {
                matches += 1;
            }
        }
    }
    matches
}
