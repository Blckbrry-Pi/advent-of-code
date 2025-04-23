#![feature(new_range_api)]

aoc_tools::aoc_sol!(day20 2022: part1, part2);

pub fn part1(input: &str) -> i64 {
    let mut list = parse_input(input);
    list.do_mix();
    list.update_list();
    list.grove_coord_sum()
}

pub fn part2(input: &str) -> i64 {
    let mut list = parse_input(input);
    list.key_part2();
    for _ in 0..10 { list.do_mix(); }
    list.update_list();
    list.grove_coord_sum()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Rewrite {
    MoveLeft {
        inc_range: (usize, usize),
        offset_idx: usize,
        new_idx: usize,
    },
    MoveRight {
        dec_range: (usize, usize),
        offset_idx: usize,
        new_idx: usize,
    },
}

#[derive(Clone)]
struct List {
    orig_list: Vec<i64>,
    list: Vec<i64>,
    positions: Vec<(usize, usize)>,
    index: usize,
    rewrites: Vec<Rewrite>
}
impl List {
    pub fn do_mix(&mut self) {
        self.index = 0;
        while self.index < self.list.len() {
            self.process();
            if self.index % 50 == 0 {
                self.canonicalize_positions();
            }
        }
    }

    pub fn process(&mut self) {
        // let position = self.positions[self.index];
        let position = self.get_pos(self.index);
        let offset = self.orig_list[self.index] as isize;
        let len = self.list.len() as isize;
        let reducer = len - 1;
        let final_pos = offset + position as isize;
        let offset = if final_pos >= len && offset != 0 {
            offset - (final_pos - len + reducer) / reducer * reducer
        } else if final_pos <= 0 && offset != 0 {
            offset + (len - final_pos - 1) / reducer * reducer
        } else {
            offset
        };
        if offset > 0 {
            self.move_right(position, offset as usize);
        } else if offset < 0 {
            self.move_left(position, (-offset) as usize);
        }
        self.print_move(position as isize + offset, offset);
        self.index += 1;
    }

    #[inline(never)]
    pub fn get_pos(&mut self, input_index: usize) -> usize {
        let (mut index, start) = self.positions[input_index];
        for &rewrite in &self.rewrites[start..] {
            match rewrite {
                Rewrite::MoveLeft { inc_range, offset_idx, new_idx } => {
                    if inc_range.0 <= index && index < inc_range.1 {
                        index += 1;
                        index %= self.list.len();
                    } else if offset_idx == index {
                        index = new_idx;
                    }
                },
                Rewrite::MoveRight { dec_range, offset_idx, new_idx } => {
                    if dec_range.0 <= index && index < dec_range.1 {
                        index += self.list.len();
                        index -= 1;
                        index %= self.list.len();
                    } else if offset_idx == index {
                        index = new_idx;
                    }
                },
            }
        }
        self.positions[input_index] = (index, self.rewrites.len());
        index
    }
    fn canonicalize_positions(&mut self) {
        for i in 0..self.list.len() {
            self.get_pos(i);
            self.positions[i].1 = 0;
        }
        self.rewrites.clear();
    }
    fn update_list(&mut self) {
        for i in 0..self.orig_list.len() {
            let pos = self.get_pos(i);
            self.list[pos] = self.orig_list[i];
        }
    }

    #[inline(never)]
    fn move_right(&mut self, i: usize, offset: usize) {
        self.rewrites.push(Rewrite::MoveRight {
            dec_range: (i+1, i+offset+1),
            offset_idx: i,
            new_idx: i + offset,
        });
    }
    #[inline(never)]
    fn move_left(&mut self, i: usize, offset: usize) {
        self.rewrites.push(Rewrite::MoveLeft {
            inc_range: (i-offset, i),
            offset_idx: i,
            new_idx: i - offset,
        });
    }
    fn print_move(&self, i: isize, offset: isize) {
        const DEBUG: bool = false;
        if DEBUG {
            if offset != 0 {
                println!(
                    "{} moves between {} and {}",
                    self.get(i),
                    self.get(i-1),
                    self.get(i+1),
                );
            } else {
                println!(
                    "{} doesn't move",
                    self.get(i),
                )
            }
            println!("List {{");
            println!("    nums {:?}", self.list);
            println!("    idxs {:?}", self.positions);
            println!("}}");
            println!();
        }
    }
    fn get(&self, i: isize) -> i64 {
        let i = i.rem_euclid(self.list.len() as isize);
        self.list[i as usize]
    }
    fn find(&self, value: i64) -> Option<usize> {
        self.list.iter().enumerate().find_map(|(i, n)| (*n == value).then_some(i))
    }
    fn key_part2(&mut self) {
        self.orig_list.iter_mut().for_each(|n| *n *= 811589153);
    }
    fn grove_coord_sum(&self) -> i64 {
        let zero = self.find(0).unwrap() as isize;
        self.get(zero + 1000) + self.get(zero + 2000) + self.get(zero + 3000)
    }
}

fn parse_input(input: &str) -> List {
    let list: Vec<_> = input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i64>())
        .collect::<Result<_, _>>()
        .unwrap();
    List {
        positions: (0..list.len()).map(|pos| (pos, 0)).collect(),
        orig_list: list.clone(),
        list,
        index: 0,
        rewrites: vec![],
    }
}
