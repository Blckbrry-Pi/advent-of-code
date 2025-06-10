use aoc_tools::graph::Graph;

type Scalar = i8;

aoc_tools::aoc_sol!(day20 2019: part1, part2);
aoc_tools::map_struct!(Map of Cell { connections: HashMap<Portal, Vec<Pos>> }, pos Scalar; +y=>D);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Portal(u8, u8);
impl Debug for Portal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0 as char, self.1 as char)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Open,
    Closed,
    Space,
    Letter(u8),
}
impl Cell {
    pub fn parse(c: char) -> Self {
        match c {
            '.' => Self::Open,
            '#' => Self::Closed,
            ' ' => Self::Space,
            l => Self::Letter(l as u8),
        }
    }
}
impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Open => '.',
            Self::Closed => '#',
            Self::Space => ' ',
            Self::Letter(l) => *l as char,
        };
        write!(f, "{c}")
    }
}

impl Map {
    #[inline(never)]
    fn graph_at_levels(&self, g: &mut Graph<(Pos, u8), (), u16>, levels: u8) {
        let mut reference_graph = Graph::<(Pos, u8), (), u16>::new();
        for y in 0..self.height() as Scalar {
            for x in 0..self.width() as Scalar {
                let pos = Pos { x, y };
                if self.get_raw(pos) != Some(&Cell::Open) { continue }
                reference_graph.insert_or_update_node((pos, 0), ());
            }
        }
        for y in 0..self.height() as Scalar {
            for x in 0..self.width() as Scalar {
                let pos = Pos { x, y };
                if self.get_raw(pos) != Some(&Cell::Open) { continue }
                for offset in [Pos::U, Pos::D, Pos::L, Pos::R] {
                    let neighbor = pos.add(offset);
                    if self.get_raw(neighbor) != Some(&Cell::Open) { continue }
                    reference_graph.insert_edge(&(pos, 0), &(neighbor, 0), 1);
                }
            }
        }
        reference_graph.reduce_chains(|_| false);

        for (&node, _) in reference_graph.nodes() {
            for l in 0..levels {
                g.insert_or_update_node((node.0, l), ());
            }
        }
        for (&node, _) in reference_graph.nodes() {
            for (other_node, &edge_data) in reference_graph.edges_for(&node) {
                for l in 0..levels {
                    g.insert_edge(&(node.0, l), &(other_node.0, l), edge_data);
                }
            }
        }
    }
    pub fn graph_p1(&self) -> Graph<(Pos, u8), (), u16> {
        let mut graph = Graph::new();
        self.graph_at_levels(&mut graph, 1);
        for (_, connection) in &self.connections {
            if connection.len() == 2 {
                let a = (connection[0], 0);
                let b = (connection[1], 0);
                graph.insert_edge(&a, &b, 1);
            }
        }

        graph
    }

    #[inline(never)]
    pub fn graph_p2(&self) -> Graph<(Pos, u8), (), u16> {
        let mut graph = Graph::new();

        if self.connections.len() > u8::MAX as usize { panic!("A") }
        let layers = self.connections.len() as u8;
        self.graph_at_levels(&mut graph, layers);
        for (_, connection) in &self.connections {
            if connection.len() != 2 { continue }
            let a = connection[0];
            let b = connection[1];

            let a_is_outer = a.x == 2 || a.y == 2 || a.x + 3 == self.width() as Scalar || a.y + 3 == self.height() as Scalar;
            let outer = if a_is_outer { a } else { b };
            let inner = if a_is_outer { b } else { a };

            for l in 0..layers-1 {
                graph.insert_edge(&(inner, l), &(outer, l+1), 1);
            }
            for l in 1..layers {
                graph.insert_edge(&(outer, l), &(inner, l-1), 1);
            }
        }

        let start = self.start();
        let end = self.end();
        graph.reduce_chains(|v| v == start || v == end);

        graph
    }
    pub fn start(&self) -> (Pos, u8) {
        (self.connections.get(&Portal(b'A', b'A')).unwrap()[0], 0)
    }
    pub fn end(&self) -> (Pos, u8) {
        (self.connections.get(&Portal(b'Z', b'Z')).unwrap()[0], 0)
    }
}

pub fn part1(input: &str) -> u16 {
    let map = parse_input(input);
    let graph = map.graph_p1();
    graph.dijkstra(&map.start(), &map.end()).unwrap().0
}

pub fn part2(input: &str) -> u16 {
    let map = parse_input(input);
    let graph = map.graph_p2();
    graph.dijkstra(&map.start(), &map.end()).unwrap().0
}

fn parse_input(input: &str) -> Map {
    let rows = aoc_tools::parse_map(input, Cell::parse);

    let mut connections = HashMap::<Portal, Vec<Pos>>::new();
    for y in 0..rows.len() {
        for x in 0..rows[y].len() {
            let Cell::Letter(l1) = rows[y][x] else { continue };
            for offset in [Pos::U, Pos::D, Pos::L, Pos::R] {
                if x == 0 && offset.x != 0 { continue }
                if y == 0 && offset.y != 0 { continue }
                let nx = x.wrapping_add_signed(offset.x as isize);
                let ny = y.wrapping_add_signed(offset.y as isize);
                if x + offset.x.abs() as usize >= rows[y].len() { continue }
                if y + offset.y.abs() as usize >= rows.len() { continue }

                if rows[ny][nx] != Cell::Open { continue }

                let lx = x.wrapping_add_signed(-offset.x as isize);
                let ly = y.wrapping_add_signed(-offset.y as isize);
                let Cell::Letter(l2) = rows[ly][lx] else { panic!("Badly formatted input") };
                let portal = Portal(l1.min(l2), l1.max(l2));
                connections.entry(portal).or_default().push(Pos { x: nx as Scalar, y: ny as Scalar });
                // println!("Portal {:?} @ {:?}", Portal(l1, l2), Pos { x: nx as Scalar, y: ny as Scalar });
            }
        }
    }

    Map { rows, connections }
}
