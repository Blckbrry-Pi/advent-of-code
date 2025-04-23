#![feature(new_range_api)]

aoc_tools::aoc_sol!(day20 2022: part1, part2);

pub fn part1(input: &str) -> i64 {
    let mut list = parse_input(input);
    list.do_mix();
    // list.update_list();
    list.grove_coord_sum()
}

pub fn part2(input: &str) -> i64 {
    let mut list = parse_input(input);
    list.key_part2();
    for _ in 0..10 { list.do_mix(); }
    // list.update_list();
    list.grove_coord_sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Entry {
    offset: i64,
    orig_idx: usize,
}

#[derive(Debug, Clone)]
struct List {
    list: Vec<Entry>,
    indexes: Vec<usize>,
    index: usize,
}
impl List {
    pub fn do_mix(&mut self) {
        self.index = 0;
        while self.index < self.list.len() {
            self.process();
        }
    }
    pub fn process(&mut self) {
        let position = self.indexes[self.index];
        let offset = self.list[position].offset as isize;
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
    fn move_right(&mut self, i: usize, offset: usize) {
        let temp = self.list[i];
        for idx in 0..offset {
            let idx = i + idx;
            let replacement = self.list[idx + 1];
            self.list[idx] = replacement;
            self.indexes[replacement.orig_idx] = idx;
        }
        self.list[i + offset] = temp;
        self.indexes[temp.orig_idx] = i + offset;
    }
    fn move_left(&mut self, i: usize, offset: usize) {
        let temp = self.list[i];
        for idx in 0..offset {
            let idx = i - idx;
            let replacement = self.list[idx - 1];
            self.list[idx] = replacement;
            self.indexes[replacement.orig_idx] = idx;
        }
        self.list[i - offset] = temp;
        self.indexes[temp.orig_idx] = i - offset;
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

            print!("    nums [");
            for (i, num) in self.list.iter().enumerate() {
                if i != 0 {
                    print!(", ");
                }
                print!("{}", num.offset);
            }
            println!("]");

            println!("    idxs {:?}", self.indexes);
            println!("}}");
            println!();
        }
    }
    fn get(&self, i: isize) -> i64 {
        let i = i.rem_euclid(self.list.len() as isize);
        self.list[i as usize].offset
    }
    fn find(&self, value: i64) -> Option<usize> {
        self.list.iter().enumerate().find_map(|(i, n)| (n.offset == value).then_some(i))
    }
    fn key_part2(&mut self) {
        self.list.iter_mut().for_each(|n| n.offset *= 811589153);
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
        .enumerate()
        .map(|(orig_idx, offset)| offset.map(|offset| Entry { orig_idx, offset }))
        .collect::<Result<_, _>>()
        .unwrap();
    List {
        indexes: (0..list.len()).collect(),
        list,
        index: 0,
    }
}
