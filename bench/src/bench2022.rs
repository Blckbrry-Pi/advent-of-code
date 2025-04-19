use criterion::criterion_main;

// aoc_tools::arena!(arena);

aoc_tools::multi_day_bench!(
    bench_2022:
        day01_2022, day02_2022,


        day16_2022,             day18_2022, day19_2022, day20_2022,
        day21_2022,
);

criterion_main!(bench_2022::bench_2022);
