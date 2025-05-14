#[macro_export]
macro_rules! aoc_sol {
    ($day:ident $($year:literal)? $($input_type:ident)?: $($part_fn:ident),+) => {
        #[allow(dead_code)]
        use std::fmt::Debug;
        #[allow(dead_code)]
        use std::collections::{ HashSet, HashMap };

        #[allow(dead_code)]
        pub fn main() {
            #[allow(dead_code)]
            const TEST: &str = $crate::input_file!(relative $day $(($year))? -> "test.txt");
            const INPUT: &str = $crate::input_file!(relative $day $(($year))? -> "input.txt");

            let input = $crate::aoc_sol!(@impl input_type $($input_type)?, TEST, INPUT);
            let mut parts = Vec::new();

            let mut i = 1;
            $({
                let part_start = std::time::Instant::now();
                let part = $part_fn(input);
                let part_time = part_start.elapsed();
                println!("Part {i}: {part} ({part_time:?})");
                parts.push(part.to_string());
                i += 1;
            })+

            if std::env::var("VERIFY_OUTPUT").is_ok() {
                let year = [$($year,)? 2024][0].to_string();
                $crate::verify(stringify!($day), &year, parts.into_iter());
            }
        }

        #[test]
        fn verify_outputs() {
            #[allow(dead_code)]
            const TEST: &str = $crate::input_file!(relative $day $(($year))? -> "test.txt");
            const INPUT: &str = $crate::input_file!(relative $day $(($year))? -> "input.txt");

            let input = $crate::aoc_sol!(@impl input_type $($input_type)?, TEST, INPUT);
            let mut parts = Vec::new();

            let mut i = 1;
            $({
                let part_start = std::time::Instant::now();
                let part = $part_fn(input);
                let part_time = part_start.elapsed();
                parts.push(part.to_string());
                i += 1;
            })+

            let year = [$($year,)? 2024][0].to_string();
            $crate::verify(stringify!($day), &year, parts.into_iter());
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
    ($day:ident) => { $crate::day_bench! { @impl $day, $day, } };
    ($day:ident $year:literal: $module:ident) => { $crate::day_bench! { @impl $module, $day, $year } };
    (@impl $module:ident, $day:ident, $($year:literal)?) => {
        mod $module {
            use criterion::{ criterion_group, Criterion, SamplingMode::AutoMin };
            use std::hint::black_box;

            const INPUT: &str = $crate::input_file!(bench $day $(($year))? -> "input.txt");

            fn p1(c: &mut criterion::Criterion) {
                let number = stringify!($day).trim_start_matches("day");
                #[allow(dead_code)]
                let name = format!("Day {number} Part 1");
                $(
                    let name = format!("{} day {number} Part 1", $year);
                )?
                c.bench_function(&name, |b| b.iter(|| ::$module::part1(black_box(INPUT))));
            }

            fn p2(c: &mut criterion::Criterion) {
                let number = stringify!($day).trim_start_matches("day");
                let name = format!("Day {number} Part 2");
                $(
                    let name = format!("{} day {number} Part 2", $year);
                )?
                c.bench_function(&name, |b| b.iter(|| ::$module::part2(black_box(INPUT))));
            }

            criterion_group! {
                name = $module;
                config = Criterion::default()
                    .measurement_time(std::time::Duration::from_secs(10))
                    .sampling_mode(AutoMin(10));
                targets = p1, p2
            }
        }
    };
}

#[macro_export]
macro_rules! input_file {
    (bench $day:ident ($year:literal) -> $file:literal) => {
        include_str!(concat!("../../data/", $year, "/", stringify!($day), "/", $file))
    };
    (relative $day:ident ($year:literal) -> $file:literal) => {
        include_str!(concat!("../../../data/", $year, "/", stringify!($day), "/", $file))
    };
    (bench $day:ident -> $file:literal) => {
        include_str!(concat!("../../data/", stringify!($day), "/", $file))
    };
    (relative $day:ident -> $file:literal) => {
        include_str!(concat!("../../data/", stringify!($day), "/", $file))
    };
}

#[macro_export]
macro_rules! multi_day_bench {
    ($multiday_name:ident: $($day:ident),+ $(,)?) => {
        $crate::multi_day_bench! { $multiday_name: $($day ($day)),+ }
    };
    ($multiday_name:ident $($year:literal)?: $($day:ident ($module:ident)),+ $(,)?) => {
        mod $multiday_name {
            use criterion::{ criterion_group, Criterion };
            use std::hint::black_box;

            const INPUT_COUNT: u64 = [$(stringify!($day),)+].len() as u64;
            #[allow(non_upper_case_globals)]
            mod inputs {
                $crate::multi_day_bench! { @impl $($year)?; $($day),+ }
            }

            fn multiday_fn(c: &mut criterion::Criterion) {
                let name = concat!("Multiday ", stringify!($multiday_name));
                c.bench_function(name, |b| b.iter(|| {
                    $(
                        ::$module::part1(black_box(inputs::$day));
                        ::$module::part2(black_box(inputs::$day));
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
    (@impl $($year:literal)?;) => {};
    (@impl $($year:literal)?; $day:ident$(,)? $($rest:ident),*) => {
        pub const $day: &str = include_str!(concat!("../../data/", $($year, "/",)? stringify!($day), "/input.txt"));
        $crate::multi_day_bench! { @impl $($year)?; $($rest),* }
    };
}

#[macro_export]
macro_rules! pos3 {
    ($inner_type:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct Pos3 { x: $inner_type, y: $inner_type, z: $inner_type }
        #[allow(dead_code)]
        impl Pos3 {
            pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };

            pub const fn add(&self, o: Self) -> Self {
                Self {
                    x: self.x.wrapping_add(o.x),
                    y: self.y.wrapping_add(o.y),
                    z: self.z.wrapping_add(o.z),
                }
            }

            pub const fn neg(&self) -> Self {
                Self {
                    x: (0u8 as $inner_type).wrapping_sub(self.x),
                    y: (0u8 as $inner_type).wrapping_sub(self.y),
                    z: (0u8 as $inner_type).wrapping_sub(self.z),
                }
            }

            pub const fn sub(&self, o: Self) -> Self {
                Self {
                    x: self.x.wrapping_sub(o.x),
                    y: self.y.wrapping_sub(o.y),
                    z: self.z.wrapping_sub(o.z),
                }
            }

            pub const fn mul(&self, s: $inner_type) -> Self {
                Self {
                    x: self.x * s,
                    y: self.y * s,
                    z: self.z * s,
                }
            }

            pub const fn abs(&self) -> Self {
                Self {
                    x: if self.x < 0 { (0_u8 as $inner_type).wrapping_sub(self.x) } else { self.x },
                    y: if self.y < 0 { (0_u8 as $inner_type).wrapping_sub(self.y) } else { self.y },
                    z: if self.z < 0 { (0_u8 as $inner_type).wrapping_sub(self.z) } else { self.z },
                }
            }

            pub fn manhattan(&self, o: Self) -> $inner_type {
                let x_diff = if self.x > o.x {
                    self.x - o.x
                } else {
                    o.x - self.x
                };
                let y_diff = if self.y > o.y {
                    self.y - o.y
                } else {
                    o.y - self.y
                };
                let z_diff = if self.z > o.z {
                    self.z - o.z
                } else {
                    o.z - self.z
                };
                x_diff + y_diff + z_diff
            }
        }
    };
}

#[macro_export]
macro_rules! pos {
    ($inner_type:ty $(: $($derives:ident),+)? $(; +y => $dir:ident)?) => {
        // #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $crate::pos!(@impl {
            struct Pos { x: $inner_type, y: $inner_type }
    
            #[allow(dead_code)]
            impl Pos {
                $(
                    pub const N: Self = Self {
                        x: 0,
                        y: $crate::pos!(@impl +y => $dir; N ($inner_type)),
                    };
                    pub const U: Self = Self::N;

                    pub const S: Self = Self {
                        x: 0,
                        y: $crate::pos!(@impl +y => $dir; S ($inner_type)),
                    };
                    pub const D: Self = Self::S;

                    pub const W: Self = Self {
                        x: (0_u8 as $inner_type).wrapping_sub(1),
                        y: 0,
                    };
                    pub const L: Self = Self::W;

                    pub const E: Self = Self {
                        x: 1,
                        y: 0,
                    };
                    pub const R: Self = Self::E;
                )?
                pub fn add(&self, o: Self) -> Self {
                    Self {
                        x: self.x.wrapping_add(o.x),
                        y: self.y.wrapping_add(o.y)
                    }
                }
    
                pub fn neg(&self) -> Self {
                    Self {
                        x: (0u8 as $inner_type).wrapping_sub(self.x),
                        y: (0u8 as $inner_type).wrapping_sub(self.y)
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

                pub fn abs(&self) -> Self {
                    Self {
                        x: if self.x < 0 { (0_u8 as $inner_type).wrapping_sub(self.x) } else { self.x },
                        y: if self.y < 0 { (0_u8 as $inner_type).wrapping_sub(self.y) } else { self.y },
                    }
                }

                pub fn swap(&self) -> Self {
                    Self {
                        x: self.y,
                        y: self.x,
                    }
                }

                $(
                    pub fn turn_r(&self) -> Self {
                        const NEW_X_MULT: $inner_type = $crate::pos!(@impl +y => $dir; N ($inner_type));
                        const NEW_Y_MULT: $inner_type = $crate::pos!(@impl +y => $dir; S ($inner_type));

                        Self {
                            x: self.y * NEW_X_MULT,
                            y: self.x * NEW_Y_MULT,
                        }
                    }
                    pub fn turn_l(&self) -> Self {
                        const NEW_X_MULT: $inner_type = $crate::pos!(@impl +y => $dir; S ($inner_type));
                        const NEW_Y_MULT: $inner_type = $crate::pos!(@impl +y => $dir; N ($inner_type));

                        Self {
                            x: self.y * NEW_X_MULT,
                            y: self.x * NEW_Y_MULT,
                        }
                    }
                )?

                pub fn manhattan(&self, o: Self) -> $inner_type {
                    let x_diff = if self.x > o.x {
                        self.x - o.x
                    } else {
                        o.x - self.x
                    };
                    let y_diff = if self.y > o.y {
                        self.y - o.y
                    } else {
                        o.y - self.y
                    };
                    x_diff + y_diff
                }
            }
        } derives: $($($derives)+)?);
    };

    (@impl +y=>$(U)?$(UP)?$(up)?; N ($inner_type:ty)) => { 1 };
    (@impl +y=>$(D)?$(DOWN)?$(down)?; N ($inner_type:ty)) => { (0_u8 as $inner_type).wrapping_sub(1) };

    (@impl +y=>$(U)?$(UP)?$(up)?; S ($inner_type:ty)) => { (0_u8 as $inner_type).wrapping_sub(1) };
    (@impl +y=>$(D)?$(DOWN)?$(down)?; S ($inner_type:ty)) => { 1 };

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
#[cfg(feature = "arena")]
macro_rules! arena {
    ($($feature:ident)?) => {
        use $crate::__hidden_ferroc::Ferroc;

        $(#[feature($feature)])?
        #[global_allocator]
        static FERROC: Ferroc = Ferroc;
    };
}
#[macro_export]
#[cfg(not(feature = "arena"))]
macro_rules! arena {
    ($($feature:ident)?) => {}
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
    pub type HashBuilder = NoRandomState;

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

use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
#[cfg(feature = "arena")]
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

#[macro_export]
macro_rules! parse_unsigned {
    ($fn_name:ident<$type:ty> (=$digits:literal digits)) => {
        pub fn $fn_name(s: impl AsRef<[u8]>) -> $type {
            let mut curr: $type = 0;
            for i in 0..$digits {
                curr *= 10;
                curr += (s.as_ref()[i] - b'0') as $type;
            }
            curr
        }
    };
    ($fn_name:ident<$type:ty> (<= $digits:literal digits)) => {
        pub fn $fn_name(s: impl AsRef<[u8]>) -> $type {
            let mut curr: $type = 0;
            let mut i = 0;
            let max = s.as_ref().len().min($digits);
            for i in 0..max {
                curr *= 10;
                curr += (unsafe { *s.as_ref().get(i).unwrap() } - b'0') as $type;
            }
            curr
        }
    };
}

pub enum SmallVec<const N: usize, T> {
    Stack([MaybeUninit<T>; N], usize),
    Heap(Vec<T>),
}
impl<const N: usize, T> SmallVec<N, T> {
    pub fn new() -> Self {
        Self::Stack([const { MaybeUninit::uninit() }; N], 0)
    }
    pub fn push(&mut self, t: T) {
        match self {
            Self::Stack(data, len) => {
                if *len < N {
                    data[*len] = MaybeUninit::new(t);
                    *len += 1;
                } else {
                    *len = 0;
                    *self = Self::Heap(
                        data.iter_mut()
                            .map(|t| (t, MaybeUninit::uninit()))
                            .map(|(t, replacement)| std::mem::replace(t, replacement))
                            .map(|t| unsafe { t.assume_init() })
                            .collect(),
                    );
                    self.push(t);
                }
            },
            Self::Heap(data) => {
                data.push(t);
            }
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::Stack(_, 0) => None,
            Self::Stack(data, len) => {
                *len -= 1;
                let output = std::mem::replace(&mut data[*len], MaybeUninit::uninit());
                unsafe { Some(output.assume_init()) }
            },
            Self::Heap(data) => {
                let output = data.pop();
                if data.len() <= N {
                    let len = data.len();
                    let mut new_data = [const { MaybeUninit::uninit() }; N];
                    data.drain(..)
                        .enumerate()
                        .for_each(|(i, t)| new_data[i] = MaybeUninit::new(t));
                    *self = Self::Stack(new_data, len);
                }
                output
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Stack(_, len) => *len,
            Self::Heap(data) => data.len(),
        }
    }

    pub fn iter(&self) -> SmallVecIter<N, T> {
        match self {
            Self::Heap(data) => SmallVecIter::Heap(data.iter()),
            Self::Stack(data, len) => SmallVecIter::Stack(data, *len, 0),
        }
    }
}
impl<const N: usize, T> IntoIterator for SmallVec<N, T> {
    type Item = T;
    type IntoIter = SmallVecIntoIter<N, T>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Heap(data) => SmallVecIntoIter::Heap(data.into_iter()),
            Self::Stack(data, len) => SmallVecIntoIter::Stack(data, len, 0),
        }
    }
}
impl<const N: usize, T> FromIterator<T> for SmallVec<N, T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut output = Self::new();
        for value in iter {
            output.push(value);
        }
        output
    }
}
impl<const N: usize, T: Debug> Debug for SmallVec<N, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        for i in 0..self.len() {
            list.entry(&self[i]);
        }
        list.finish()
    }
}
impl<const N: usize, T: Clone> Clone for SmallVec<N, T> {
    fn clone(&self) -> Self {
        match self {
            Self::Heap(data) => Self::Heap(data.clone()),
            Self::Stack(data, len) => {
                let mut cloned_data = [const { MaybeUninit::uninit() }; N];
                for i in 0..*len {
                    cloned_data[i] = unsafe { MaybeUninit::new(data[i].assume_init_ref().clone()) };
                }
                Self::Stack(cloned_data, *len)
            }
        }
    }
}
impl<const N: usize, T: PartialEq> PartialEq for SmallVec<N, T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Heap(data_a), Self::Heap(data_b)) => {
                data_a.eq(data_b)
            },
            (Self::Stack(data_a, len_a), Self::Stack(data_b, len_b)) => {
                if len_a != len_b { return false; }
                for i in 0..*len_a {
                    if unsafe { data_a[i].assume_init_ref() != data_b[i].assume_init_ref() } {
                        return false;
                    }
                }
                true
            }
            _ => false,
        }
    }
}
impl<const N: usize, T: Eq> Eq for SmallVec<N, T> {}

impl<const N: usize, T: Hash> Hash for SmallVec<N, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Stack(data, len) => {
                state.write_usize(*len);
                for i in 0..*len {
                    unsafe { data[i].assume_init_ref().hash(state); }
                }
            },
            Self::Heap(data) => {
                data.hash(state);
            }
        }
    }
}

