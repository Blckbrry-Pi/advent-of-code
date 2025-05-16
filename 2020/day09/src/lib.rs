aoc_tools::aoc_sol!(day09 2020: part1, part2);

const PREAMBLE_SIZE: usize = 25;

pub fn find_invalid(nums: &[u64], preamble: usize) -> Option<u64> {
    for target in preamble..nums.len() {
        let target_is_valid = 'valid: {
            for a in target-preamble..target {
                for b in a+1..target {
                    if nums[a] + nums[b] == nums[target] {
                        break 'valid true;
                    }
                }
            }
            false
        };
        if !target_is_valid {
            return Some(nums[target]);
        }
    }
    None
}

pub fn part1(input: &str) -> u64 {
    let nums = parse_input(input);
    find_invalid(&nums, 25).unwrap()
}

pub fn part2(input: &str) -> u64 {
    let nums = parse_input(input);
    let invalid = find_invalid(&nums, 25).unwrap();
    for start in 0..nums.len() {
        let mut sum = 0;
        for i in start..nums.len() {
            sum += nums[i];
            if sum == invalid {
                let range = &nums[start..=i];
                return range.iter().min().unwrap() + range.iter().max().unwrap();
            } else if sum > invalid {
                break
            }
        }
    }
    panic!("No sum run found")
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}
