aoc_tools::aoc_sol!(day08 2019: part1, part2);
aoc_tools::map_struct!(Layer of Pixel, pos i16);

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pixel(u8);
impl Pixel {
    const BLACK: Self = Self(0);
    const WHITE: Self = Self(1);
    const TRANSPARENT: Self = Self(2);
    pub fn from_u8(v: u8) -> Self {
        Self(v - b'0')
    }
}
impl Debug for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "█"),
            1 => write!(f, " "),
            2 => write!(f, "░"),
            v => write!(f, "{v}"),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let layers = parse_input(input);
    let min_zero_layer = layers.into_iter()
        .min_by_key(|l| l.count_matching(|&v| v == Pixel::BLACK))
        .unwrap();
    let ones = min_zero_layer.count_matching(|&v| v == Pixel::WHITE);
    let twos = min_zero_layer.count_matching(|&v| v == Pixel::TRANSPARENT);
    ones * twos
}

pub fn part2(input: &str) -> i64 {
    let layers = parse_input(input);
    let mut output = layers[0].clone();
    for layer in &layers[1..] {
        for y in 0..layer.height() {
            for x in 0..layer.width() {
                if output.rows[y][x] == Pixel::TRANSPARENT {
                    output.rows[y][x] = layer.rows[y][x];
                }
            }
        }
    }
    // TODO: Decode it into a `String`
    println!("{output:?}");
    0
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const PIXELS_PER_LAYER: usize = WIDTH * HEIGHT;

fn parse_input(input: &str) -> Vec<Layer> {
    let input = input.trim_ascii();
    let layer_count = input.len() / PIXELS_PER_LAYER;
    let mut layers = Vec::with_capacity(layer_count);
    for l in 0..layer_count {
        let input = &input.as_bytes()[l * PIXELS_PER_LAYER..(l+1) * PIXELS_PER_LAYER];
        let mut layer = Layer { rows: Vec::with_capacity(HEIGHT) };
        for y in 0..HEIGHT {
            layer.rows.push(Vec::with_capacity(WIDTH));
            for x in 0..WIDTH {
                let idx = y * WIDTH + x;
                layer.rows[y].push(Pixel::from_u8(input[idx]));
            }
        }
        layers.push(layer);
    }
    layers
}
