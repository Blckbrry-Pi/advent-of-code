aoc_tools::aoc_sol!(day23: part1, part2);

pub fn part1(input: &str) -> usize {
    let connections = parse_input(input);
    let mut t_group_count = 0;

    for a in connections.keys() {
        let a_conns = connections.get(a).unwrap();

        for b in a_conns.iter().filter(|c| *c > a) {
            let b_conns = connections.get(b).unwrap();

            for c in b_conns.iter().filter(|c| *c > b) {
                if a_conns.contains(c) {
                    if a.starts_with_t() || b.starts_with_t() || c.starts_with_t() {
                        t_group_count += 1;
                    }
                }
            }
        }
    }
    t_group_count
}

pub fn part2(input: &str) -> String {
    let connections = parse_input(input);
    let mut groups: Vec<_> = connections.keys().map(|&c| vec![c]).collect();

    let mut biggest_seen = vec![];
    while let Some(group) = groups.pop() {
        // Get non-group computers that all computers in the group have direct
        // connections to
        let mut new_possible_connections: HashSet<_> = connections.get(&group[0]).unwrap().clone();
        for other_computer_in_group in group.iter().rev() {
            let other_computer_connections = connections.get(other_computer_in_group).unwrap();
            new_possible_connections.retain(|c| c > other_computer_in_group && other_computer_connections.contains(c));
        }
        // Optimization: If the biggest seen is larger than an expanded group
        // will ever be, discard this expanded group
        if new_possible_connections.len() + group.len() < biggest_seen.len() { continue; }

        for new_connection in new_possible_connections {
            let mut new_group = group.clone();
            new_group.push(new_connection);

            if biggest_seen.len() < new_group.len() {
                biggest_seen = new_group.clone();
            }
            groups.push(new_group);
        }
    }

    let mut output = biggest_seen
        .into_iter().map(|c| format!("{c:?},"))
        .collect::<String>();
    output.pop();

    output
}

fn parse_input(input: &str) -> HashMap<Computer, HashSet<Computer>> {
    let lines = (input.len() + 1) / 6;
    let comp_count = (2.0 * (lines as f64).powf(0.7)) as usize;
    let conn_count = (comp_count as f64).sqrt() as usize;
    let mut connections: HashMap<Computer, HashSet<Computer>> = HashMap::with_capacity(comp_count);
    for i in 0..lines {
        let line = &input[i*6 .. i*6+5];
        let (a, b) = (&line[0..2], &line[3..5]);
        let (a, b) = (Computer::from_str(a), Computer::from_str(b));

        connections.entry(a).or_insert_with(|| HashSet::with_capacity(conn_count)).insert(b);
        connections.entry(b).or_insert_with(|| HashSet::with_capacity(conn_count)).insert(a);
    }
    connections
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Computer(u8, u8);
impl Computer {
    fn from_str(s: &str) -> Self {
        Self(s.as_bytes()[0], s.as_bytes()[1])
    }

    fn starts_with_t(&self) -> bool {
        self.0 == b't'
    }
}

impl Debug for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0 as char, self.1 as char)
    }
}
