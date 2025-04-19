#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    TurnL,
    TurnR,
    Advance(usize),
}

pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut remaining = input.trim();
    let mut output = vec![];
    while !remaining.is_empty() {
        let next = remaining.as_bytes()[0] as char;
        match next {
            'L' => {
                output.push(Instruction::TurnL);
                remaining = &remaining[1..];
                continue
            },
            'R' => {
                output.push(Instruction::TurnR);
                remaining = &remaining[1..];
                continue
            }
            _ => (),
        }
        let mut new_remaining = remaining;
        while !new_remaining.is_empty() && new_remaining.as_bytes()[0].is_ascii_digit() {
            new_remaining = &new_remaining[1..];
        }
        let number = &remaining[..remaining.len()-new_remaining.len()];
        let number = number.parse().unwrap();
        output.push(Instruction::Advance(number));
        remaining = new_remaining;
    }
    output
}