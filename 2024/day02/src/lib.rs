aoc_tools::aoc_sol!(day02 2024: part1, part2);

pub fn part1(input: &str) -> usize {
    let rows = parse_input(input);

    rows.into_iter()
        .filter(|r| r.is_valid(false))
        .count()
}

pub fn part2(input: &str) -> usize {
    let rows = parse_input(input);

    rows.into_iter()
        .filter(|r| r.is_valid(true))
        .count()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Row {
    entries: Vec<u8>,
}

impl Row {
    fn _is_valid_internal(entries: &[u8], skip: Option<usize>) -> bool {
        let mut failed_decreasing = false;
        let mut failed_increasing = false;
        let mut curr = entries[0];
        for i in 1..entries.len() {
            if Some(i) == skip { continue; }
            let new = entries[i];
            if new >= curr || curr - new > 3 {
                failed_decreasing = true;
                if failed_increasing { return false; }
            }
            if new <= curr || new - curr > 3 {
                failed_increasing = true;
                if failed_decreasing { return false; }
            }
            curr = new;
        }
        true
    }

    pub fn is_valid(&self, allow_skip: bool) -> bool {
        if allow_skip {
            for skip in 0..self.entries.len() {
                let entries = if skip == 0 { &self.entries[1..] } else { &self.entries };
                let skip = if skip == 0 { None } else { Some(skip) };
                if Self::_is_valid_internal(entries, skip) {
                    return true;
                }
            }
            false
        } else {
            Self::_is_valid_internal(&self.entries, None)
        }
    }
}

fn parse_input(input: &str) -> Vec<Row> {
    let mut rows = Vec::with_capacity(input.len() / 10);
    let mut curr_row = Row { entries: Vec::with_capacity(8) };
    let mut i = 0;

    while i < input.len() {
        if input.as_bytes()[i] == b'\n' {
            i += 1;
            rows.push(curr_row);
            curr_row = Row { entries: Vec::with_capacity(8) };
            continue;
        }
        let mut val = 0;
        while i < input.len() && b'0' <= input.as_bytes()[i] && input.as_bytes()[i] <= b'9' {
            val *= 10;
            val += input.as_bytes()[i] - b'0';
            i += 1;
        }
        while i < input.len() && input.as_bytes()[i] == b' ' { i += 1; }
        curr_row.entries.push(val);
    }

    if !curr_row.entries.is_empty() {
        rows.push(curr_row);
    }

    rows
}
