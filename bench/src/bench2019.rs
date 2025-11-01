use criterion::criterion_main;

// aoc_tools::arena!(arena);

aoc_tools::multi_day_bench!(
    bench_2019 2019:
        // day01 (day01_2019), day02 (day02_2019), day03 (day03_2019), day04 (day04_2019), day05 (day05_2019),
        // day06 (day06_2019), day07 (day07_2019), day08 (day08_2019), day09 (day09_2019), day10 (day10_2019),
        day11 (day11_2019), day12 (day12_2019), day13 (day13_2019), day14 (day14_2019), day15 (day15_2019),
        // day16 (day16_2019), day17 (day17_2019), day18 (day18_2019), day19 (day19_2019), day20 (day20_2019),
        // day21 (day21_2019), day22 (day22_2019), day23 (day23_2019), day24 (day24_2019), day25 (day25_2019),
);

criterion_main!(bench_2019::bench_2019);
