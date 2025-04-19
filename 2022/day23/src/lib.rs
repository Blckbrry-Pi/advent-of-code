use std::fmt::Formatter;
use std::str::FromStr;

aoc_tools::aoc_sol!(day23 2022: part1, part2);

aoc_tools::fast_hash!();

#[inline(never)]
pub fn part1(input: &str) -> usize {
    let mut map = parse_input::<false>(input);
    for i in 0..10 {
        map.round_first_half(i);
        // println!("{map:?}");
        map.round_second_half();
    }
    map.count_empty_in(map.bounding_box())
}

#[inline(never)]
pub fn part2(input: &str) -> usize {
    let mut map = parse_input::<true>(input);
    // println!("\x1b[2J\x1b[H");
    for i in 0.. {
        map.round_first_half(i);
        if !map.round_second_half() {
            return i + 1;
        }
        // println!("\x1b[H");
        // println!("{map:?}");
        // std::thread::sleep(std::time::Duration::from_millis(50));
    }
    unreachable!("Uhhhhhhhh")
}

type PosScalar = i8;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: PosScalar,
    y: PosScalar,
}
impl Pos {
    pub const ZERO: Pos = Self::new(0, 0);
    pub const N: Pos = Self::new( 0, -1);
    pub const E: Pos = Self::new( 1,  0);
    pub const S: Pos = Self::new( 0,  1);
    pub const W: Pos = Self::new(-1,  0);

    pub const NE: Pos = Self::N.add(&Self::E);
    pub const NW: Pos = Self::N.add(&Self::W);
    pub const SE: Pos = Self::S.add(&Self::E);
    pub const SW: Pos = Self::S.add(&Self::W);

    pub const MIN: Pos = Self::new(PosScalar::MIN, PosScalar::MIN);
    pub const MAX: Pos = Self::new(PosScalar::MAX, PosScalar::MAX);

    pub const fn new(x: PosScalar, y: PosScalar) -> Self { Self { x, y } }
    pub const fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
    pub const fn sub(&self, other: &Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
    pub const fn neg(&self) -> Self {
        Self::new(-self.x, -self.y)
    }
    pub const fn mul(&self, other: PosScalar) -> Self {
        Self::new(self.x * other, self.y * other)
    }
    pub const fn div(&self, other: PosScalar) -> Self {
        Self::new(self.x / other, self.y / other)
    }

    pub const fn min(&self, other: &Self) -> Self {
        let x = if self.x < other.x { self.x } else { other.x };
        let y = if self.y < other.y { self.y } else { other.y };
        Self::new(x, y)
    }
    pub const fn max(&self, other: &Self) -> Self {
        let x = if self.x > other.x { self.x } else { other.x };
        let y = if self.y > other.y { self.y } else { other.y };
        Self::new(x, y)
    }

    pub const fn manhattan(&self, other: &Self) -> PosScalar {
        PosScalar::abs(self.x - other.x) + PosScalar::abs(self.y - other.y)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum ProposalList {
    #[default]
    None,
    One(usize),
    Many,
}
impl ProposalList {
    pub fn add(&mut self, idx: usize) {
        let new_self = match self {
            ProposalList::None => ProposalList::One(idx),
            ProposalList::One(_) => ProposalList::Many,
            ProposalList::Many => ProposalList::Many,
        };
        *self = new_self;
    }
    pub fn as_u16(&self) -> u16 {
        match self {
            Self::None => u16::MAX,
            Self::Many => u16::MAX - 1,
            &Self::One(idx) => idx as u16,
        }
    }
    pub fn from_u16(n: u16) -> Self {
        match n {
            0xFFFF => ProposalList::None,
            0xFFFE => ProposalList::Many,
            n => ProposalList::One(n as usize),
        }
    }
}

#[repr(align(16384))]
#[derive(Clone, PartialEq, Eq)]
struct PosSetInner([u16; 4096]);

#[derive(Clone, PartialEq, Eq)]
struct PosSet(Box<PosSetInner>);
impl PosSet {
    pub fn new() -> Self {
        Self(Box::new(PosSetInner([0; 4096])))
    }
    fn idx(pos: Pos) -> usize {
        (((pos.y as u8) as usize) << 4) | (((pos.x as u8) as usize) >> 4)
    }
    fn bit(pos: Pos) -> u16 {
        1 << (pos.x as u8 & 0xF)
    }
    pub fn has(&self, pos: Pos) -> bool {
        self.0.0[Self::idx(pos)] & Self::bit(pos) != 0
    }
    pub fn set(&mut self, pos: Pos) {
        self.0.0[Self::idx(pos)] |= Self::bit(pos);
    }
    pub fn unset(&mut self, pos: Pos) {
        self.0.0[Self::idx(pos)] &= !Self::bit(pos);
    }
}
impl FromIterator<Pos> for PosSet {
    fn from_iter<T: IntoIterator<Item = Pos>>(iter: T) -> Self {
        let mut output = Self::new();
        iter.into_iter().for_each(|p| output.set(p));
        output
    }
}
impl Debug for PosSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut set = f.debug_set();
        for y in PosScalar::MIN.. {
            for x in PosScalar::MIN.. {
                let pos = Pos { x, y };
                if self.has(pos) {
                    set.entry(&pos);
                }
            }
        }
        set.finish()
    }
}

#[derive(Clone, PartialEq)]
struct ElfMap<const DO_STAGNATION: bool> {
    elves: Vec<Pos>,
    elves_positions: PosSet,
    stagnant: PosSet,
    proposed: HashMap<Pos, ProposalList>,
    bounding: (Pos, Pos),
}
impl<const DO_STAGNATION: bool> ElfMap<DO_STAGNATION> {
    pub fn bounding_box(&self) -> (Pos, Pos) {
        self.bounding
    }

