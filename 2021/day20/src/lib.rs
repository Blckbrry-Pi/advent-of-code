use std::str::FromStr;

type Scalar = i16;

aoc_tools::aoc_sol!(day20 2021: part1, part2);
aoc_tools::pos!(i16; +y => D);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Dark = 0,
    Light = 1,
}
impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Light => write!(f, "#"),
            Self::Dark => write!(f, "."),
        }
    }
}
impl FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 { return Err("String must be 1 char".to_string()); }
        match s.as_bytes()[0] as char {
            '#' => Ok(Self::Light),
            '.' => Ok(Self::Dark),
            c => Err(format!("Invalid color {c:?}")),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct ImageEnhancement([Color; 512]);
impl ImageEnhancement {
    pub fn get(&self, colors: [Color; 9]) -> Color {
        let mut idx = 0;
        for color in colors {
            idx <<= 1;
            idx |= color as usize;
        }
        self.0[idx]
    }
}
impl Debug for ImageEnhancement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..512 {
            write!(f, "{:?}", self.0[i])?;
        }
        Ok(())
    }
}
impl FromStr for ImageEnhancement {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut output = Self([Color::Dark; 512]);
        if s.len() != 512 {
            return Err(format!("String must be 512 chars, is {} chars", s.len()));
        }
        for i in 0..512 {
            let substr = &s[i..i+1];
            output.0[i] = substr.parse()?;
        }
        Ok(output)
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Image {
    rows: Vec<Vec<Color>>,
    background_color: Color,
}
impl Image {
    pub fn non_inf_width(&self) -> usize {
        self.rows[0].len()
    }
    pub fn non_inf_height(&self) -> usize {
        self.rows.len()
    }
    pub fn count(&self) -> usize {
        self.rows
            .iter()
            .map(
                |r| r.iter()
                    .filter(|&&c| c == Color::Light)
                    .count(),
            )
            .sum()
    }
    pub fn get(&self, pos: Pos) -> Color {
        if 0 > pos.x || pos.x as usize >= self.non_inf_width() {
            self.background_color
        } else if 0 > pos.y || pos.y as usize >= self.non_inf_height() {
            self.background_color
        } else {
            self.rows[pos.y as usize][pos.x as usize]
        }
    }
    pub fn get_surrounding(&self, pos: Pos) -> [Color; 9] {
        let above = pos.add(Pos::N);
        let below = pos.add(Pos::S);
        let positions = [
            above.add(Pos::W),
            above,
            above.add(Pos::E),
            pos.add(Pos::W),
            pos,
            pos.add(Pos::E),
            below.add(Pos::W),
            below,
            below.add(Pos::E),
        ];
        positions.map(|p| self.get(p))
    }
    pub fn get_infinite(&self) -> [Color; 9] {
        [self.background_color; 9]
    }
    pub fn enhanced_with(&self, enhancement: &ImageEnhancement) -> Self {
        let mut new_output = Self {
            rows: vec![],
            background_color: Color::Light,
        };
        for new_y in -1..=self.non_inf_height() as Scalar {
            new_output.rows.push(vec![]);
            for new_x in -1..=self.non_inf_width() as Scalar {
                let new_pixel = enhancement.get(self.get_surrounding(Pos {
                    x: new_x,
                    y: new_y
                }));
                new_output.rows.last_mut().unwrap().push(new_pixel);
            }
        }
        new_output.background_color = enhancement.get(self.get_infinite());
        new_output
    }
}
impl Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.non_inf_width()+2 {
            write!(f, "{:?}", self.background_color)?;
        }
        writeln!(f)?;
        for y in 0..self.non_inf_height() {
            write!(f, "{:?}", self.background_color)?;
            for x in 0..self.non_inf_width() {
                write!(f, "{:?}", self.rows[y][x])?;
            }
            write!(f, "{:?}", self.background_color)?;
            writeln!(f)?;
        }
        for _ in 0..self.non_inf_width()+2 {
            write!(f, "{:?}", self.background_color)?;
        }
        Ok(())
    }
}
impl FromStr for Image {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|line| {
                (0..line.len())
                    .map(|i| &line[i..i+1])
                    .map(|c| c.parse())
                    .collect::<Result<Vec<Color>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            rows,
            background_color: Color::Dark,
        })
    }
}

pub fn part1(input: &str) -> usize {
    let (enhancement, mut image) = parse_input(input);
    for _ in 0..2 {
        image = image.enhanced_with(&enhancement);
    }
    image.count()
}

pub fn part2(input: &str) -> usize {
    let (enhancement, mut image) = parse_input(input);
    for _ in 0..50 {
        image = image.enhanced_with(&enhancement);
    }
    println!("{image:?}");
    image.count()
}

fn parse_input(input: &str) -> (ImageEnhancement, Image) {
    let (enhancement, image) = input.split_once("\n\n").unwrap();
    (
        enhancement.parse().unwrap(),
        image.parse().unwrap(),
    )
}
