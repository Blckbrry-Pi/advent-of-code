use criterion::criterion_main;

aoc_tools::arena!(arena);

aoc_tools::day_bench!(day01);
aoc_tools::day_bench!(day02);
aoc_tools::day_bench!(day03);
aoc_tools::day_bench!(day04);
aoc_tools::day_bench!(day05);
aoc_tools::day_bench!(day06);
aoc_tools::day_bench!(day07);
aoc_tools::day_bench!(day08);
aoc_tools::day_bench!(day09);
aoc_tools::day_bench!(day10);
aoc_tools::day_bench!(day11);
aoc_tools::day_bench!(day12);
aoc_tools::day_bench!(day13);
aoc_tools::day_bench!(day14);
aoc_tools::day_bench!(day15);
aoc_tools::day_bench!(day16);
aoc_tools::day_bench!(day17);
aoc_tools::day_bench!(day18);
aoc_tools::day_bench!(day19);
aoc_tools::day_bench!(day20);
aoc_tools::day_bench!(day21);
aoc_tools::day_bench!(day22);
aoc_tools::day_bench!(day23);
aoc_tools::day_bench!(day24);
aoc_tools::day_bench!(day25);

// aoc_tools::day_bench!(day01 2022: day01_2022);
// aoc_tools::day_bench!(day02 2022: day02_2022);
// aoc_tools::day_bench!(day16 2022: day16_2022);
aoc_tools::day_bench!(day17 2022: day17_2022);
// aoc_tools::day_bench!(day18 2022: day18_2022);
// aoc_tools::day_bench!(day19 2022: day19_2022);
aoc_tools::day_bench!(day20 2022: day20_2022);
aoc_tools::day_bench!(day21 2022: day21_2022);
aoc_tools::day_bench!(day23 2022: day23_2022);
aoc_tools::day_bench!(day24 2022: day24_2022);

criterion_main!(
    day01::day01,
    day02::day02,
    day03::day03,
    day04::day04,
    day05::day05,
    day06::day06,
    day07::day07,
    day08::day08,
    day09::day09,
    day10::day10,
    day11::day11,
    day12::day12,
    day13::day13,
    day14::day14,
    day15::day15,
    day16::day16,
    day17::day17,
    day18::day18,
    day19::day19,
    day20::day20,
    day21::day21,
    day22::day22,
    day23::day23,
    day24::day24,
    day25::day25,

    // day01_2022::day01_2022,
    // day02_2022::day02_2022,
    // day16_2022::day16_2022,
    day17_2022::day17_2022,
    // day18_2022::day18_2022,
    // day19_2022::day19_2022,
    day20_2022::day20_2022,
    day21_2022::day21_2022,
    day23_2022::day23_2022,
    day24_2022::day24_2022,
);
