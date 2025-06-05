aoc_tools::aoc_sol!(day16 2019: part1, part2);

type Scalar = i64;

#[derive(Debug)]
struct Sections {
    sums: Vec<Vec<Scalar>>,
}
impl Sections {
    #[inline(never)]
    pub fn from_inputs(input: &[Scalar]) -> Self {
        let mut output = Self { sums: vec![Vec::with_capacity(input.len())] };
        for i in 0..input.len() {
            output.sums[0].push(input[i]);
        }
        for bit in 1..usize::BITS as u8 - input.len().leading_zeros() as u8 {
            let step_size = 1 << bit;
            let count = input.len() / step_size;
            output.sums.push(Vec::with_capacity(count));
            for i in 0..count {
                let l = output.sums[bit as usize - 1][i * 2];
                let r = output.sums[bit as usize - 1][i * 2 + 1];
                output.sums[bit as usize].push(l + r);
            }
        }
        output
    }
    // #[inline(never)]
    pub fn get_sum(&self, range: (usize, usize)) -> Scalar {
        fn get_midpoint(range: (usize, usize)) -> usize {
            let mut midpoint = range.0;
            loop {
                let diff = 1 << midpoint.trailing_zeros();
                if midpoint + diff > range.1 { break } 
                else { midpoint += diff; }
            }
            midpoint
        }
        fn get_start_to_mid(s: &Sections, mut start: usize, mid: usize) -> Scalar {
            let mut sum = 0;
            for bit in 0..mid.trailing_zeros() {
                let step = 1 << bit;
                if bit >= start.trailing_zeros() {
                    sum += s.sums[bit as usize][start / step];
                    start += step;
                }
            }
            sum
        }
        fn get_mid_to_end(s: &Sections, mut mid: usize, end: usize) -> Scalar {
            let mut sum = 0;
            for bit in (0..mid.trailing_zeros()).rev() {
                let step = 1 << bit;
                if mid + step <= end {
                    sum += s.sums[bit as usize][mid / step];
                    mid += step;
                }
            }
            sum
        }

        if range.0 == range.1 { return 0 }
        let midpoint = get_midpoint(range);
        let s_to_m = get_start_to_mid(self, range.0, midpoint);
        let m_to_e = get_mid_to_end(self, midpoint, range.1);
        s_to_m + m_to_e
    }
}

pub fn pattern(output_idx: usize) -> impl Iterator<Item = Scalar> {
    (0_usize..)
        .map(move |i| (i / (output_idx+1)) % 4)
        .map(|i| [0, 1, 0, -1][i])
        .skip(1)
}

pub fn ranges(output_idx: usize) -> impl Iterator<Item = ((usize, usize), Scalar)> {
    let index_multiplier = output_idx + 1;
    (0_usize..).map(move |i| {
        let mul = [0, 1, 0, -1][i % 4];
        let start = i * index_multiplier;
        let end = (i+1) * index_multiplier;
        ((start, end), mul)
    })
}

pub fn to_num(digits: &[Scalar]) -> Scalar {
    digits.iter().fold(0, |num, digit| num * 10 + digit)
}

pub fn part1(input: &str) -> Scalar {
    let mut input = parse_input(input);
    for _ in 0..100 {
        let mut output = vec![0; input.len()];
        for output_digit_idx in 0..output.len() {
            let output_digit = input.iter()
                .copied()
                .zip(pattern(output_digit_idx))
                .fold(0, |sum, (a, b)| sum + a * b);

            output[output_digit_idx] = output_digit.abs() % 10; // Set last digit
        }
        input = output;
    }
    input[0..8].iter().fold(0, |n, digit| n * 10 + *digit)
}

pub fn part2(input: &str) -> Scalar {
    let mut input = parse_input(input).repeat(10000);
    let offset = to_num(&input[0..7]) as usize;
    for i in 0..100 {
        input.insert(0, 0);
        let sections = Sections::from_inputs(&input);

        let mut output = vec![];
        for output_digit_idx in 0..input.len()-1 {
            let mut sum = 0;
            for ((start, end), mul) in ranges(output_digit_idx) {
                if start >= input.len() { break }
                if mul == 0 { continue }

                let end = end.min(input.len());

                let range_sum = if output_digit_idx > 4 {
                    sections.get_sum((start, end))
                } else {
                    input[start..end].iter().sum()
                };
                sum += mul * range_sum;
            }
            output.push(sum.abs() % 10);
        }
        input = output;
        println!("{i}");
    }
    to_num(&input[offset..offset+8])
}

fn parse_input(input: &str) -> Vec<Scalar> {
    input
        .trim_ascii()
        .chars()
        .map(|c| c as u8 - b'0')
        .map(|v| v as Scalar)
        .collect()
}
