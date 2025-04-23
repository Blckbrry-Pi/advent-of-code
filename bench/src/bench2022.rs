use criterion::criterion_main;

// aoc_tools::arena!(arena);

aoc_tools::multi_day_bench!(
    bench_2022 2022:
        day01 (day01_2022), day02 (day02_2022),


        day16 (day16_2022), day17 (day17_2022), day18 (day18_2022), day19 (day19_2022), day20 (day20_2022),
        day21 (day21_2022), day22 (day22_2022), day23 (day23_2022), day24 (day24_2022), day25 (day25_2022),
);

criterion_main!(bench_2022::bench_2022);