    #[inline(never)]
    pub fn round_first_half(&mut self, round_num: usize) {
        let ElfMap {
            elves,
            elves_positions,
            stagnant,
            proposed,
            bounding,
        } = self;
        *bounding = (Pos::MAX, Pos::MIN);
        elves.iter().enumerate().for_each(|(i, pos)| {
            bounding.0 = bounding.0.min(pos);
            bounding.1 = bounding.1.max(pos);

            if DO_STAGNATION && stagnant.has(*pos) { return; }

            let has_n = elves_positions.has(Pos::N.add(pos));
            let has_e = elves_positions.has(Pos::E.add(pos));
            let has_s = elves_positions.has(Pos::S.add(pos));
            let has_w = elves_positions.has(Pos::W.add(pos));
            let has_ne = elves_positions.has(Pos::NE.add(pos));
            let has_nw = elves_positions.has(Pos::NW.add(pos));
            let has_se = elves_positions.has(Pos::SE.add(pos));
            let has_sw = elves_positions.has(Pos::SW.add(pos));

            if !has_n && !has_e && !has_s && !has_w && !has_ne && !has_nw && !has_se && !has_sw {
                if DO_STAGNATION {
                    stagnant.set(*pos);
                }
                return
            }

            let rules = [
                (has_n || has_ne || has_nw, Pos::N.add(pos)),
                (has_s || has_se || has_sw, Pos::S.add(pos)),
                (has_w || has_nw || has_sw, Pos::W.add(pos)),
                (has_e || has_ne || has_se, Pos::E.add(pos)),
            ];
            for r in 0..rules.len() {
                let rule_idx = (round_num + r) & 3;
                if !rules[rule_idx].0 {
                    proposed.entry(rules[rule_idx].1).or_default().add(i);
                    bounding.0 = bounding.0.min(&rules[rule_idx].1);
                    bounding.1 = bounding.1.max(&rules[rule_idx].1);
                    break;
                }
            }
        });
    }
    #[inline(never)]
    pub fn round_second_half(&mut self) -> bool {
        #[inline(never)]
        fn handle_proposal<const DO_STAGNATION: bool>(
            elves: &mut Vec<Pos>,
            elves_positions: &mut PosSet,
            stagnant: &mut PosSet,
            new_pos: Pos,
            idx: usize
        ) -> bool {
            let old = elves[idx];
            elves[idx] = new_pos;
            elves_positions.unset(old);
            elves_positions.set(new_pos);

            if DO_STAGNATION {
                let offsets = [Pos::N, Pos::E, Pos::S, Pos::W, Pos::NE, Pos::NW, Pos::SE, Pos::SW];
                for offset in offsets {
                    let pos_to_recheck = new_pos.add(&offset);
                    stagnant.unset(pos_to_recheck);
                }
            }
            true
        }

        let ElfMap {
            elves,
            elves_positions,
            stagnant,
            proposed,
            ..
        } = self;
        let mut something_moved = false;
        proposed.drain().for_each(|(new_pos, proposal_list)| {
            something_moved |= handle_proposal::<DO_STAGNATION>(
                elves,
                elves_positions,
                stagnant,
                new_pos,
                if let ProposalList::One(idx) = proposal_list { idx } else { return },
            );
        });
        something_moved
    }
    pub fn count_empty_in(&self, inclusive_box: (Pos, Pos)) -> usize {
        let mut count = 0;
        for y in inclusive_box.0.y..=inclusive_box.1.y {
            for x in inclusive_box.0.x..=inclusive_box.1.x {
                if !self.elves_positions.has(Pos { x, y }) {
                    count += 1;
                }
            }
        }
        count
    }
}
impl<const DO_STAGNATION: bool> FromStr for ElfMap<DO_STAGNATION> {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elves: Vec<_> = s.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .enumerate()
            .flat_map(|(y, line)| {
                line
                    .chars()
                    .enumerate()
                    .filter_map(move |(x, c)| {
                        (c == '#').then(|| Pos::new(x as PosScalar, y as PosScalar))
                    })
            })
            .collect();
        Ok(Self {
            elves_positions: elves.iter().copied().collect(),
            elves,
            stagnant: PosSet::new(),
            proposed: HashMap::new(),
            bounding: (Pos::new(0, 0), Pos::new(0, 0)),
        })
    }
}
impl<const DO_STAGNATION: bool> Debug for ElfMap<DO_STAGNATION> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self.bounding_box();
        for y in min.y-1..=max.y+1 {
            for x in min.x-1..=max.x+1 {
                let pos = Pos::new(x, y);
                if self.elves_positions.has(pos) {
                    if self.stagnant.has(pos) {
                        write!(f, "%")
                    } else {
                        write!(f, "#")
                    }
                } else {
                    match self.proposed.get(&pos) {
                        Some(ProposalList::One(_)) => write!(f, "*"),
                        Some(ProposalList::Many) => write!(f, "X"),
                        _ => write!(f, "."),
                    }
                }?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[inline(never)]
fn parse_input<const DO_STAGNATION: bool>(input: &str) -> ElfMap<DO_STAGNATION> {
    input.parse().unwrap()
}
