aoc_tools::aoc_sol!(day25: part1);

pub fn part1(input: &str) -> usize {
    let Setup { keys, locks } = parse_input(input);
    let mut matches = 0;

    for key in keys {
        for lock in &locks {
            if key.fits(lock) { matches += 1 }
        }
    }

    matches as usize
}

pub fn part2(input: &str) -> usize { input.len() }

fn parse_input(input: &str) -> Setup {
    let mut keys = Vec::with_capacity(input.len() / 80);
    let mut locks = Vec::with_capacity(input.len() / 80);

    let mut i = 0;
    while i < input.len() - 5 {
        let is_lock = input.as_bytes()[i] == b'#';
        let mut col = 0;
        let mut row = 0;
        let mut curr_heights = [0; 5];
        i += 6;
        while row < 5 {
            match input.as_bytes()[i] {
                b'\n' => {
                    row += 1;
                    col = 0;
                },
                b'#' => {
                    curr_heights[col] += 1;
                    col += 1;
                },
                _ => {
                    col += 1;
                },
            }
            i += 1;
        }
        let encoding = Encoding::from(curr_heights);
        if is_lock {
            locks.push(encoding);
        } else {
            keys.push(Encoding::from(curr_heights));
        }
        i += 7;
    }
    Setup { keys, locks }
}

struct Setup { keys: Vec<Encoding>, locks: Vec<Encoding> }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Encoding(u32);
impl Encoding {
    pub fn fits(&self, other: &Self) -> bool {
        (self.0 + other.0 + 0x22222) & 0x88888 == 0
    }
}
impl From<[u8; 5]> for Encoding {
    fn from(value: [u8; 5]) -> Self {
        let [a, b, c, d, e] = value.map(|v| v as u32);
        Self((a << 16) | (b << 12) | (c << 8) | (d << 4) | e)
    }
}
