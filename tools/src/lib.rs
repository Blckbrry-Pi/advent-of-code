#[macro_export]
macro_rules! aoc_sol {
    ($day:ident $($input_type:ident)?: $($part_fn:ident),+) => {
        #[allow(dead_code)]
        use std::fmt::Debug;
        #[allow(dead_code)]
        use std::collections::{ HashSet, HashMap };

        #[allow(dead_code)]
        pub fn main() {
            #[allow(dead_code)]
            const TEST: &str = include_str!(concat!("../../data/", stringify!($day), "/test.txt"));
            const INPUT: &str = include_str!(concat!("../../data/", stringify!($day), "/input.txt"));

            let input = $crate::aoc_sol!(@impl input_type $($input_type)?, TEST, INPUT).trim();
            let mut parts = Vec::new();

            let mut i = 1;
            $({
                let part_start = std::time::Instant::now();
                let part = $part_fn(input);
                let part_time = part_start.elapsed();
                println!("Part {i}: {part:?} ({part_time:?})");
                parts.push(format!("{part:?}"));
                i += 1;
            })+

            if std::env::var("VERIFY_OUTPUT").is_ok() {
                $crate::verify(stringify!($day), "2024", parts.into_iter());
            }
        }
    };
    (@impl input_type test, $test:ident, $input:ident) => {
        $test
    };
    (@impl input_type , $test:ident, $input:ident) => {
        $input
    };
}

#[macro_export]
macro_rules! pos {
    ($inner_type:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        struct Pos { x: $inner_type, y: $inner_type }

        #[allow(dead_code)]
        impl Pos {
            pub fn add(&self, o: Self) -> Self {
                Self {
                    x: self.x + o.x,
                    y: self.y + o.y
                }
            }

            pub fn neg(&self) -> Self {
                Self {
                    x: 0 - self.x,
                    y: 0 - self.y
                }
            }

            pub fn sub(&self, o: Self) -> Self {
                Self {
                    x: self.x - o.x,
                    y: self.y - o.y
                }
            }

            pub fn mul(&self, s: $inner_type) -> Self {
                Self {
                    x: self.x * s,
                    y: self.y * s,
                }
            }
        }

    };
}

#[macro_export]
macro_rules! fast_hash {
    () => {
        #[allow(dead_code)]
        pub use $crate::__hidden_hasher::*;
    };
}

pub mod __hidden_hasher {
    use std::collections::{ HashSet, HashMap };

    // type HashBuilder = xxhash_rust::xxh3::Xxh3Builder;
    type HashBuilder = std::hash::RandomState;

    pub type FastMap<K, V> = HashMap<K, V, HashBuilder>;
    pub fn new_fastmap<K, V>() -> FastMap<K, V> {
        HashMap::with_hasher(HashBuilder::new())
    }
    pub fn new_fastmap_with_capacity<K, V>(capacity: usize) -> FastMap<K, V> {
        HashMap::with_capacity_and_hasher(capacity, HashBuilder::new())
    }

    pub type FastSet<T> = HashSet<T, HashBuilder>;
    pub fn new_fastset<T>() -> FastSet<T> {
        HashSet::with_hasher(HashBuilder::new())
    }
    pub fn new_fastset_with_capacity<T>(capacity: usize) -> FastSet<T> {
        HashSet::with_capacity_and_hasher(capacity, HashBuilder::new())
    }
}

pub fn get_cookie() -> String {
    std::fs::read_to_string(".aoc_cookie").unwrap()
}

pub fn get_page(day: &str, year: &str) -> String {
    reqwest::blocking::Client::new()
        .get(format!("https://adventofcode.com/{year}/day/{day}"))
        .header("Cookie", get_cookie())
        .send()
        .unwrap()
        .text()
        .unwrap()
}

pub fn verify(day: &str, year: &str, values: impl Iterator<Item = String>) {
    let day = day.trim_start_matches("day").trim_start_matches('0');
    let page = get_page(day, year);

    for (i, output) in values.enumerate() {
        if page.contains(&output) {
            println!("Part {} verified", i + 1);
        } else {
            println!("Part {} ERROR", i + 1);
        }
    }
}
