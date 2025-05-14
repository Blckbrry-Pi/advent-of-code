use criterion::criterion_main;

// aoc_tools::arena!(arena);

aoc_tools::multi_day_bench!(
    bench_2021 2021:
                                                day08 (day18_2021), day09 (day19_2021), day10 (day20_2021),
        day11 (day16_2021), day12 (day17_2021), day13 (day18_2021), day14 (day19_2021), day15 (day20_2021),
        day16 (day16_2021), day17 (day17_2021), day18 (day18_2021), day19 (day19_2021), day20 (day20_2021),
        day21 (day21_2021), day22 (day22_2021), day23 (day23_2021), day24 (day24_2021), day25 (day25_2021),
);

criterion_main!(bench_2021::bench_2021);
