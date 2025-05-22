const DEBUG: bool = false;

#[derive(Clone)]
struct CupRing {
    next_indicies: Vec<u32>,
    curr: u32,
    max: u32,
}
impl CupRing {
    pub fn from_list(list: &[u32]) -> Self {
        let mut output = Self {
            next_indicies: vec![u32::MAX; list.len()+1],
            curr: list[0],
            max: *list.iter().max().unwrap(),
        };
        for i in 0..list.len() {
            output.next_indicies[list[i] as usize] = list[(i+1) % list.len()];
        }
        output
    }
    pub fn take_3(&mut self) -> [u32; 3] {
        let a = self.next_indicies[self.curr as usize];
        let b = self.next_indicies[a as usize];
        let c = self.next_indicies[b as usize];
        let after = self.next_indicies[c as usize];
        self.next_indicies[self.curr as usize] = after;
        [a, b, c]
    }
    pub fn insert_3_after(&mut self, at: u32, values: [u32; 3]) {
        let after = self.next_indicies[at as usize];
        self.next_indicies[at as usize] = values[0];
        self.next_indicies[values[0] as usize] = values[1];
        self.next_indicies[values[1] as usize] = values[2];
        self.next_indicies[values[2] as usize] = after;
    }
    pub fn find_insertion_pos(&self, values: [u32; 3]) -> u32 {
        for i in (1..self.curr).rev().chain((1..=self.max).rev()) {
            if i == self.curr { continue }
            if values.contains(&i) { continue }
            return i;
        }
        panic!("Invalid ring");
    }
    pub fn do_step(&mut self) {
        if DEBUG { println!("cups: {self:?}") }
        let taken = self.take_3();
        if DEBUG { println!("pick up: {} {} {}", taken[0], taken[1], taken[2]) }
        let target = self.find_insertion_pos(taken);
        if DEBUG { println!("destination: {target}") }
        self.insert_3_after(target, taken);
        self.curr = self.next_indicies[self.curr as usize];
    }
    pub fn labels(&self) -> u64 {
        let start = 1;

        let mut output = 0;
        let mut curr = self.next_indicies[start as usize];
        while curr != start {
            output *= 10;
            output += curr as u64;
            curr = self.next_indicies[curr as usize];
        }
        output
    }
    pub fn cw_2_prod(&self) -> u64 {
        let first = self.next_indicies[1];
        let last = self.next_indicies[first as usize];
        first as u64 * last as u64
    }
}
impl Debug for CupRing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.curr)?;
        let mut curr = self.next_indicies[self.curr as usize];
        while curr != self.curr {
            write!(f, " {curr}")?;
            curr = self.next_indicies[curr as usize];
        }
        Ok(())
    }
}

aoc_tools::aoc_sol!(day23 2020: part1, part2);

pub fn part1(input: &str) -> u64 {
    let values = parse_input(input);
    let mut ring = CupRing::from_list(&values);
    for i in 0..100 {
        if DEBUG { println!("-- move {} --", i + 1) }
        ring.do_step();
        if DEBUG { println!() }
    }
    ring.labels()
}

pub fn part2(input: &str) -> u64 {
    let mut values = parse_input(input);
    values.extend(values.len() as u32 + 1..=1_000_000);

    let mut ring = CupRing::from_list(&values);
    for i in 0..10_000_000 {
        if DEBUG { println!("-- move {} --", i + 1) }
        ring.do_step();
        if DEBUG { println!() }
    }
    ring.cw_2_prod()
}

fn parse_input(input: &str) -> Vec<u32> {
    input.trim().bytes().map(|b| b - b'0').map(|b| b as u32).collect()
}
