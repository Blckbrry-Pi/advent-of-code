use criterion::criterion_main;

aoc_tools::arena!(arena);

aoc_tools::day_bench!(day01 2024: day01_2024);
aoc_tools::day_bench!(day02 2024: day02_2024);
aoc_tools::day_bench!(day03 2024: day03_2024);
aoc_tools::day_bench!(day04 2024: day04_2024);
aoc_tools::day_bench!(day05 2024: day05_2024);
aoc_tools::day_bench!(day06 2024: day06_2024);
aoc_tools::day_bench!(day07 2024: day07_2024);
aoc_tools::day_bench!(day08 2024: day08_2024);
aoc_tools::day_bench!(day09 2024: day09_2024);
aoc_tools::day_bench!(day10 2024: day10_2024);
aoc_tools::day_bench!(day11 2024: day11_2024);
aoc_tools::day_bench!(day12 2024: day12_2024);
aoc_tools::day_bench!(day13 2024: day13_2024);
aoc_tools::day_bench!(day14 2024: day14_2024);
aoc_tools::day_bench!(day15 2024: day15_2024);
aoc_tools::day_bench!(day16 2024: day16_2024);
aoc_tools::day_bench!(day17 2024: day17_2024);
aoc_tools::day_bench!(day18 2024: day18_2024);
aoc_tools::day_bench!(day19 2024: day19_2024);
aoc_tools::day_bench!(day20 2024: day20_2024);
aoc_tools::day_bench!(day21 2024: day21_2024);
aoc_tools::day_bench!(day22 2024: day22_2024);
aoc_tools::day_bench!(day23 2024: day23_2024);
aoc_tools::day_bench!(day24 2024: day24_2024);
aoc_tools::day_bench!(day25 2024: day25_2024);

aoc_tools::day_bench!(day01 2022: day01_2022);
aoc_tools::day_bench!(day02 2022: day02_2022);
aoc_tools::day_bench!(day16 2022: day16_2022);
aoc_tools::day_bench!(day17 2022: day17_2022);
aoc_tools::day_bench!(day18 2022: day18_2022);
aoc_tools::day_bench!(day19 2022: day19_2022);
aoc_tools::day_bench!(day20 2022: day20_2022);
aoc_tools::day_bench!(day21 2022: day21_2022);
aoc_tools::day_bench!(day22 2022: day22_2022);
aoc_tools::day_bench!(day23 2022: day23_2022);
aoc_tools::day_bench!(day24 2022: day24_2022);
aoc_tools::day_bench!(day25 2022: day25_2022);

criterion_main!(
    // 2024
    day01_2024::day01_2024,
    day02_2024::day02_2024,
    day03_2024::day03_2024,
    day04_2024::day04_2024,
    day05_2024::day05_2024,
    day06_2024::day06_2024,
    day07_2024::day07_2024,
    day08_2024::day08_2024,
    day09_2024::day09_2024,
    day10_2024::day10_2024,
    day11_2024::day11_2024,
    day12_2024::day12_2024,
    day13_2024::day13_2024,
    day14_2024::day14_2024,
    day15_2024::day15_2024,
    day16_2024::day16_2024,
    day17_2024::day17_2024,
    day18_2024::day18_2024,
    day19_2024::day19_2024,
    day20_2024::day20_2024,
    day21_2024::day21_2024,
    day22_2024::day22_2024,
    day23_2024::day23_2024,
    day24_2024::day24_2024,
    day25_2024::day25_2024,

    // 2022
    day01_2022::day01_2022,
    day02_2022::day02_2022,
    day16_2022::day16_2022,
    day17_2022::day17_2022,
    day18_2022::day18_2022,
    day19_2022::day19_2022,
    day20_2022::day20_2022,
    day21_2022::day21_2022,
    day22_2022::day22_2022,
    day23_2022::day23_2022,
    day24_2022::day24_2022,
    day25_2022::day25_2022,
);
