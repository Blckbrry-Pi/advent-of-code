use criterion::criterion_main;

// aoc_tools::arena!(arena);

aoc_tools::multi_day_bench!(
    bench_2024 2024:
        day01 (day01_2024), day02 (day02_2024), day03 (day03_2024), day04 (day04_2024), day05 (day05_2024),
        day06 (day06_2024), day07 (day07_2024), day08 (day08_2024), day09 (day09_2024), day10 (day10_2024),
        day11 (day11_2024), day12 (day12_2024), day13 (day13_2024), day14 (day14_2024), day15 (day15_2024),
        day16 (day16_2024), day17 (day17_2024), day18 (day18_2024), day19 (day19_2024), day20 (day20_2024),
        day21 (day21_2024), day22 (day22_2024), day23 (day23_2024), day24 (day24_2024), day25 (day25_2024),
);

criterion_main!(bench_2024::bench_2024);
