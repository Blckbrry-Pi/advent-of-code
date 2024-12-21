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
macro_rules! day_bench {
    ($day:ident) => {
        mod $day {
            use criterion::{ criterion_group, Criterion, SamplingMode::AutoMin };
            use std::hint::black_box;

            const INPUT: &str = include_str!(concat!("../../data/", stringify!($day), "/input.txt"));

            fn p1(c: &mut criterion::Criterion) {
                let number = stringify!($day).trim_start_matches("day");
                let name = format!("Day {number} Part 1");
                c.bench_function(&name, |b| b.iter(|| ::$day::part1(black_box(INPUT.trim()))));
            }

            fn p2(c: &mut criterion::Criterion) {
                let number = stringify!($day).trim_start_matches("day");
                let name = format!("Day {number} Part 2");
                c.bench_function(&name, |b| b.iter(|| ::$day::part2(black_box(INPUT.trim()))));
            }

            criterion_group! {
                name = $day;
                config = Criterion::default()
                    .measurement_time(std::time::Duration::from_secs(10))
                    .sampling_mode(AutoMin(10));
                targets = p1, p2
            }
        }
    };
}

#[macro_export]
macro_rules! multi_day_bench {
    ($multiday_name:ident: $($day:ident),+ $(,)?) => {
        mod $multiday_name {
            use criterion::{ criterion_group, Criterion };
            use std::hint::black_box;

            const INPUT_COUNT: u64 = [$(stringify!($day),)+].len() as u64;
            #[allow(non_upper_case_globals)]
            mod inputs {
                $(
                    pub const $day: &str = include_str!(concat!("../../data/", stringify!($day), "/input.txt"));
                )+
            }

            fn multiday_fn(c: &mut criterion::Criterion) {
                let name = concat!("Multiday ", stringify!($multiday_name));
                c.bench_function(name, |b| b.iter(|| {
                    $(
                        ::$day::part1(black_box(inputs::$day.trim()));
                        ::$day::part2(black_box(inputs::$day.trim()));
                    )+
                }));
            }

            criterion_group! {
                name = $multiday_name;
                config = Criterion::default().measurement_time(std::time::Duration::from_secs(INPUT_COUNT * 10));
                targets = multiday_fn
            }
        }
    };
}

#[macro_export]
macro_rules! pos {
    ($inner_type:ty $(: $($derives:ident),+)?) => {
        // #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $crate::pos!(@impl {
            struct Pos { x: $inner_type, y: $inner_type }
    
            #[allow(dead_code)]
            impl Pos {
                pub fn add(&self, o: Self) -> Self {
                    Self {
                        x: self.x.wrapping_add(o.x),
                        y: self.y.wrapping_add(o.y)
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
                        x: self.x.wrapping_sub(o.x),
                        y: self.y.wrapping_sub(o.y),
                    }
                }
    
                pub fn mul(&self, s: $inner_type) -> Self {
                    Self {
                        x: self.x * s,
                        y: self.y * s,
                    }
                }
            }
        } derives: $($($derives)+)?);
    };

    (@impl { $($input:tt)+ } derives: $($derive:ident)+) => {
        #[derive($($derive),+)]
        $($input)+
    };
    (@impl { $($input:tt)+ } derives: ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $($input)+
    };
}

#[macro_export]
macro_rules! arena {
    ($($feature:ident)?) => {
        use $crate::__hidden_ferroc::Ferroc;

        $(#[feature($feature)])?
        #[global_allocator]
        static FERROC: Ferroc = Ferroc;
    };
}

#[macro_export]
macro_rules! fast_hash {
    () => {
        #[allow(dead_code)]
        pub use $crate::__hidden_hasher::*;
    };
}

#[doc(hidden)]
#[allow(unused_imports)]
pub mod __hidden_hasher {
    use std::collections::{ HashSet, HashMap };
    use std::hash::{ BuildHasher, DefaultHasher, Hash, Hasher };
    use std::num::Wrapping;

    // type HashBuilder = FastHasherBuilder;
    // type HashBuilder = xxhash_rust::xxh3::Xxh3Builder;
    // type HashBuilder = std::hash::RandomState;
    type HashBuilder = NoRandomState;

    // #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    // pub struct FastHasher(Wrapping<u64>);

    // macro_rules! impl_int {
    //     ($fn:ident($type:ty) << $shift:literal) => {
    //         fn $fn(&mut self, i: $type) {
    //             self.0 = (self.0 << $shift) ^ self.0 ^ Wrapping(i as u64);
    //         }
    //     };
    // }

    // impl Hasher for FastHasher {
    //     fn write(&mut self, bytes: &[u8]) {
    //         for &byte in bytes {
    //             self.0 = (self.0 << 5) ^ self.0 ^ Wrapping(byte as u64);
    //         }
    //     }

    //     impl_int!(write_i8(i8) << 5);
    //     impl_int!(write_u8(u8) << 5);

    //     impl_int!(write_i16(i16) << 11);
    //     impl_int!(write_u16(u16) << 11);

    //     impl_int!(write_i32(i32) << 23);
    //     impl_int!(write_u32(u32) << 23);

    //     impl_int!(write_i64(i64) << 41);
    //     impl_int!(write_u64(u64) << 41);

    //     fn finish(&self) -> u64 {
    //         // Modulo first prime after 2^62
    //         // self.0.0 % 4_611_686_018_427_388_039
    //         self.0.0
    //     }
    // }

    // #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    // pub struct FastHasherBuilder(Wrapping<u64>);
    // impl BuildHasher for FastHasherBuilder {
    //     type Hasher = FastHasher;
    //     fn build_hasher(&self) -> Self::Hasher {
    //         FastHasher(self.0)
    //     }
    // }
    // impl Default for FastHasherBuilder {
    //     fn default() -> Self {
    //         // Prime number with nice digits spread evenly
    //         // First prime before 2^63.5
    //         Self(Wrapping(13_043_817_825_332_783_101))
    //     }
    // }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct NoRandomState;
    impl BuildHasher for NoRandomState {
        type Hasher = std::hash::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }
    

    pub type FastMap<K, V> = HashMap<K, V, HashBuilder>;
    pub fn new_fastmap<K, V>() -> FastMap<K, V> {
        HashMap::with_hasher(HashBuilder::default())
    }
    pub fn new_fastmap_with_capacity<K, V>(capacity: usize) -> FastMap<K, V> {
        HashMap::with_capacity_and_hasher(capacity, HashBuilder::default())
    }

    pub type FastSet<T> = HashSet<T, HashBuilder>;
    pub fn new_fastset<T>() -> FastSet<T> {
        HashSet::with_hasher(HashBuilder::default())
    }
    pub fn new_fastset_with_capacity<T>(capacity: usize) -> FastSet<T> {
        HashSet::with_capacity_and_hasher(capacity, HashBuilder::default())
    }
}

#[doc(hidden)]
pub use ferroc as __hidden_ferroc;

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
