aoc_tools::aoc_sol!(day09: part1, part2);

fn part1(input: &str) -> usize {
    let mut filesystem = parse_input(input);
    filesystem.compact();
    filesystem.checksum()
}

fn part2(input: &str) -> usize {
    let mut filesystem = parse_input(input);
    filesystem.compact_p2();
    filesystem.checksum()
}

fn parse_input(input: &str) -> FileSystem {
    FileSystem::from_int_iter(
        input.chars().map(|ch| (ch as u8 - b'0') as u64),
    )
}

#[derive(Debug, Clone)]
struct FileSystem {
    blocks: Vec<Block>,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Block {
    File { id: u32, size: u64 },
    Free { size: u64 },
}
impl Block {
    pub fn free(&self) -> Option<u64> {
        if let Self::Free { size } = self {
            Some(*size)
        } else {
            None
        }
    }
    pub fn file(&self) -> Option<(u64, u32)> {
        if let Self::File { size, id } = self {
            Some((*size, *id))
        } else {
            None
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File { id, size } => write!(f, "File<{size} bytes, #{id}>"),
            Self::Free { size } => write!(f, "Free<{size}>"),
        }
    }
}


impl FileSystem {
    fn from_int_iter(ints: impl Iterator<Item = u64>) -> Self {
        let mut next_is_free = false;
        let mut next_file_id = 0;
        let mut blocks = vec![];
        for int in ints {
            if next_is_free {
                blocks.push(Block::Free { size: int });
                next_is_free = false;
            } else {
                blocks.push(Block::File { id: next_file_id, size: int });
                next_file_id += 1;
                next_is_free = true;
            }
        }

        Self { blocks }
    }

    fn next_free_block_idx(&self, start: usize) -> Option<(usize, u64)> {
        self.blocks.iter()
            .enumerate()
            .skip(start)
            .find_map(|(i, block)| block.free().map(|size| (i, size)))
    }

    fn compact(&mut self) {
        let mut min_free_idx = 0;
        while let Some((idx, free_size)) = self.next_free_block_idx(min_free_idx) {
            min_free_idx = idx;

            let Some(&Block::File { id, size }) = self.blocks.last() else {
                self.blocks.pop();
                continue;
            };

            if size < free_size {
                self.blocks[idx] = Block::Free { size: free_size - size };
                self.blocks.insert(idx, Block::File { id, size });
                self.blocks.pop();
            } else {
                self.blocks[idx] = Block::File { id, size: free_size };
                if size == free_size {
                    self.blocks.pop();
                } else {
                    let last_idx = self.blocks.len() - 1;
                    self.blocks[last_idx] = Block::File { id, size: size - free_size };
                }
            }
        }
    }

    fn compact_p2(&mut self) {
        let mut max_file_idx = self.blocks.len();
        let mut insertions = HashMap::<usize, Vec<Block>>::new();

        loop {
            let mut file = None;
            for idx in (0..max_file_idx.min(self.blocks.len())).rev() {
                let Some((size, id)) = self.blocks[idx].file() else { continue };
                file = Some((idx, (size, id)));
                break;
            }

            if let Some((file_idx, (file_size, id))) = file {
                max_file_idx = file_idx;
                let mut curr_free_idx = 0;
                while let Some((free_idx, free_size)) = self.next_free_block_idx(curr_free_idx + 1) {
                    if free_idx >= file_idx { break; }

                    if free_size >= file_size {
                        self.blocks[file_idx] = Block::Free { size: file_size };
                        self.blocks[free_idx] = Block::Free { size: free_size - file_size };
                        insertions.entry(free_idx).or_default().push(Block::File { size: file_size, id });
                        break;
                    }

                    curr_free_idx = free_idx;
                }
            } else {
                break
            }
        }

        self.blocks = self.blocks.iter()
            .copied()
            .enumerate()
            .flat_map(|(i, block)| if let Some(blocks) = insertions.remove(&i) {
                blocks.into_iter().chain([block])
            } else {
                vec![].into_iter().chain([block])
            })
            .collect();

        self.combine_adjacent_frees_and_remove_trailing();
    }

    fn combine_adjacent_frees_and_remove_trailing(&mut self) {
        let mut new_blocks = vec![];

        let mut current_free_size = None;
        for i in 0..self.blocks.len() {
            match (current_free_size, self.blocks[i].free()) {
                (Some(a), Some(b)) => {
                    current_free_size = Some(a + b);
                },
                (Some(size), None) => {
                    current_free_size = None;
                    new_blocks.push(Block::Free { size: size });
                    new_blocks.push(self.blocks[i]);
                },
                (None, Some(size)) => {
                    current_free_size = Some(size);
                },
                (None, None) => {
                    new_blocks.push(self.blocks[i]);
                }
            }
        }

        self.blocks = new_blocks;
    }

    fn checksum(&mut self) -> usize {
        let mut running_idx: usize = 0;
        let mut sum = 0;
        for &block in self.blocks.iter() {
            let (size, id) = match block {
                Block::File { id, size } => (size, id),
                Block::Free { size } => (size, 0),
            };
            let mut subsum = 0;
            for i in running_idx..running_idx + size as usize {
                subsum += i * id as usize;
            }
            running_idx += size as usize;
            sum += subsum;
        }

        sum
    }

    #[allow(dead_code)]
    fn canonical_representation(&self) -> String {
        let mut s = String::new();
        for &block in self.blocks.iter() {
            match block {
                Block::File { id, size } => for _ in 0..size {
                    s.push((id as u8 + b'0') as char);
                },
                Block::Free { size } => for _ in 0..size {
                    s.push('.');
                },
            }
        }
        s
    }
}
