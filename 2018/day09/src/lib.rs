aoc_tools::aoc_sol!(day09 2018: part1, part2);

struct Marbles(Vec<(u32, u32)>, u32);
impl Marbles {
    pub fn add(&mut self, marble: u32) -> u64 {
        if marble == 0 {
            self.0.push((0, 0));
            self.1 = 0;
            0
        } else if marble % 23 == 0 {
            let mut curr = self.1;
            for _ in 0..7 {
                curr = self.0[curr as usize].0;
            }
            let (a, b) = self.0[curr as usize];
            self.0.push((u32::MAX, u32::MAX));
            self.0[curr as usize] = (u32::MAX, u32::MAX);
            self.0[a as usize].1 = b;
            self.0[b as usize].0 = a;
            self.1 = b;
            curr as u64 + marble as u64
        } else {
            let a = self.0[self.1 as usize].1;
            let b = self.0[a as usize].1;
            self.0.push((a, b));
            self.0[a as usize].1 = marble as u32;
            self.0[b as usize].0 = marble as u32;
            self.1 = marble as u32;
            0
        }
    }
}
impl Debug for Marbles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut curr = self.1;
        write!(f, "\x1b[1m({curr})\x1b[0m ")?;
        loop {
            curr = self.0[curr as usize].1;
            if curr == self.1 { break; }
            write!(f, "{curr} ")?;
        }
        Ok(())
    }
}

pub fn part1(input: &str) -> u64 {
    let (players, worth) = parse_input(input);
    let mut players = vec![0; players as usize];
    let mut marbles = Marbles(Vec::with_capacity(worth as usize + 1), 0);
    marbles.add(0);
    for i in 1..=worth {
        let player_idx = (i as usize - 1) % players.len();
        players[player_idx] += marbles.add(i);
    }
    players.into_iter().max().unwrap()
}

pub fn part2(input: &str) -> u64 {
    let (players, worth) = parse_input(input);
    let worth = worth * 100;
    let mut players = vec![0; players as usize];
    let mut marbles = Marbles(Vec::with_capacity(worth as usize + 1), 0);
    marbles.add(0);
    for i in 1..=worth {
        let player_idx = (i as usize - 1) % players.len();
        players[player_idx] += marbles.add(i);
    }
    players.into_iter().max().unwrap()
}

fn parse_input(input: &str) -> (u16, u32) {
    let (players, worth) = input.trim().split_once(" players; last marble is worth ").unwrap();
    let worth = worth.strip_suffix(" points").unwrap();
    let players = players.parse::<u16>().unwrap();
    let worth = worth.parse::<u32>().unwrap();
    (players, worth)
}
