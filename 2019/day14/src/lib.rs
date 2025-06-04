aoc_tools::aoc_sol!(day14 2019: part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Chemical<'a> {
    Ore,
    Fuel,
    Other(&'a str),
}
impl<'a> Chemical<'a> {
    pub fn parse(s: &'a str) -> Self {
        match s {
            "ORE" => Self::Ore,
            "FUEL" => Self::Fuel,
            v => Self::Other(v),
        }
    }
}


#[derive(Debug, Clone)]
struct Reactions<'a> {
    reactions: HashMap<Chemical<'a>, (i64, Vec<(i64, Chemical<'a>)>)>,
}
impl<'a> Reactions<'a> {
    pub fn ore_needed_for(&self, count: i64, chem: Chemical<'a>) -> i64 {
        let mut total_ore = 0;
        let mut has = HashMap::<Chemical, i64>::new();
        let mut needs: HashMap<_, _> = [(chem, count)].into_iter().collect();
        while let Some((&chem, &count)) = needs.iter().next() {
            needs.remove(&chem);

            if chem == Chemical::Ore {
                total_ore += count;
                continue
            }

            let has_ref = has.entry(chem).or_default();
            if *has_ref >= count {
                *has_ref -= count;
                continue
            }
            let mut count = count - *has_ref;
            
            let (added, inputs) = self.reactions.get(&chem).unwrap();
            let times = (count + added - 1) / added;
            count -= *added * times;
            for &(count, chem) in inputs {
                *needs.entry(chem).or_default() += count * times;
            }
            // println!("{count:?}");
            // while count > 0 {
            // }
            *has_ref = -count;
        }
        total_ore
    }
    pub fn parse(s: &'a str) -> Self {
        let reactions = s.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| {
                let (inputs, output) = l.split_once(" => ").unwrap();
                let inputs: Vec<_> = inputs.split(", ")
                    .map(|input| input.split_once(' ').unwrap())
                    .map(|(count, chem)| (count.parse::<i64>().unwrap(), Chemical::parse(chem)))
                    .collect();
                let (count, chem) = output.split_once(' ').unwrap();
                let count = count.parse::<i64>().unwrap();
                let chem = Chemical::parse(chem);
                (chem, (count, inputs))
            })
            .collect();
        Self { reactions }
    }
}

pub fn part1(input: &str) -> i64 {
    let reactions = parse_input(input);
    reactions.ore_needed_for(1, Chemical::Fuel)
}


const STORAGE: i64 = 1_000_000_000_000;
pub fn part2(input: &str) -> i64 {
    let reactions = parse_input(input);
    let for_one = reactions.ore_needed_for(1, Chemical::Fuel);
    let mut min = STORAGE / for_one;
    let mut max = min * 2;

    // Initial bounds
    while reactions.ore_needed_for(max, Chemical::Fuel) < STORAGE {
        min += STORAGE / for_one;
        max = min + STORAGE / for_one;
    }

    // Binary search
    while max > min {
        let new_test = (min + max) / 2;
        if reactions.ore_needed_for(new_test, Chemical::Fuel) <= STORAGE {
            min = new_test;
        } else {
            max = new_test - 1;
        }
    }

    min
}

fn parse_input(input: &str) -> Reactions<'_> {
    Reactions::parse(input)
}
