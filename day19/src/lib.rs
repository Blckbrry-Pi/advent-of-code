aoc_tools::aoc_sol!(day19: part1, part2);

pub fn part1(input: &str) -> usize {
    let (towels,  patterns) = parse_input(input);

    let mut count = 0;
    for pattern in patterns {
        if pattern.solve(&towels) != 0 {
            count += 1;
        }
    }
    count
}

pub fn part2(input: &str) -> usize {
    let (towels,  patterns) = parse_input(input);

    let mut count = 0;
    for pattern in patterns {
        count += pattern.solve(&towels);
    }
    count
}

fn parse_input(input: &str) -> (ColorTrie, Vec<Pattern>) {
    let (towels, patterns) = input.trim().split_once("\n\n").unwrap();
    let towels = ColorTrie::build(towels.split(", ").map(Pattern::parse));

    let patterns = patterns.lines().map(Pattern::parse).collect();

    (towels, patterns)
}

#[derive(Clone, PartialEq, Eq)]
struct Pattern(Vec<Color>);
impl Pattern {
    pub fn parse(s: &str) -> Self {
        Self(s.chars().map(Color::from_char).collect())
    }

    pub fn solve(&self, trie: &ColorTrie) -> usize {
        fn solve_recursive(remaining: &[Color], trie: &ColorTrie, count_cache: &mut HashMap<usize, usize>) -> usize {
            if remaining.is_empty() { return 1; }
            if let Some(cached_count) = count_cache.get(&remaining.len()) {
                return *cached_count;
            }
            let mut count = 0;
            for new_remaining in trie.stripped(remaining) {
                // let bit_len = remaining.len() - new_remaining.len();
                let solutions = solve_recursive(new_remaining, trie, count_cache);
                count += solutions;
            }
            count_cache.insert(remaining.len(), count);
            count
        }

        solve_recursive(&self.0, trie, &mut HashMap::new())
    }
}
impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for color in &self.0 {
            write!(f, "{color:?}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Color { W = 0, U = 1, B = 2, R = 3, G = 4 }

impl Color {
    const COLOR_COUNT: usize = 5;

    fn from_char(c: char) -> Self {
        match c {
            'w' => Self::W,
            'u' => Self::U,
            'b' => Self::B,
            'r' => Self::R,
            'g' => Self::G,
            _ => panic!("Invalid color character"),
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::W => 'w',
            Self::U => 'u',
            Self::B => 'b',
            Self::R => 'r',
            Self::G => 'g',
        }
    }

    fn from_u8(v: u8) -> Self {
        use Color::*;
        [W, U, B, R, G][v as usize]
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone)]
struct ColorTrie {
    children: [Option<Box<ColorTrie>>; Color::COLOR_COUNT],
    is_leaf: bool,
}
impl ColorTrie {
    pub fn build(patterns: impl IntoIterator<Item = Pattern>) -> Self {
        let mut output = Self {
            children: [const { None }; Color::COLOR_COUNT],
            is_leaf: false,
        };
        for pattern in patterns {
            output.add(&pattern.0);
        }
        output
    }
    pub fn add(&mut self, remaining: &[Color]) {
        if remaining.is_empty() {
            self.is_leaf = true;
        } else {
            let child = if let Some(c) = &mut self.children[remaining[0] as u8 as usize] {
                c
            } else {
                self.children[remaining[0] as u8 as usize] = Some(Box::new(ColorTrie {
                    children: [const { None }; Color::COLOR_COUNT],
                    is_leaf: false,
                }));
                self.children[remaining[0] as u8 as usize].as_mut().unwrap()
            };
            child.add(&remaining[1..]);
        }
    }
    pub fn stripped<'a: 'b, 'b>(&'b self, starting: &'a [Color]) -> impl Iterator<Item = &'a [Color]> + 'b {
        let child_iter: Box<dyn Iterator<Item = _>> = if starting.is_empty() {
            Box::new([].into_iter())
        } else if let Some(child) = &self.children[starting[0] as u8 as usize] {
            Box::new(child.stripped(&starting[1..]))
        } else {
            Box::new([].into_iter())
        };
        [starting].into_iter().filter(|_| self.is_leaf).chain(child_iter)
    }

    pub fn iter(&self) -> impl Iterator<Item = Pattern> + '_ {
        fn iter_recursive(trie: &ColorTrie) -> impl Iterator<Item = Pattern> + '_ {
            let child_iter = (0..Color::COLOR_COUNT)
                .flat_map(|i| {
                    let color = Color::from_u8(i as u8);
                    trie.children[i]
                        .iter()
                        .flat_map(|v| iter_recursive(&v))
                        .map(move |mut pat| { pat.0.push(color); pat })
                });
            let child_iter: Box<dyn Iterator<Item = _>> = Box::new(child_iter);
            [Pattern(vec![])].into_iter().filter(|_| trie.is_leaf).chain(child_iter)
        }
        iter_recursive(self).map(|mut v| { v.0.reverse(); v })
    }
}

impl Debug for ColorTrie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Trie");
        if self.is_leaf {
            fmt.field("leaf", &true);
        }

        for i in 0..Color::COLOR_COUNT {
            if let Some(subtrie) = &self.children[i] {
                fmt.field(&Color::from_u8(i as u8).to_char().to_string(), subtrie);
            }
        }

        fmt.finish()
    }
}
