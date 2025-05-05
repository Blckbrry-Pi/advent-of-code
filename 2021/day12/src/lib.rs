use std::str::FromStr;

aoc_tools::aoc_sol!(day12 2021: part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cave(u8);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum CaveName {
    Start,
    End,
    Other(u8, u8),
}
impl CaveName {
    pub fn is_big(&self) -> bool {
        match self {
            Self::Start | Self::End => false,
            Self::Other(a, _) => a.is_ascii_uppercase(),
        }
    }
    pub fn is_start_end(&self) -> bool {
        match self {
            Self::Start | Self::End => true,
            Self::Other(..) => false,
        }
    }
}
impl Debug for CaveName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "start"),
            Self::End => write!(f, "end"),
            Self::Other(a, b) => write!(f, "{}{}", *a as char, *b as char),
        }
    }
}
impl FromStr for CaveName {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "start" {
            Ok(Self::Start)
        } else if s == "end" {
            Ok(Self::End)
        } else {
            let a = s.as_bytes()[0];
            let b = if s.len() > 1 {
                s.as_bytes()[1]
            } else {
                b'\0'
            };
            Ok(Self::Other(a, b))
        }
    }
}

#[derive(Clone)]
struct CaveMapper {
    curr_val: u8,
    to_name: HashMap<u8, CaveName>,
    to_cave: HashMap<CaveName, u8>,
    big_caves: u64,
    start: Cave,
    end: Cave,
}
impl CaveMapper {
    fn get_cave(&self, name: CaveName) -> Option<Cave> {
        self.to_cave.get(&name).copied().map(Cave)
    }
    fn get_cave_defaulting(&mut self, name: CaveName) -> Cave {
        if let Some(cave) = self.get_cave(name) { return cave }
        let new_cave = self.curr_val;
        self.curr_val += 1;
        self.to_name.insert(new_cave, name);
        self.to_cave.insert(name, new_cave);

        if name.is_big() {
            self.big_caves |= 1 << new_cave;
        } else if name == CaveName::Start {
            self.start = Cave(new_cave);
        } else if name == CaveName::End {
            self.end = Cave(new_cave);
        }

        Cave(new_cave)
    }
    fn get_name(&self, cave: Cave) -> Option<CaveName> {
        self.to_name.get(&cave.0).copied()
    }
    fn is_big(&self, cave: Cave) -> bool {
        (self.big_caves >> cave.0) & 1 != 0
    }
    fn is_start_end(&self, cave: Cave) -> bool {
        self.start == cave || self.end == cave
    }
}
impl Debug for CaveMapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_cave.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State(u32, bool);
impl State {
    pub fn with_cave(self, cave: Cave, is_big: bool) -> Self {
        let used = !is_big && self.has_cave(cave);
        Self(self.0 | (1 << cave.0), used || self.1)
    }
    pub fn has_cave(&self, cave: Cave) -> bool {
        (self.0 >> cave.0) & 1 != 0
    }
    pub fn extra_used(&self) -> bool {
        self.1
    }
}

#[derive(Clone)]
struct Map {
    mapper: CaveMapper,
    tunnels: HashMap<Cave, Vec<Cave>>,
}
impl Map {
    pub fn start(&self) -> Cave {
        self.mapper.start
    }
    pub fn end(&self) -> Cave {
        self.mapper.end
    }
    pub fn paths(&self, allow_single_double: bool) -> u32 {
        let mut paths_finished = 0;
        let mut working_paths: HashMap<_, _> = [(
            (State(0, false), self.start()),
            1,
        )].into_iter().collect();
        while let Some((&(seen, now_at), &count)) = working_paths.iter().next() {
            working_paths.remove(&(seen, now_at));
            if now_at == self.end() {
                paths_finished += count;
            }

            let seen = seen.with_cave(now_at, self.mapper.is_big(now_at));
            for &neighbor in self.tunnels.get(&now_at).unwrap() {
                if seen.has_cave(neighbor) && !self.mapper.is_big(neighbor) {
                    if !allow_single_double || seen.extra_used() || self.mapper.is_start_end(neighbor) {
                        continue
                    }
                }
                *working_paths.entry((seen, neighbor)).or_default() += count;
            }
        }
        paths_finished
    }
}
impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct ConnectionsFormatter<'a>(&'a [Cave], &'a CaveMapper);
        impl<'a> Debug for ConnectionsFormatter<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for (i, &cave) in self.0.iter().enumerate() {
                    let name = self.1.get_name(cave).unwrap();
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{name:?}")?;
                }
                Ok(())
            }
        }

        let mut formatter = f.debug_map();
        for (&location, connections) in &self.tunnels {
            let location = self.mapper.get_name(location).unwrap();
            formatter.entry(
                &location,
                &ConnectionsFormatter(&connections, &self.mapper),
            );
        }
        formatter.finish()
    }
}
impl FromStr for Map {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut output = Self {
            mapper: CaveMapper {
                curr_val: 0,
                to_name: HashMap::new(),
                to_cave: HashMap::new(),
                big_caves: 0,
                start: Cave(0),
                end: Cave(0),
            },
            tunnels: HashMap::new(),
        };
        for line in s.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
            let Some((a, b)) = line.split_once('-') else {
                return Err(format!("Bad line {line:?}"));
            };
            let a: CaveName = a.parse()?;
            let b: CaveName = b.parse()?;
            let a = output.mapper.get_cave_defaulting(a);
            let b = output.mapper.get_cave_defaulting(b);
            output.tunnels.entry(a).or_default().push(b);
            output.tunnels.entry(b).or_default().push(a);
        }
        Ok(output)
    }
}

pub fn part1(input: &str) -> u32 {
    let map = parse_input(input);
    map.paths(false)
}

pub fn part2(input: &str) -> u32 {
    let map = parse_input(input);
    map.paths(true)
}

fn parse_input(input: &str) -> Map {
    input.parse().unwrap()
}
