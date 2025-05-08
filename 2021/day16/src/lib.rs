use std::str::FromStr;

aoc_tools::aoc_sol!(day16 2021: part1, part2);

#[derive(Debug, Clone, PartialEq)]
enum Packet {
    Literal {
        version: u8,
        type_id: u8,
        value: u64,
    },
    OperatorBits {
        version: u8,
        type_id: u8,
        bits: u16,
        sub_packets: Vec<Packet>,
    },
    OperatorCount {
        version: u8,
        type_id: u8,
        packet_count: u16,
        sub_packets: Vec<Packet>,
    }
}
impl Packet {
    pub fn version_number_sum(&self) -> u64 {
        match self {
            Self::Literal { version, .. } => *version as u64,
            | Self::OperatorBits  { version, sub_packets, .. }
            | Self::OperatorCount { version, sub_packets, .. } => {
                *version as u64 + sub_packets.iter().map(|p| p.version_number_sum()).sum::<u64>()
            },
        }
    }
    pub fn value(&self) -> u64 {
        match self {
            Self::Literal { value, .. } => *value,
            | Self::OperatorBits  { type_id, sub_packets, .. }
            | Self::OperatorCount { type_id, sub_packets, .. } => match *type_id {
                0 => sub_packets.iter().map(|p| p.value()).sum(),
                1 => sub_packets.iter().map(|p| p.value()).product(),
                2 => sub_packets.iter().map(|p| p.value()).min().unwrap(),
                3 => sub_packets.iter().map(|p| p.value()).max().unwrap(),
                5 => if sub_packets[0].value() > sub_packets[1].value() { 1 } else { 0 },
                6 => if sub_packets[0].value() < sub_packets[1].value() { 1 } else { 0 },
                7 => if sub_packets[0].value() == sub_packets[1].value() { 1 } else { 0 },
                _ => panic!("Invalid packet"),
            }
        }
    }
}

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
    pub fn parse_packet(&self, start: usize) -> (Packet, usize) {
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
                Packet::Literal {
                    version,
                    type_id,
                    value: output,
                },
                offset,
            );
        }
        let length_type_id = self.get_bits(start + 6, 1);
        if length_type_id == 0 {
            let bits = self.get_bits(start + 7, 15) as u16;
            let mut curr_offset = start + 22;
            let mut sub_packets = vec![];
            while curr_offset < start + 22 + bits as usize {
                let (new_packet, new_offset) = self.parse_packet(curr_offset);
                curr_offset = new_offset;
                sub_packets.push(new_packet);
            }
            (
                Packet::OperatorBits {
                    version,
                    type_id,
                    bits,
                    sub_packets,
                },
                curr_offset,
            )
        } else {
            let packet_count = self.get_bits(start + 7, 11) as u16;
            let mut curr_offset = start + 18;
            let mut sub_packets = vec![];
            for _ in 0..packet_count {
                let (new_packet, new_offset) = self.parse_packet(curr_offset);
                curr_offset = new_offset;
                sub_packets.push(new_packet);
            }
            (
                Packet::OperatorCount {
                    version,
                    type_id,
                    packet_count,
                    sub_packets,
                },
                curr_offset,
            )
        }
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
    bit_map.parse_packet(0).0.version_number_sum()
}

pub fn part2(input: &str) -> u64 {
    let bit_map = parse_input(input);
    bit_map.parse_packet(0).0.value()
}

fn parse_input(input: &str) -> BitMap {
    input.trim().parse().unwrap()
}
