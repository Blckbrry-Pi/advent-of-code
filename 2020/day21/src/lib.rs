aoc_tools::aoc_sol!(day21 2020: part1, part2);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Ingredient<'a>(&'a str);
impl<'a> Debug for Ingredient<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Allergen<'a>(&'a str);
impl<'a> Debug for Allergen<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Line<'a>(Vec<Ingredient<'a>>, Vec<Allergen<'a>>);
impl<'a> Line<'a> {
    fn parse(s: &'a str) -> Self {
        let (ingredients, allergens) = s.split_once(" (contains ").unwrap();
        let allergens = allergens.trim_end_matches(')');
        let ingredients: Vec<_> = ingredients.split(' ').map(Ingredient).collect();
        let allergens: Vec<_> = allergens.split(", ").map(Allergen).collect();
        Self(ingredients, allergens)
    }
}
impl<'a> Debug for Line<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ingredient in &self.0 {
            write!(f, "{ingredient:?} ")?;
        }
        write!(f, "(contains ")?;
        for i in 0..self.1.len() {
            if i != 0 { write!(f, " ")?; }
            write!(f, "{:?}", self.1[i])?;
        }
        write!(f, ")")
    }
}

fn get_ingredient_allergen_map<'a>(lines: &[Line<'a>]) -> HashMap<Ingredient<'a>, Allergen<'a>> {
    let mut allergen_map = HashMap::<Allergen, HashSet<Ingredient>>::new();
    for line in lines {
        for allergen in &line.1 {
            if let Some(possibilities) = allergen_map.get_mut(&allergen) {
                *possibilities = possibilities.iter()
                    .filter(|possibility| line.0.contains(&possibility))
                    .copied()
                    .collect();
            } else {
                allergen_map.insert(*allergen, line.0.iter().copied().collect());
            }
        }
    }

    let mut taken = HashSet::new();
    loop {
        let mut something_changed = false;
        for (_, ingredients) in allergen_map.iter_mut() {
            if ingredients.len() == 1 {
                if taken.insert(ingredients.iter().next().copied().unwrap()) {
                    something_changed = true;
                }
            } else {
                let new_ingredients: HashSet<_> = ingredients.iter()
                    .filter(|i| !taken.contains(&i))
                    .copied()
                    .collect();
                if new_ingredients.len() != ingredients.len() {
                    something_changed = true;
                }
                *ingredients = new_ingredients;
            }
        }
        if !something_changed { break }
    }

    allergen_map.into_iter()
        .map(|(a, i)| (i.into_iter().next().unwrap(), a))
        .collect()
}

pub fn part1(input: &str) -> i64 {
    let lines = parse_input(input);
    let map = get_ingredient_allergen_map(&lines);

    let mut good_ingredient_count = 0;
    for line in lines {
        for ingredient in line.0 {
            if !map.contains_key(&ingredient) {
                good_ingredient_count += 1;
            }
        }
    }

    good_ingredient_count
}

pub fn part2(input: &str) -> String {
    let lines = parse_input(input);
    let map = get_ingredient_allergen_map(&lines);

    let mut list: Vec<_> = map.into_iter().collect();
    list.sort_by(|a, b| a.1.cmp(&b.1));
    let list: Vec<_> = list.into_iter().map(|pair| pair.0.0).collect();

    list.join(",")
}

fn parse_input<'a>(input: &'a str) -> Vec<Line<'a>> {
    input.lines().filter(|l| !l.is_empty()).map(Line::parse).collect()
}
