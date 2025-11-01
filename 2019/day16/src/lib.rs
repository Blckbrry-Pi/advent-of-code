aoc_tools::aoc_sol!(day16 2019: part1, part2);

type Scalar = i32;

#[derive(Debug)]
struct Sections {
    sums: Vec<Scalar>,
}
impl Sections {
    pub fn from_inputs(input: &[Scalar]) -> Self {
        let mut running_sum = 0;
        let mut sums = Vec::with_capacity(input.len() + 1);
        sums.push(0);
        for i in 0..input.len() {
            running_sum += input[i];
            sums.push(running_sum);
        }
        Self { sums }
    }
    pub fn get_sum(&self, range: (usize, usize)) -> Scalar {
        self.sums[range.1] - self.sums[range.0]
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
        let (start, end) = (start.saturating_sub(1), end.saturating_sub(1));
        ((start, end), mul)
    })
}

pub fn to_num(digits: &[Scalar]) -> Scalar {
    digits.iter().fold(0, |num, digit| num * 10 + digit)
}

pub fn part1(input: &str) -> Scalar {
    let mut input = parse_input(input);

    let ranges: Vec<Vec<_>> = (0..input.len())
        .map(ranges)
        .map(|range| range.take_while(|v| v.0.0 < input.len()))
        .map(|range| range.map(|v| ((v.0.0, v.0.1.min(input.len())), v.1)))
        .map(|range| range.filter(|v| v.1 != 0))
        .map(|range| range.collect())
        .collect();

    for _ in 0..100 {
        let sections = Sections::from_inputs(&input);
        let mut output = vec![];
        for output_digit_idx in 0..input.len() {
            let mut sum = 0;
            for &((start, end), mul) in &ranges[output_digit_idx] {
                sum += mul * sections.get_sum((start, end));
            }
            output.push(sum.abs() % 10);
        }
        input = output;
    }
    to_num(&input[0..8])
}

// #[inline(never)]
pub fn part2rangesgen(offset: usize, len: usize) -> Vec<Vec<((usize, usize), Scalar)>> {
    (offset..len)
        .map(ranges)
        .map(|range| range.skip_while(|&((_, e), _)| e < offset))
        .map(|range| range.take_while(|&((s, _), _)| s < len))
        .map(|range| range.filter(|&(_, mul)| mul != 0))
        .map(|range| range.map(|((s, e), mul)| ((s, e.min(len)), mul)))
        .map(|range| range.map(|((s, e), mul)| ((s.max(offset), e), mul)))
        .map(|range| range.map(|((s, e), mul)| ((s - offset, e - offset), mul)))
        .map(|range| range.collect())
        .collect()
}

pub fn part2(input: &str) -> Scalar {
    let input = parse_input(input);
    let base_len = input.len();
    let offset = to_num(&input[0..7]) as usize;

    let mut copies_omitted = offset / base_len;
    let mut skipped = offset - copies_omitted * base_len;
    let mut input: Vec<_> = input.iter()
        .skip(skipped)
        .chain(std::iter::repeat_n(&input, 10_000 - copies_omitted - 1).flatten())
        .copied()
        .collect();


    let ranges: Vec<Vec<_>> = part2rangesgen(offset, base_len * 10_000);

    for i in 0..100 {
        let sections = Sections::from_inputs(&input);
        let mut output = vec![];
        for output_digit_idx in 0..input.len() {
            let mut sum = 0;
            for &((start, end), mul) in &ranges[output_digit_idx] {
                sum += mul * sections.get_sum((start, end));
            }
            output.push(sum.abs() % 10);
        }
        input = output;
    }

    to_num(&input[..8])
}

fn parse_input(input: &str) -> Vec<Scalar> {
    input
        .trim_ascii()
        .chars()
        .map(|c| c as u8 - b'0')
        .map(|v| v as Scalar)
        .collect()
}
