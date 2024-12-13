use criterion::criterion_main;

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
);
