aoc_tools::aoc_sol!(day10: part1, part2);
type Scalar = i16;
aoc_tools::pos!(Scalar; +y=>D);

pub fn part1(input: &str) -> usize {
    let topography = parse_input(input);

    let trailheads: Vec<_> = topography.trailheads()
        .map(|head| get_9s(head, &topography))
        .collect();

    trailheads.iter().copied().sum()
}

pub fn part2(input: &str) -> usize {
    let topography = parse_input(input);

    count_9s_paths(topography.trailheads(), &topography)
}

fn get_9s(start: Pos, topography: &Topography) -> usize {
    let mut curr_positions: HashSet<_> = [start].into_iter().collect();
    let mut new_positions = HashSet::new();
    while topography.get(*curr_positions.iter().next().unwrap()) != 9 {
        for position in curr_positions {
            for candidate in topography.adjacent_iter(position) {
                new_positions.insert(candidate);
            }
        }
        curr_positions = new_positions;
        new_positions = HashSet::with_capacity(curr_positions.len() * 3);
    }
    curr_positions.len()
}

fn count_9s_paths(starts: impl Iterator<Item = Pos>, topography: &Topography) -> usize {
    let mut curr_paths: HashMap<_, _> = starts.map(|p| (p, 1)).collect();
    let mut new_paths = HashMap::new();
    while topography.get(*curr_paths.iter().next().unwrap().0) != 9 {
        for (pos, count) in curr_paths {
            for candidate in topography.adjacent_iter(pos) {
                *new_paths.entry(candidate).or_default() += count;
            }
        }
        curr_paths = new_paths;
        new_paths = HashMap::with_capacity(curr_paths.len() * 2);
    }
    curr_paths.into_values().sum()
}

fn parse_input(input: &str) -> Topography {
    let levels = input.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|v| v.chars().map(|v| v as u8 - b'0'))
        .map(|r| r.collect::<Vec<_>>())
        .collect();

    Topography { levels }
}

#[derive(Clone)]
struct Topography {
    levels: Vec<Vec<u8>>,
}
impl Topography {
    pub fn width(&self) -> usize { self.levels[0].len() }
    pub fn height(&self) -> usize { self.levels.len() }
    pub fn get(&self, pos: Pos) -> u8 {
        if 0 > pos.x || pos.x >= self.width() as Scalar {
            255
        } else if 0 > pos.y || pos.y >= self.height() as Scalar {
            255
        } else {
            self.levels[pos.y as usize][pos.x as usize]
        }
    }
    pub fn adjacent_iter(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        let at = self.get(pos);
        // let candidates = if at == 255 {
        //     vec![]
        // } else {
        //     vec![pos.add(Pos::N), pos.add(Pos::S), pos.add(Pos::W), pos.add(Pos::E)]
        // };
        [pos.add(Pos::N), pos.add(Pos::S), pos.add(Pos::W), pos.add(Pos::E)]
            .into_iter()
            // .filter(move |_| at != 255)
            .filter(move |v| self.get(*v) == at + 1)
        // candidates.into_iter().filter(move |v| self.get(*v) == at + 1)
    }

    pub fn trailheads(&self) -> impl Iterator<Item = Pos> + '_ {
        self.levels.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, &cell)| {
                        (cell == 0).then_some(Pos { x: x as Scalar, y: y as Scalar })
                    })
            })
    }
}
impl Debug for Topography {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.levels {
            for &cell in row {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
