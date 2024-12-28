aoc_tools::aoc_sol!(day03: part1, part2);

pub fn part1(input: &str) -> i32 {
    let mut remaining = input;
    let mut sum = 0;
    while let Some((new_remaining, (a, b))) = find_next_mul(remaining) {
        remaining = new_remaining;
        sum += a * b;
    }
    sum
}

pub fn part2(input: &str) -> i32 {
    let mut remaining = input;
    let mut sum = 0;
    while !remaining.is_empty() {
        let Some((new_remaining, mul)) = find_next_mul_dont(remaining) else { break };
        remaining = new_remaining;
        if let Some((a, b)) = mul {
            sum += a * b;
            continue;
        }

        let Some(new_remaining) = find_next_do(remaining) else { break };
        remaining = new_remaining;
    }
    sum
}

fn parse_mul(mut s: &str) -> Result<(&str, (i32, i32)), &str> {
    let m_matches = s.as_bytes()[0] == b'm';
    s = &s[1..];
    if !m_matches {
        return Err(s);
    }

    let Some(ul_part) = s.as_bytes().get(0..3) else { return Err("") };
    if ul_part != b"ul(" {
        return Err(s);
    }
    s = &s[3..];
    
    let Some(comma_part) = s.get(0..4) else { return Err("") };
    let Some(comma_offset) = comma_part.find(',') else { return Err(s) };
    let l = &s[0..comma_offset];
    s = &s[comma_offset+1..];
    
    let Some(paren_part) = s.get(0..s.len().min(4)) else { return Err("") };
    let Some(paren_offset) = paren_part.find(')') else { return Err(s) };
    let r = &s[0..paren_offset];
    s = &s[paren_offset+1..];
    
    if l.as_bytes().iter().any(|b| !b.is_ascii_digit()) { return Err(s) };
    if r.as_bytes().iter().any(|b| !b.is_ascii_digit()) { return Err(s) };

    let l = parse_i32_3(l);
    let r = parse_i32_3(r);

    Ok((s, (l, r)))
}

fn find_next_mul(s: &str) -> Option<(&str, (i32, i32))> {
    let mut s = s;
    while !s.is_empty() {
        if s.as_bytes()[0] != b'm' {
            s = &s[1..];
            continue;
        }
        match parse_mul(s) {
            Ok(v) => return Some(v),
            Err(new_s) => s = new_s,
        }
    }

    None
}
fn find_next_do(s: &str) -> Option<&str> {
    let mut s = s;
    while !s.is_empty() {
        if s.as_bytes()[0] != b'd' {
            s = &s[1..];
            continue;
        }

        if s.as_bytes().get(1..4)? == b"o()" {
            return Some(&s[4..]);
        } else {
            s = &s[1..];
        }
    }
    None
}
fn find_next_mul_dont(s: &str) -> Option<(&str, Option<(i32, i32)>)> {
    let mut s = s;
    while !s.is_empty() {
        match s.as_bytes()[0] {
            b'd' => {
                if s.as_bytes().get(1..7)? == b"on't()" {
                    return Some((&s[7..], None));
                } else {
                    s = &s[1..];
                }
            },
            b'm' => match parse_mul(s) {
                Ok(v) => return Some((v.0, Some(v.1))),
                Err(new_s) => s = new_s,
            },
            _ => s = &s[1..],
        }
    }

    None

}

aoc_tools::parse_unsigned!(parse_i32_3<i32>(<= 3 digits));
