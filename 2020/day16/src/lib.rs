use std::str::FromStr;

aoc_tools::aoc_sol!(day16 2020: part1, part2);

#[derive(Clone, PartialEq, Eq)]
struct Field(String, (u16, u16), (u16, u16));
impl Field {
    pub fn contains(&self, v: u16) -> bool {
        let range_a_contains = self.1.0 <= v && v <= self.1.1;
        let range_b_contains = self.2.0 <= v && v <= self.2.1;
        range_a_contains || range_b_contains
    }
}
impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: ", self.0)?;
        write!(f, "{}-{} or ", self.1.0, self.1.1)?;
        write!(f, "{}-{}", self.2.0, self.2.1)
    }
}
impl FromStr for Field {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((name, ranges)) = s.split_once(": ") else {
            return Err("Missing colon after field name".to_string());
        };
        let name = name.to_string();
        let Some((range_a, range_b)) = ranges.split_once(" or ") else {
            return Err("Expected multiple ranges".to_string());
        };
        let Some((range_a_lo, range_a_hi)) = range_a.split_once('-') else {
            return Err("Invalid format for range A".to_string());
        };
        let Some((range_b_lo, range_b_hi)) = range_b.split_once('-') else {
            return Err("Invalid format for range B".to_string());
        };
        let range_a_lo = range_a_lo.parse::<u16>().map_err(|e| e.to_string())?;
        let range_a_hi = range_a_hi.parse::<u16>().map_err(|e| e.to_string())?;
        let range_b_lo = range_b_lo.parse::<u16>().map_err(|e| e.to_string())?;
        let range_b_hi = range_b_hi.parse::<u16>().map_err(|e| e.to_string())?;
        Ok(Self(
            name,
            (range_a_lo, range_a_hi),
            (range_b_lo, range_b_hi),
        ))
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Ticket(Vec<u16>);
impl Ticket {
    pub fn completely_invalid_field(&self, fields: &[Field]) -> Option<u16> {
        for &field_val in &self.0 {
            let mut found_matching_field = false;
            for field in fields {
                if field.contains(field_val) {
                    found_matching_field = true;
                    break
                }
            }
            if !found_matching_field {
                return Some(field_val)
            }
        }
        None
    }
}
impl Debug for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ticket< ")?;
        for i in 0..self.0.len() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:3}", self.0[i])?;
        }
        write!(f, " >")
    }
}
impl FromStr for Ticket {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(",").map(|n| n.parse()).collect::<Result<_, _>>().map(Self)
    }
}

pub fn part1(input: &str) -> u32 {
    let (fields, _yours, nearby) = parse_input(input);
    let mut error_rate = 0;
    for ticket in nearby {
        if let Some(err) = ticket.completely_invalid_field(&fields) {
            error_rate += err as u32;
        }
    }
    error_rate
}

fn filter_invalid(tickets: Vec<Ticket>, fields: &[Field]) -> Vec<Ticket> {
    let mut output = Vec::with_capacity(tickets.len());
    for ticket in tickets {
        if ticket.completely_invalid_field(fields).is_some() {
            continue
        }
        output.push(ticket);
    }
    output
}
fn gen_individual_possibilities<'a>(tickets: impl Iterator<Item = &'a Ticket>, fields: &[Field]) -> Vec<HashSet<u8>> {
    let mut possibilities: Vec<HashSet<u8>> = vec![];
    for ticket in tickets {
        for i in 0..ticket.0.len() {
            if possibilities.len() <= i { possibilities.push((0..fields.len() as u8).collect()); }

            for field_id in 0..fields.len() as u8 {
                if !possibilities[i].contains(&field_id) { continue }
                if !fields[field_id as usize].contains(ticket.0[i]) {
                    possibilities[i].remove(&field_id);
                }
            }
        }
    }
    possibilities
}
fn narrow_possibilities(individual_possibilities: Vec<HashSet<u8>>) -> Vec<u8> {
    let mut possibs = individual_possibilities;
    let mut mappings = vec![u8::MAX; possibs.len()];
    loop {
        let mut changed = false;
        for i in 0..possibs.len() {
            let mut trimmed_possibilities = possibs[i].iter().filter(|fid| !mappings.contains(fid));
            if trimmed_possibilities.clone().count() == 1 {
                mappings[i] = *trimmed_possibilities.next().unwrap();
                possibs[i] = HashSet::new();
                changed = true;
                // break;
            }
        }
        if !changed { break mappings }
    }
}
fn gen_pairs<'a>(ticket: &'a Ticket, mappings: &'a [u8], fields: &'a [Field]) -> impl Iterator<Item = (u16, &'a Field)> + 'a {
    mappings.into_iter()
        .enumerate()
        .map(|(tid, fid)| (ticket.0[tid], &fields[*fid as usize]))
}

pub fn part2(input: &str) -> u64 {
    let (fields, yours, nearby) = parse_input(input);
    let nearby = filter_invalid(nearby, &fields);
    let possibilities = gen_individual_possibilities(
        nearby.iter().chain([&yours]),
        &fields,
    );
    let mappings = narrow_possibilities(possibilities);
    gen_pairs(&yours, &mappings, &fields)
        .filter_map(|(v, f)| f.0.starts_with("departure").then_some(v as u64))
        .product()
}

fn parse_input(input: &str) -> (Vec<Field>, Ticket, Vec<Ticket>) {
    let (fields, tickets) = input.split_once("\n\nyour ticket:\n").unwrap();
    let (yours, nearby) = tickets.split_once("\n\nnearby tickets:\n").unwrap();
    let fields = fields.lines().map(|l| l.parse().unwrap()).collect();
    let yours = yours.parse().unwrap();
    let nearby = nearby.lines().map(|l| l.parse().unwrap()).collect();
    (fields, yours, nearby)
}
