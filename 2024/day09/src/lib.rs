aoc_tools::aoc_sol!(day09 2024: part1, part2);
aoc_tools::fast_hash!();

pub fn part1(input: &str) -> usize {
    let mut filesystem = parse_input(input);
    filesystem.compact();
    filesystem.checksum()
}

pub fn part2(input: &str) -> usize {
    let mut filesystem = parse_input(input);
    filesystem.compact_p2();
    filesystem.checksum()
}

fn parse_input(input: &str) -> FileSystem {
    FileSystem::from_int_iter(
        input.bytes().map(|ch| (ch - b'0') as u16),
    )
}

#[derive(Debug, Clone)]
struct FileSystem {
    blocks: Vec<Block>,
    file_count: u32,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Block {
    File { id: u16, size: u16 },
    Free { size: u16 },
}
impl Block {
    pub fn free(&self) -> Option<u16> {
        if let Self::Free { size } = self {
            Some(*size)
        } else {
            None
        }
    }
    pub fn file(&self) -> Option<(u16, u16)> {
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
    fn from_int_iter(ints: impl Iterator<Item = u16>) -> Self {
        let mut next_is_free = false;
        let mut next_file_id = 0;
        
        let capacity = if let Some(s) = ints.size_hint().1 { s * 2 } else { 0 };
        let mut blocks = Vec::with_capacity(capacity);
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

        Self { blocks, file_count: next_file_id as u32 }
    }

    fn next_free_block_idx(&self, start: usize) -> Option<(usize, u16)> {
        (start..self.blocks.len())
            .find_map(|i| self.blocks[i].free().map(|size| (i, size)))
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
        let free_count = self.blocks.len() - self.file_count as usize;
        let mut insertions: FastMap<usize, StackVec<9, _>> = new_fastmap_with_capacity(free_count as usize);

        let mut insertions_count = 0;

        let mut free_block_indicies = Vec::with_capacity(free_count as usize + 1);
        for i in 0..self.blocks.len() {
            if matches!(self.blocks[i], Block::Free { .. }) {
                free_block_indicies.push(i);
            }
        }

        loop {
            let mut file = None;
            for idx in (0..max_file_idx.min(self.blocks.len())).rev() {
                let Some((size, id)) = self.blocks[idx].file() else { continue };
                file = Some((idx, (size, id)));
                break;
            }

            if let Some((file_idx, (file_size, id))) = file {
                max_file_idx = file_idx;

                for free_idx in free_block_indicies.iter().copied() {
                    if free_idx >= file_idx { break }

                    let Block::Free { size: free_size } = self.blocks[free_idx] else { continue };
                    if free_size >= file_size {
                        let remaining_free_size = free_size - file_size;
                        self.blocks[file_idx] = Block::Free { size: file_size };
                        self.blocks[free_idx] = Block::Free { size: remaining_free_size };
                        
                        insertions.entry(free_idx)
                            .or_default()
                            .push(Block::File { size: file_size, id });
                        insertions_count += 1;
                        break;
                    }
                }
            } else {
                break
            }
        }

        let mut new_blocks = Vec::with_capacity(self.blocks.len() + insertions_count + 10);

        for i in 0..self.blocks.len() {
            if let Some(blocks) = insertions.get(&i) {
                new_blocks.extend(blocks.into_iter())
            }
            new_blocks.push(self.blocks[i]);
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

            let algebraic = (running_idx + (running_idx + size as usize - 1)) * size as usize / 2;
            subsum += algebraic * id as usize;

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

#[derive(Debug, Clone, Copy)]
struct StackVec<const SIZE: usize, T>([Option<T>; SIZE], usize);

impl<const SIZE: usize, T> StackVec<SIZE, T> {
    pub const fn new() -> Self {
        Self([const { None }; SIZE], 0)
    }

    pub fn push(&mut self, v: T) {
        assert!(self.1 < SIZE);
        self.0[self.1] = Some(v);
        self.1 += 1;
    }

    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.0.into_iter().take(self.1).map(|v| v.unwrap())
    }
}

impl<const SIZE: usize, T> Default for StackVec<SIZE, T> {
    fn default() -> Self {
        Self::new()
    }
}
