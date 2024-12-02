fn main() {
    part1();
    part2();
}

const TEST: &str = include_str!("../../data/day2/test.txt");
const PART_1: &str = include_str!("../../data/day2/p1.txt");
const PART_2: &str = include_str!("../../data/day2/p2.txt");

// fn part1() {
//     let (mut left, mut right) = parse_input(PART_1);

//     left.sort();
//     right.sort();

//     let sum: i32 = zipped(left, right).map(diffed).sum();

//     println!("Part 1: {}", sum);
// }

fn part1() {
    let rows = parse_input(PART_1);
    
    let mut safe_count = 0;
    for row in rows {
        let mut increasing = true;
        let mut decreasing = true;
        let mut diffs_valid = true;
        for i in 0..row.len() - 1 {
            if row[i] > row[i + 1] {
                increasing = false;
            }
            if row[i] < row[i + 1] {
                decreasing = false;
            }
            if !(1..=3).contains(&(row[i] - row[i + 1]).abs()) {
                diffs_valid = false;
            }
        }

        if (increasing || decreasing) && diffs_valid {
            safe_count += 1;
        }
    }

    println!("Part 1: {}", safe_count);
}

fn part2() {
    let rows = parse_input(PART_2);
    
    let mut safe_count = 0;
    for row in rows {
        for skip in 0..row.len() {
            let mut increasing = true;
            let mut decreasing = true;
            let mut diffs_valid = true;
            for i in 0..row.len() - 2 {
                let this = if i < skip { row[i] } else { row[i + 1] };
                let next = if i + 1 < skip { row[i + 1] } else { row[i + 2] };
                if this > next {
                    increasing = false;
                }
                if this < next {
                    decreasing = false;
                }
                if !(1..=3).contains(&(this - next).abs()) {
                    diffs_valid = false;
                }
            }
    
            if (increasing || decreasing) && diffs_valid {
                safe_count += 1;
                break;
            }
        }
    }

    println!("Part 2: {}", safe_count);
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.split('\n')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::split_whitespace)
        .map(|r| r.map(str::parse))
        .map(|r| r.map(Result::unwrap))
        .map(|r| r.collect::<Vec<_>>())
        .collect()
}
