use criterion::criterion_main;

aoc_tools::multi_day_bench!(
    bench_2024:
        day01, day02, day03, day04, day05,
        day06, day07, day08, day09, day10,
        day11, day12, day13,
);

criterion_main!(bench_2024::bench_2024);