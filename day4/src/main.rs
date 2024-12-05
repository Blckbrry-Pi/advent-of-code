macro_rules! matching_pattern {
    (@impl _) => { None };
    (@impl X) => { Some(Letter::X) };
    (@impl M) => { Some(Letter::M) };
    (@impl A) => { Some(Letter::A) };
    (@impl S) => { Some(Letter::S) };
    (
        $(
            [
                $(
                    $cell:tt
                ),+
            ]
        )+
    ) => {
        [$(
            [$(
                matching_pattern!(@impl $cell),
            )+],
        )+]
    };
}

fn main() {
    part1();
    part2();
}

const TEST: &str = include_str!("../../data/day4/test.txt");
const PART_1: &str = include_str!("../../data/day4/p1.txt");
const PART_2: &str = include_str!("../../data/day4/p2.txt");

const XMAS: Pattern = matching_pattern![
    [X, _, _, _, _]
    [_, M, _, _, _]
    [_, _, A, _, _]
    [_, _, _, S, _]
    [_, _, _, _, _]
];

const X_MAS: Pattern = matching_pattern![
    [_, _, _, _, _]
    [_, M, _, S, _]
    [_, _, A, _, _]
    [_, M, _, S, _]
    [_, _, _, _, _]
];

fn part1() {
    const ROTATIONS: &[usize] = &[0, 1, 2, 3, 4, 5, 6, 7];

    let field = parse_input(PART_1);
    let matches_found = find_5x5_with_rotations(&field, XMAS, &ROTATIONS).len();

    println!("Part 1: {}", matches_found);
}

fn part2() {
    const ROTATIONS: &[usize] = &[0, 2, 4, 6];

    let field = parse_input(PART_2);
    let matches_found = find_5x5_with_rotations(&field, X_MAS, &ROTATIONS).len();

    println!("Part 2: {}", matches_found);
}

fn parse_input(input: &str) -> Vec<Vec<Letter>> {
    input.lines()
        .map(|line| line.chars().map(Letter::new).collect())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Letter {
    X,
    M,
    A,
    S,
}

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

fn is(field: &[Vec<Letter>], r: isize, c: isize, letter: Letter) -> bool {
    if r < 0 || c < 0 {
        return false;
    }
    field.get(r as usize)
        .and_then(|row| row.get(c as usize))
        .map_or(false, |&l| l == letter)
}

type Pattern = [[Option<Letter>; 5]; 5];

fn find_5x5_with_rotations(field: &[Vec<Letter>], pattern: Pattern, rotations: &[usize]) -> Vec<(usize, (isize, isize))> {
    let mut found = vec![];

    let width = field[0].len() as isize;
    let height = field.len() as isize;

    for r in -5..height + 5 {
        for c in -5..width + 5 {
            for &rotation in rotations {
                if check_5x5(field, r, c, rotate_5x5_45deg_n(pattern, rotation)) {
                    found.push((rotation, (r, c)));
                }
            }
        }
    }
    found
}

fn find_5x5(field: &[Vec<Letter>], pattern: Pattern) -> Vec<(isize, isize)> {
    let mut found = vec![];
    let width = field[0].len() as isize;
    let height = field.len() as isize;
    for r in -5..height + 5 {
        for c in -5..width + 5 {
            if check_5x5(field, r, c, pattern) {
                found.push((r, c));
            }
        }
    }
    found
}

fn check_5x5(field: &[Vec<Letter>], r: isize, c: isize, pattern: Pattern) -> bool {
    for (or, row) in pattern.iter().enumerate() {
        for (oc, letter) in row.iter().enumerate() {
            let (r, c) = (or as isize + r, oc as isize + c);
            let Some(letter) = letter else { continue };
            if !is(field, r, c, *letter) {
                return false;
            }
        }
    }
    true
}

fn rotate_5x5_45deg_n<T>(val: [[T; 5]; 5], n: usize) -> [[T; 5]; 5] {
    let mut val = val;
    for _ in 0..n {
        val = rotate_5x5_45deg(val);
    }
    val
}

fn rotate_5x5_45deg<T>(val: [[T; 5]; 5]) -> [[T; 5]; 5] {
    let [
        [a, b, c, d, e],
        [p, q, r, s, f],
        [o, x, y, t, g],
        [n, w, v, u, h],
        [m, l, k, j, i],
    ] = val;

    [
        [o, p, a, b, c],
        [n, x, q, r, d],
        [m, w, y, s, e],
        [l, v, u, t, f],
        [k, j, i, h, g],
    ]
}

