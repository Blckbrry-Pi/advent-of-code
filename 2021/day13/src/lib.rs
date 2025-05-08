use std::str::FromStr;

aoc_tools::aoc_sol!(day13 2021: part1, part2);
aoc_tools::pos!(i16; +y => D);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fold {
    AlongX(i16),
    AlongY(i16),
}
impl Fold {
    pub fn fold(&self, pos: Pos) -> Pos {
        match *self {
            Self::AlongX(x) => Pos {
                x: x.abs_diff(x.abs_diff(pos.x) as i16) as i16,
                y: pos.y
            },
            Self::AlongY(y) => Pos {
                x: pos.x,
                y: y.abs_diff(y.abs_diff(pos.y) as i16) as i16,
            },
        }
    }
}
impl FromStr for Fold {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((axis, num)) = s.trim_start_matches("fold along ").split_once('=') else {
            return Err("Missing `=`".to_string());
        };
        let num = num.parse::<i16>().map_err(|e| e.to_string())?;
        if axis == "x" {
            Ok(Self::AlongX(num))
        } else {
            Ok(Self::AlongY(num))
        }
    }
}

#[derive(Clone)]
struct Transparency {
    dots: HashSet<Pos>,
}
impl Transparency {
    pub fn fold(&mut self, fold: Fold) {
        self.dots = self.dots.drain()
            .map(|dot| fold.fold(dot))
            .collect();
    }
}
impl Debug for Transparency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (max_x, max_y) = self.dots.iter().fold((0, 0), |(x, y), new| (x.max(new.x), y.max(new.y)));
        for y in 0..=max_y {
            for x in 0..=max_x {
                let pos = Pos { x, y };
                if self.dots.contains(&pos) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl FromStr for Transparency {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dots = s.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| {
                let (x, y) = l.split_once(',').unwrap();
                let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
                Pos { x, y }
            })
            .collect();
        Ok(Self { dots })
    }
}

pub fn part1(input: &str) -> usize {
    let (mut transparency, folds) = parse_input(input);
    transparency.fold(folds[0]);
    transparency.dots.len()
}

pub fn part2(input: &str) -> String {
    let (mut transparency, folds) = parse_input(input);
    for fold in folds {
        transparency.fold(fold);
    }
    // TODO: Decode it into a `String`
    println!("{transparency:?}");
    "".to_string()
}

fn parse_input(input: &str) -> (Transparency, Vec<Fold>) {
    let (transparency, folds) = input.split_once("\n\n").unwrap();
    let transparency = Transparency::from_str(transparency).unwrap();
    let folds = folds.trim().lines().map(Fold::from_str).map(Result::unwrap).collect();
    (transparency, folds)
}
