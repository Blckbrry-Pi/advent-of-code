aoc_tools::aoc_sol!(day10 2021: part1, part2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BraceType {
    Paren,
    Square,
    Curly,
    Angle,
}
impl BraceType {
    pub fn syntax_err_score(&self) -> u64 {
        match self {
            Self::Paren => 3,
            Self::Square => 57,
            Self::Curly => 1197,
            Self::Angle => 25137,
        }
    }
    pub fn autocomplete_score(rev_list: &[BraceType]) -> u64 {
        let mut curr = 0;
        for &brace in rev_list.iter().rev() {
            curr *= 5;
            curr += match brace {
                Self::Paren => 1,
                Self::Square => 2,
                Self::Curly => 3,
                Self::Angle => 4,
            };
        }
        curr
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Open(BraceType),
    Close(BraceType),
}
impl Token {
    fn parse(c: char) -> Self {
        match c {
            '(' => Self::Open(BraceType::Paren),
            ')' => Self::Close(BraceType::Paren),
            '[' => Self::Open(BraceType::Square),
            ']' => Self::Close(BraceType::Square),
            '{' => Self::Open(BraceType::Curly),
            '}' => Self::Close(BraceType::Curly),
            '<' => Self::Open(BraceType::Angle),
            '>' => Self::Close(BraceType::Angle),
            _ => panic!("Invalid character"),
        }
    }

    fn get_line_status(tokens: &[Token]) -> Result<Vec<BraceType>, BraceType> {
        let mut stack = vec![];
        for &token in tokens {
            match token {
                Self::Open(brace) => stack.push(brace),
                Self::Close(brace) => if let Some(expected) = stack.pop() {
                    if brace != expected {
                        return Err(brace);
                    }
                },
            }
        }
        Ok(stack)
    }
}


pub fn part1(input: &str) -> u64 {
    let lines = parse_input(input);
    let mut total = 0;
    for tokens in lines {
        let Err(token) = Token::get_line_status(&tokens) else { continue };
        total += token.syntax_err_score();
    }
    total
}

pub fn part2(input: &str) -> u64 {
    let lines = parse_input(input);
    let mut results = vec![];
    for tokens in lines {
        let Ok(stack) = Token::get_line_status(&tokens) else { continue };
        results.push(BraceType::autocomplete_score(&stack));
    }
    results.sort();
    results[results.len() / 2]
}

fn parse_input(input: &str) -> Vec<Vec<Token>> {
    input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(Token::parse).collect())
        .collect()
}
