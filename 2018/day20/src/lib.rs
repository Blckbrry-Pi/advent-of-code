use std::collections::VecDeque;

aoc_tools::aoc_sol!(day20 2018: part1, part2);
aoc_tools::map_struct!(Map of bool { doors: HashMap<Pos, Vec<Pos>>, start: Pos }, pos Scalar; +y => D);
type Scalar = i16;

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl Map {
    pub fn from_nfa(nfa: &Nfa) -> Self {
        let (pos_set, connections) = nfa.traverse();
        Self::from_pos_set_and_connections(&pos_set, &connections)
    }
    pub fn from_pos_set_and_connections(pos_set: &HashSet<Pos>, connections: &HashSet<(Pos, Pos)>) -> Self {
        let (x_range, y_range) = pos_set.iter().fold(
            ((0, 0), (0, 0)),
            |(x_range, y_range), pos| (
                (x_range.0.min(pos.x), x_range.1.max(pos.x)),
                (y_range.0.min(pos.y), y_range.1.max(pos.y)),
            ),
        );
        let mut output = Map {
            rows: Vec::with_capacity((y_range.1 - y_range.0 + 1) as usize),
            doors: HashMap::new(),
            start: Pos { x: -x_range.0, y: -y_range.0 },
        };
        for y in y_range.0..=y_range.1 {
            output.rows.push(Vec::with_capacity((x_range.1 - x_range.0 + 1) as usize));
            for x in x_range.0..=x_range.1 {
                let has_pos = pos_set.contains(&Pos { x, y });
                output.rows.last_mut().unwrap().push(has_pos);
            }
        }
        let offset = Pos { x: x_range.0, y: y_range.0 };
        for &(a, b) in connections {
            let (a, b) = (a.sub(offset), b.sub(offset));
            output.doors.entry(a).or_default().push(b);
            output.doors.entry(b).or_default().push(a);
        }

        output
    }
    pub fn largest_shortest_walk(&self) -> usize {
        let mut seen = HashSet::new();
        let mut curr = vec![self.start];
        for i in 0.. {
            let mut new_curr = HashSet::new();
            for pos in curr {
                if seen.contains(&pos) { continue }
                seen.insert(pos);
                for &connection in self.doors.get(&pos).unwrap() {
                    new_curr.insert(connection);
                }
            }
            curr = new_curr.into_iter().filter(|v| !seen.contains(v)).collect();
            if curr.is_empty() { return i }
        }
        unreachable!()
    }
    pub fn door_count_at(&self, steps: usize) -> usize {
        let mut seen = HashSet::new();
        let mut curr = vec![self.start];
        for i in 0..steps {
            let mut new_curr = HashSet::new();
            for pos in curr {
                if seen.contains(&pos) { continue }
                seen.insert(pos);
                for &connection in self.doors.get(&pos).unwrap() {
                    new_curr.insert(connection);
                }
            }
            curr = new_curr.into_iter().filter(|v| !seen.contains(v)).collect();
            if curr.is_empty() { return i }
        }
        self.filled_doors() - seen.len()
    }
    pub fn filled_doors(&self) -> usize {
        self.rows.iter().map(|r| r.iter().filter(|c| **c).count()).sum()
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..=2*self.width() {
            write!(f, "#")?;
        }
        writeln!(f)?;
        for y in 0..self.height() as Scalar {
            write!(f, "#")?;
            for x in 0..self.width() as Scalar {
                let pos = Pos { x, y };
                let cell = *self.get_raw(pos).unwrap();
                if pos == self.start {
                    write!(f, "X")
                } else if cell {
                    write!(f, ".")
                } else {
                    write!(f, "#")
                }?;
                let e_neighbor = pos.add(Pos::E);
                let e_is_door = self.doors.get(&pos).map(|l| l.contains(&e_neighbor)).unwrap_or(false);
                if e_is_door {
                    write!(f, "|")
                } else {
                    write!(f, "#")
                }?;
            }
            writeln!(f)?;
            write!(f, "#")?;
            for x in 0..self.width() as Scalar {
                let pos = Pos { x, y };
                let s_neighbor = pos.add(Pos::S);
                let s_is_door = self.doors.get(&pos).map(|l| l.contains(&s_neighbor)).unwrap_or(false);
                if s_is_door {
                    write!(f, "-")
                } else {
                    write!(f, "#")
                }?;
                write!(f, "#")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Default)]
struct Transitions {
    epsilon: HashSet<u32>,
    chars: [HashSet<u32>; 4],
}
impl Transitions {
    pub fn apply(&self, pos: Pos) -> impl Iterator<Item = (u32, Pos)> + '_ {
        self.chars.iter()
            .zip([DirChr::N, DirChr::E, DirChr::S, DirChr::W].map(|c| c.to_pos()))
            .chain([(&self.epsilon, Pos::ZERO)])
            .flat_map(move |(new_states, offset)| {
                new_states.iter().map(move |new_state| (*new_state, pos.add(offset)))
            })
    }
}
impl Debug for Transitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char_iter = [DirChr::N, DirChr::E, DirChr::S, DirChr::W]
            .into_iter()
            .map(|c| c.to_char())
            .zip(&self.chars)
            .rev()
            .chain([('ε', &self.epsilon)])
            .rev()
            .filter(|(_, targets)| !targets.is_empty());
        for (i, (c, targets)) in char_iter.enumerate() {
            if i != 0 { write!(f, "; ")?; }

            write!(f, "{c} => [")?;
            for (i, target) in targets.iter().enumerate() {
                if i != 0 { write!(f, ", ")?; }
                write!(f, "{target}")?;
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}


type Path = Vec<u8>;

struct Nfa(HashMap<u32, Transitions>, u32);
impl Nfa {
    fn from_regex(r: &Regex) -> Self {
        let mut output = Self(HashMap::new(), 0);
        output.0.entry(0).or_default();

        let mut path = Path::with_capacity(512);
        let mut states = HashMap::new();
        states.insert(path.clone(), 0);

        let accept = output.add_regex(
            r,
            &mut states,
            &mut 1,
            0,
            &mut path,
        );
        output.1 = accept;

        output.reduce_epsilons();

        output
    }
    fn add_regex(
        &mut self,
        r: &Regex,
        states: &mut HashMap<Path, u32>,
        next_state: &mut u32,
        parent_state: u32,
        path: &mut Path,
    ) -> u32 {
        let mut last_state = parent_state;
        path.push(0);
        for i in 0..r.0.len() {
            let inner_last_state = self.add_regex_part(&r.0[i], states, next_state, last_state, path);

            path.pop();
            path.push(i as u8 + 1);
            last_state = *next_state;
            *next_state += 1;
            assert!(states.insert(path.clone(), last_state).is_none(), "{:#?}\n{:?}\n{:?}", self, states, path);
            self.0.entry(last_state).or_default();

            self.0.get_mut(&inner_last_state).unwrap().epsilon.insert(last_state);
        }
        path.pop();

        last_state
    }
    fn add_regex_part(
        &mut self,
        r: &RegexPart,
        states: &mut HashMap<Path, u32>,
        next_state: &mut u32,
        parent_state: u32,
        path: &mut Path,
    ) -> u32 {
        match r {
            RegexPart::Char(c) => {
                path.push(1);
                let inner_state = *next_state;
                *next_state += 1;
                assert!(states.insert(path.clone(), inner_state).is_none());
                self.0.entry(inner_state).or_default();
                path.pop();

                self.0.get_mut(&parent_state).unwrap().chars[*c as usize].insert(inner_state);
                inner_state
            }
            RegexPart::Branch(branches) => {
                path.push(branches.len() as u8);
                let joining_state = *next_state;
                *next_state += 1;
                assert!(states.insert(path.clone(), joining_state).is_none());
                path.pop();

                self.0.entry(joining_state).or_default();

                for i in 0..branches.len() {
                    path.push(i as u8);
                    let inner_last_state = self.add_regex(&branches[i], states, next_state, parent_state, path);
                    self.0.get_mut(&inner_last_state).unwrap().epsilon.insert(joining_state);
                    path.pop();
                }

                joining_state
            },
        }
    }

    fn reduce_epsilons(&mut self) {
        fn memoized_get(remaps: &mut HashMap<u32, u32>, v: u32) -> u32 {
            if let Some(&new_v) = remaps.get(&v) {
                let new_v = memoized_get(remaps, new_v);
                remaps.insert(v, new_v);
                new_v
            } else {
                v
            }
        }
        let mut remaps = HashMap::new();
        for (state, transitions) in &self.0 {
            if transitions.chars.iter().all(|t| t.is_empty()) && transitions.epsilon.len() == 1 {
                remaps.insert(*state, *transitions.epsilon.iter().next().unwrap());
            }
        }

        for (_, transitions) in &mut self.0 {
            let new_epsilon: HashSet<_> = transitions.epsilon
                .drain()
                .map(|t| memoized_get(&mut remaps, t))
                .collect();
            transitions.epsilon = new_epsilon;

            for i in 0..4 {
                let new_chars: HashSet<_> = transitions.chars[i]
                    .drain()
                    .map(|t| memoized_get(&mut remaps, t))
                    .collect();
                transitions.chars[i] = new_chars;
            }
        }
        self.0.retain(|v, _| !remaps.contains_key(v))
    }

    fn traverse(&self) -> (HashSet<Pos>, HashSet<(Pos, Pos)>) {
        let mut doors = HashSet::<(Pos, Pos)>::new();
        let mut seen = HashSet::<(u32, Pos)>::new();
        let mut queue = VecDeque::<(u32, Pos)>::new();
        queue.push_back((0, Pos::ZERO));
        while let Some((state, pos)) = queue.pop_front() {
            if seen.contains(&(state, pos)) { continue }
            seen.insert((state, pos));
            let transitions = self.0.get(&state).unwrap();
            for (new_state, new_pos) in transitions.apply(pos) {
                if seen.contains(&(new_state, new_pos)) { continue }
                if new_pos != pos {
                    doors.insert((pos.min(new_pos), pos.max(new_pos)));
                }
                queue.push_back((new_state, new_pos));
            }
        }
        let seen = seen.into_iter().map(|(_, pos)| pos).collect();
        (seen, doors)
    }
}

impl Debug for Nfa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nfa ")?;
        let mut formatter = f.debug_map();
        let mut seen = 0;
        for i in 0.. {
            let Some(state) = self.0.get(&i) else { continue };
            if i == self.1 {
                formatter.entry(&i, &"ACCEPT");
            } else {
                formatter.entry(&i, state);
            }

            seen += 1;
            if seen >= self.0.len() { break }
        }
        formatter.finish()
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Regex(Vec<RegexPart>);
impl Regex {
    pub fn parse_parenthesized(s: &str) -> (RegexPart, &str) {
        if s.as_bytes()[0] != b'(' { panic!("Expected opening paren"); }
        let s = &s[1..];
        let (v, s) = Self::parse_branch(s);
        if s.as_bytes()[0] != b')' { panic!("Expected closing paren"); }
        (v, &s[1..])
    }
    pub fn parse_branch(s: &str) -> (RegexPart, &str) {
        let mut branches = vec![];
        let mut remaining = s;
        loop {
            let (branch, new_remaining) = Self::parse(remaining);
            remaining = new_remaining;
            branches.push(branch);
            if remaining.as_bytes().get(0) == Some(&b'|') {
                remaining = &remaining[1..];
            } else {
                break;
            }
        }
        (RegexPart::Branch(branches), remaining)
    }
    pub fn parse(s: &str) -> (Self, &str) {
        let mut remaining = s;
        let mut parts = vec![];
        while let Some(&c) = remaining.as_bytes().get(0) {
            match c {
                b')' | b'|' | b'$' => break,
                b'(' => {
                    let (part, new_remaining) = Self::parse_parenthesized(remaining);
                    remaining = new_remaining;
                    parts.push(part);
                }
                c => {
                    remaining = &remaining[1..];
                    parts.push(RegexPart::Char(DirChr::from_char(c as char)));
                }
            }
        }
        (Self(parts), remaining)
    }
    pub fn parse_full(s: &str) -> Self {
        let s = s.strip_prefix('^').unwrap();
        let (regex, s) = Self::parse(s);
        assert_eq!(s, "$");
        regex
    }
}
impl Debug for Regex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for part in &self.0 {
            write!(f, "{part:?}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirChr { N = 0, E = 1, S = 2, W = 3 }
impl DirChr {
    pub fn to_pos(&self) -> Pos {
        match self {
            Self::N => Pos::N,
            Self::E => Pos::E,
            Self::S => Pos::S,
            Self::W => Pos::W,
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            Self::N => 'N',
            Self::E => 'E',
            Self::S => 'S',
            Self::W => 'W',
        }
    }
    pub fn from_char(c: char) -> Self {
        match c {
            'N' => Self::N,
            'E' => Self::E,
            'S' => Self::S,
            'W' => Self::W,
            _ => panic!("Invalid character")
        }
    }
}


#[derive(Clone, PartialEq, Eq)]
enum RegexPart {
    Char(DirChr),
    Branch(Vec<Regex>),
}
impl Debug for RegexPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Char(c) => write!(f, "{c:?}"),
            Self::Branch(branches) => {
                write!(f, "(")?;
                for (i, branch) in branches.iter().enumerate() {
                    if i != 0 { write!(f, "|")?; }
                    write!(f, "{branch:?}")?;
                }
                write!(f, ")")?;
                Ok(())
            },
        }
    }
}

#[inline(never)]
pub fn part1(input: &str) -> usize {
    let regex = parse_input(input);
    let nfa = Nfa::from_regex(&regex);
    let map = Map::from_nfa(&nfa);
    map.largest_shortest_walk()
}

#[inline(never)]
pub fn part2(input: &str) -> usize {
    let regex = parse_input(input);
    let nfa = Nfa::from_regex(&regex);
    let map = Map::from_nfa(&nfa);
    map.door_count_at(1000)
}

fn parse_input(input: &str) -> Regex {
    Regex::parse_full(input.trim())
}
