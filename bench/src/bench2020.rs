use criterion::criterion_main;

// aoc_tools::arena!(arena);

aoc_tools::multi_day_bench!(
    bench_2020 2020:
        day01 (day01_2020), day02 (day02_2020), day03 (day03_2020), day04 (day04_2020), day05 (day05_2020),
        day06 (day06_2020), day07 (day07_2020), day08 (day08_2020), day09 (day09_2020), day10 (day10_2020),
        day11 (day11_2020), day12 (day12_2020), day13 (day13_2020), day14 (day14_2020), day15 (day15_2020),
        day16 (day16_2020), day17 (day17_2020), day18 (day18_2020), day19 (day19_2020), day20 (day20_2020),
        day21 (day21_2020), day22 (day22_2020), day23 (day23_2020), day24 (day24_2020), day25 (day25_2020),
);

criterion_main!(bench_2020::bench_2020);
