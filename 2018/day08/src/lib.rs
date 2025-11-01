aoc_tools::aoc_sol!(day08 2018: part1, part2);


#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u8>,
    value: u32,
}
impl Node {
    pub fn from_children_and_meta(children: Vec<Node>, metadata: Vec<u8>) -> Self {
        let mut output = Self { children, metadata, value: 0 };
        output.update_value();
        output
    }
    pub fn metadata_sum(&self) -> u32 {
        let self_sum = self.metadata.iter().map(|v| *v as u32).sum::<u32>();
        self.children.iter().map(|c| c.metadata_sum()).sum::<u32>() + self_sum
    }
    pub fn update_value(&mut self) {
        if self.children.is_empty() {
            self.value = self.metadata.iter().map(|v| *v as u32).sum();
        } else {
            self.value = 0;
            for idx in self.metadata.iter().copied() {
                let Some(child) = self.children.get(idx as usize - 1) else { continue };
                self.value += child.value;
            }
        }
    }
}

fn parse_node(input: &[u8]) -> (Node, &[u8]) {
    let child_count = input[0];
    let meta_count = input[1];
    let mut remaining = &input[2..];
    let mut children = Vec::<Node>::with_capacity(child_count as usize);
    for _ in 0..child_count {
        let (child, new_remaining) = parse_node(remaining);
        children.push(child);
        remaining = new_remaining;
    }
    let metadata = remaining[..meta_count as usize].to_vec();
    (Node::from_children_and_meta(children, metadata), &remaining[meta_count as usize..])
}

pub fn part1(input: &str) -> u32 {
    let data = parse_input(input);
    let base = parse_node(&data).0;
    base.metadata_sum()
}

pub fn part2(input: &str) -> u32 {
    let data = parse_input(input);
    let base = parse_node(&data).0;
    base.value
}

fn parse_input(input: &str) -> Vec<u8> {
    input.split(' ').map(|s| s.trim().parse::<u8>().unwrap()).collect()
}