impl<const N: usize, T> Default for SmallVec<N, T> {
    fn default() -> Self { Self::new() }
}

impl<const N: usize, T> Deref for SmallVec<N, T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        match self {
            Self::Heap(data) => data.as_slice(),
            Self::Stack(data, len) => {
                unsafe { &*(&data[0..*len] as *const [MaybeUninit<T>] as *const [T]) }
            }
        }
    }
}
impl<const N: usize, T> DerefMut for SmallVec<N, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        match self {
            Self::Heap(data) => data.as_mut_slice(),
            Self::Stack(data, len) => {
                unsafe { &mut *(&mut data[0..*len] as *mut [MaybeUninit<T>] as *mut [T]) }
            }
        }
    }
}

pub enum SmallVecIntoIter<const N: usize, T> {
    Stack([MaybeUninit<T>; N], usize, usize),
    Heap(std::vec::IntoIter<T>),
}
impl<const N: usize, T> Iterator for SmallVecIntoIter<N, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Stack(data, len, idx) => if idx >= len {
                None
            } else {
                *idx += 1;
                let output = std::mem::replace(&mut data[*idx], MaybeUninit::uninit());
                unsafe { Some(output.assume_init()) }
            },
            Self::Heap(data) => data.next(),
        }
    }
}

pub enum SmallVecIter<'a, const N: usize, T> {
    Stack(&'a [MaybeUninit<T>; N], usize, usize),
    Heap(std::slice::Iter<'a, T>),
}
impl<'a, const N: usize, T> Iterator for SmallVecIter<'a, N, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Stack(data, len, idx) => if idx >= len {
                None
            } else {
                let output = unsafe { data[*idx].assume_init_ref() };
                *idx += 1;
                Some(output)
            },
            Self::Heap(data) => data.next(),
        }
    }
}

impl<const N: usize, T> std::ops::Index<usize> for SmallVec<N, T> {
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            Self::Stack(data, len) => if idx >= *len {
                panic!("Index out of bounds");
            } else {
                unsafe { data[idx].assume_init_ref() }
            },
            Self::Heap(data) => &data[idx],
        }
    }
}

impl<const N: usize, T> std::ops::IndexMut<usize> for SmallVec<N, T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match self {
            Self::Stack(data, len) => if idx >= *len {
                panic!("Index out of bounds");
            } else {
                unsafe { data[idx].assume_init_mut() }
            },
            Self::Heap(data) => data.iter_mut().nth(idx).unwrap(),
        }
    }
}
