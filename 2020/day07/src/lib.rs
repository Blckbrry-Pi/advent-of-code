use std::{rc::Rc, str::FromStr};

aoc_tools::aoc_sol!(day07 2020: part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BagType<'a>(&'a str);
impl BagType<'static> {
    fn shiny_gold() -> Self {
        Self("shiny gold")
    }
}

struct Rules<'a>(HashMap<BagType<'a>, Vec<(u32, BagType<'a>)>>);
impl<'a> Rules<'a> {
    pub fn contains_shiny_gold(&self, ty: BagType<'a>, memo: &mut HashMap<BagType<'a>, bool>) -> bool {
        if let Some(output) = memo.get(&ty) {
            return *output;
        }
        let output = self.0.get(&ty)
            .unwrap()
            .iter()
            .any(|(_, ty)| ty == &BagType::shiny_gold() || self.contains_shiny_gold(*ty, memo));
        memo.insert(ty, output);
        output
    }
    pub fn count_inside(&self, ty: BagType<'a>, memo: &mut HashMap<BagType<'a>, u32>) -> u32 {
        if let Some(output) = memo.get(&ty) {
            return *output;
        }
        let output = self.0.get(&ty)
            .unwrap()
            .iter()
            .map(|(count, ty)| *count * (1 + self.count_inside(*ty, memo)))
            .sum();
        memo.insert(ty, output);
        output
    }
    pub fn add(&mut self, rule: Rule<'a>) {
        self.0.insert(rule.0, rule.1);
    }
}

struct Rule<'a>(BagType<'a>, Vec<(u32, BagType<'a>)>);
impl<'a> Rule<'a> {
    pub fn parse(input: &'a str) -> Self {
        let (ty, contained) = input.split_once(" bags contain ").unwrap();
        let ty = BagType(ty);
        if contained == "no other bags." { return Self(ty, vec![]); }
        let mut contained_list = vec![];
        for partial_contained in contained.split(", ") {
            let partial_contained = partial_contained
                .trim_end_matches('.')
                .trim_end_matches('s')
                .trim_end_matches(" bag");
            let (count, ty) = partial_contained.split_once(' ').unwrap();
            let count: u32 = count.parse().unwrap();
            contained_list.push((count, BagType(ty)));
        }
        Self(ty, contained_list)
    }
}

pub fn part1(input: &str) -> u16 {
    let rules = parse_input(input);
    let mut memo = HashMap::new();
    let mut count = 0;
    for &bag in rules.0.keys() {
        if rules.contains_shiny_gold(bag, &mut memo) {
            count += 1;
        }
    }
    count
}

pub fn part2(input: &str) -> u32 {
    let rules = parse_input(input);
    rules.count_inside(BagType::shiny_gold(), &mut HashMap::new())
}

fn parse_input(input: &str) -> Rules<'_> {
    let mut rules = Rules(HashMap::new());
    input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| Rule::parse(line))
        .for_each(|rule| rules.add(rule));
    rules
}
