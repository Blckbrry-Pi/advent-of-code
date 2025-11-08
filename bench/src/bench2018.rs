use criterion::criterion_main;

// aoc_tools::arena!(arena);

aoc_tools::multi_day_bench!(
    bench_2018 2018:
        day01 (day01_2018), day02 (day02_2018), day03 (day03_2018), day04 (day04_2018), day05 (day05_2018),
        day06 (day06_2018), day07 (day07_2018), day08 (day08_2018), day09 (day09_2018), day10 (day10_2018),
        day11 (day11_2018), day12 (day12_2018), day13 (day13_2018), day14 (day14_2018), day15 (day15_2018),
        /* day16 (day16_2018), day17 (day17_2018), day18 (day18_2018), day19 (day19_2018), day20 (day20_2018),
        day21 (day21_2018), day22 (day22_2018), day23 (day23_2018), day24 (day24_2018), day25 (day25_2018), */
);

criterion_main!(bench_2018::bench_2018);
