aoc_tools::aoc_sol!(day10: part1, part2);

fn part1(input: &str) -> usize {
    let topography = parse_input(input);

    let trailheads: Vec<_> = topography.trailheads()
        .map(|head| get_9s(head, &topography))
        .map(|nines| nines.len())
        .collect();

    trailheads.iter().copied().sum()
}

fn part2(input: &str) -> usize {
    let topography = parse_input(input);

    let trailheads: Vec<_> = topography.trailheads()
        .map(|head| get_9s_paths(head, &topography))
        .map(|nines| nines.len())
        .collect();

    trailheads.iter().copied().sum()
}

fn get_9s(start: Pos, topography: &Topography) -> HashSet<Pos> {
    let mut curr_positions: HashSet<_> = [start].into_iter().collect();
    let mut new_positions = HashSet::new();
    while topography.get(*curr_positions.iter().next().unwrap()) != 9 {
        for position in curr_positions {
            for candidate in topography.adjacent_iter(position) {
                new_positions.insert(candidate);
            }
        }
        curr_positions = new_positions;
        new_positions = HashSet::new();
    }
    curr_positions
}

fn get_9s_paths(start: Pos, topography: &Topography) -> HashSet<Vec<Pos>> {
    let mut curr_paths: HashSet<_> = [vec![start]].into_iter().collect();
    let mut new_paths = HashSet::new();
    while topography.get(*curr_paths.iter().next().unwrap().last().unwrap()) != 9 {
        for path in curr_paths {
            for candidate in topography.adjacent_iter(*path.last().unwrap()) {
                let new_path = path.iter().copied().chain([candidate]);
                new_paths.insert(new_path.collect());
            }
        }
        curr_paths = new_paths;
        new_paths = HashSet::new();
    }
    curr_paths
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
        if 0 > pos.x || pos.x >= self.width() as isize {
            255
        } else if 0 > pos.y || pos.y >= self.height() as isize {
            255
        } else {
            self.levels[pos.y as usize][pos.x as usize]
        }
    }
    pub fn adjacent_iter(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        let at = self.get(pos);
        let candidates = if at == 255 {
            vec![]
        } else {
            vec![pos.up(), pos.left(), pos.down(), pos.right()]
        };
        candidates.into_iter().filter(move |v| self.get(*v) == at + 1)
    }

    pub fn trailheads(&self) -> impl Iterator<Item = Pos> + '_ {
        self.levels.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(x, &cell)| {
                        (cell == 0).then_some(Pos { x: x as isize, y: y as isize })
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}
impl Pos {
    pub fn up(&self) -> Pos {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn down(&self) -> Pos {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn left(&self) -> Pos {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    pub fn right(&self) -> Pos {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
}
