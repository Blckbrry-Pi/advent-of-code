use std::str::FromStr;

aoc_tools::aoc_sol!(day14 2020: part1, part2);

#[derive(Debug, Clone, Copy)]
enum Instruction {
    UpdateMask(Mask),
    SetMem(u64, u64),
}
impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(mask) = s.strip_prefix("mask = ") {
            return Ok(Self::UpdateMask(mask.parse()?));
        }
        let Some(write) = s.strip_prefix("mem[") else {
            return Err("Instruction must be a mask update or a memset".to_string());
        };
        let Some((addr, val)) = write.split_once("] = ") else {
            return Err("Invalid memset format".to_string());
        };
        let addr = addr.parse::<u64>().map_err(|e| e.to_string())?;
        let val = val.parse::<u64>().map_err(|e| e.to_string())?;
        Ok(Self::SetMem(addr, val))
    }
}



#[derive(Clone, Copy, PartialEq, Eq)]
struct Mask { and_mask: u64, or_mask: u64 }
impl Mask {
    pub fn mask(&self, value: u64) -> u64 {
        (self.and_mask & value) | self.or_mask
    }
    pub fn iter(self) -> impl Iterator<Item = Mask> {
        (0..1 << self.and_mask.count_ones())
            .map(move |n| {
                let mut output = Self { and_mask: 0, or_mask: 0 };
                let mut and_mask_idx = 0;
                for i in (0..36).rev() {
                    output.and_mask <<= 1;
                    output.or_mask <<= 1;
                    let and_mask_bit = (self.and_mask >> i) & 0b1 == 1;
                    let or_mask_bit = (self.or_mask >> i) & 0b1 == 1;
                    if or_mask_bit {
                        output.or_mask |= 1;
                    } else if and_mask_bit {
                        if (n >> and_mask_idx) & 0b1 == 1 {
                            output.or_mask |= 1;
                        }
                        and_mask_idx += 1;
                    } else {
                        output.and_mask |= 1;
                    }
                }

                output
            })
    }
}
impl Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..36).rev() {
            let and_mask_bit = (self.and_mask >> i) & 0b1 == 1;
            let or_mask_bit = (self.or_mask >> i) & 0b1 == 1;
            if or_mask_bit {
                write!(f, "1")
            } else if and_mask_bit {
                write!(f, "X")
            } else {
                write!(f, "0")
            }?;
        }
        Ok(())
    }
}
impl FromStr for Mask {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.len() == 36 || !s.is_ascii() {
            return Err("Mask must be a 36 bit ascii string".to_string())
        }
        let mut output = Self { and_mask: 0, or_mask: 0 };
        for i in 0..36 {
            output.and_mask <<= 1;
            output.or_mask <<= 1;
            let c = s.as_bytes()[i];
            match c as char {
                '0' => (),
                'X' => output.and_mask |= 1,
                '1' => output.or_mask |= 1,
                c => return Err(format!("Invalid mask char {}", c)),
            }
        }
        Ok(output)
    }
}

pub fn part1(input: &str) -> u64 {
    let instructions = parse_input(input);
    let mut mask = Mask { and_mask: u64::MAX, or_mask: 0 };
    let mut values = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::UpdateMask(new_mask) => mask = new_mask,
            Instruction::SetMem(addr, val) => { values.insert(addr, mask.mask(val)); },
        }
    }
    values.values().sum()
}

pub fn part2(input: &str) -> u64 {
    let instructions = parse_input(input);
    let mut mask = Mask { and_mask: u64::MAX, or_mask: 0 };
    let mut values = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::UpdateMask(new_mask) => mask = new_mask,
            Instruction::SetMem(addr, val) => for addr_mask in mask.iter() {
                values.insert(addr_mask.mask(addr), val);
            },
        }
    }
    values.values().sum()
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}
