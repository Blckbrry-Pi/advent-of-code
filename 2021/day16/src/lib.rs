use std::str::FromStr;

aoc_tools::aoc_sol!(day16 2021: part1, part2);

#[derive(Clone, PartialEq, Eq)]
struct BitMap(Vec<u64>);
impl BitMap {
    pub fn get_bits(&self, bit_idx: usize, bit_count: usize) -> u64 {
        let idx = bit_idx / 64;
        let offset = bit_idx % 64;
        let left = self.0[idx] << offset;
        let right = if offset == 0 {
            0
        } else {
            *self.0.get(idx+1).unwrap_or(&0) >> (64 - offset)
        };
        let full_u64_bits = left | right;
        full_u64_bits >> (64 - bit_count)
    }
    pub fn parse_packet(&self, start: usize) -> ((u64, u64), usize) {
        let version = self.get_bits(start, 3) as u8;
        let type_id = self.get_bits(start + 3, 3) as u8;
        if type_id == 4 {
            let mut offset = start + 6;
            let mut output = 0;
            loop {
                let a = self.get_bits(offset, 5);
                output <<= 4;
                output |= a & 0b1111;
                offset += 5;
                if a & 0b10000 == 0 { break }
            }
            return (
                (version as u64, output),
                offset,
            );
        }

        let length_type_id = self.get_bits(start + 6, 1);
        let (bits, packet_count, mut curr_offset) = if length_type_id == 0 {
            (self.get_bits(start + 7, 15) as u16, u16::MAX, start + 22)
        } else {
            (u16::MAX, self.get_bits(start + 7, 11) as u16, start + 18)
        };
        let mut output = match type_id {
            0 => 0,
            1 => 1,
            2 => u64::MAX,
            3 => 0,
            5 | 6 | 7 => u64::MAX,
            _ => panic!("Invalid packet type id"),
        };
        let mut version_sum = version as u64;
        let mut i = 0;
        while curr_offset < start + 22 + bits as usize && i < packet_count {
            let ((sub_version, sub_output), new_offset) = self.parse_packet(curr_offset);
            curr_offset = new_offset;

            match type_id {
                0 => output += sub_output,
                1 => output *= sub_output,
                2 => output = output.min(sub_output),
                3 => output = output.max(sub_output),
                v @ (5 | 6 | 7) => {
                    let expected = match v {
                        5 => std::cmp::Ordering::Greater,
                        6 => std::cmp::Ordering::Less,
                        _ => std::cmp::Ordering::Equal,
                    };
                    if output == u64::MAX {
                        output = sub_output;
                    } else if output.cmp(&sub_output) == expected {
                        output = 1;
                    } else {
                        output = 0;
                    }
                },
                _ => panic!("Invalid type ID"),
            }
            version_sum += sub_version;
            i += 1;
        }
        (
            (version_sum, output),
            curr_offset,
        )
    }
}
impl Debug for BitMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &byte in &self.0 {
            write!(f, "{byte:016X}")?;
        }
        Ok(())
    }
}
impl FromStr for BitMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes()
            .chunks(16)
            .map(|chunk| {
                let mut output = 0_u64;
                for i in 0..16 {
                    let byte = *chunk.get(i).unwrap_or(&b'0');
                    output <<= 4;
                    output |= if byte > b'9' {
                        byte - b'A' + 10
                    } else {
                        byte - b'0'
                    } as u64;
                }
                output
            })
            .collect();
        Ok(Self(s))
    }
}

pub fn part1(input: &str) -> u64 {
    let bit_map = parse_input(input);
    bit_map.parse_packet(0).0.0
}

pub fn part2(input: &str) -> u64 {
    let bit_map = parse_input(input);
    bit_map.parse_packet(0).0.1
}

fn parse_input(input: &str) -> BitMap {
    input.trim().parse().unwrap()
}
