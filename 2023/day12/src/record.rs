use std::{collections::HashMap, fmt::Debug, str::FromStr, usize};

#[derive(Clone)]
pub struct Record {
    conditions: Vec<Condition>,
    groups: Vec<Group>,
}

const UNFOLD_COUNT: usize = 5;

impl Record {
    pub fn unfold(&self) -> Record {
        let conditions: Vec<_> = std::iter::repeat(&self.conditions)
            .take(UNFOLD_COUNT)
            .map(std::ops::Deref::deref)
            .intersperse_with(|| &[Condition::Unknown])
            .flatten()
            .copied()
            .collect();

        let groups: Vec<_> = std::iter::repeat(&self.groups)
            .take(UNFOLD_COUNT)
            .flatten()
            .copied()
            .collect();

        Self {
            conditions,
            groups,
        }
    }

    pub fn build_valid_pos_cache(&self) -> Vec<Vec<bool>> {
        let mut cache = vec![vec![false; self.conditions.len()]; self.groups.len()];

        for group_idx in 0..self.groups.len() {
            for cond_idx in 0..self.conditions.len() {
                cache[group_idx][cond_idx] = self.groups[group_idx].can_be_at(cond_idx, &self.conditions);
            }
        }

        cache
    }

    pub fn build_next_damaged_cache(&self) -> Vec<usize> {
        let mut cache = vec![usize::MAX; self.conditions.len()];

        for cond_idx in 0..self.conditions.len() {
            for i in cond_idx..self.conditions.len() {
                if self.conditions[i] == Condition::Damaged {
                    cache[cond_idx] = i;
                    break;
                }
            }
        }

        cache
    }

    pub fn count_valid_positions(&self, state: CountState, validity: &[Vec<bool>], next_damaged: &[usize], cache: &mut HashMap<CountState, usize>) -> usize {
        // println!("State: {state:?}");
        // println!("Cache: {cache:?}");
        // println!("State: {state:?}");
        let CountState { group_idx, cond_idx } = state;

        // Base Case
        if group_idx == self.groups.len() {
            if cond_idx < self.conditions.len() && next_damaged[cond_idx] < self.conditions.len() {
                return 0;
            } else {
                return 1;
            }
        }

        // Memoization
        if let Some(&count) = cache.get(&state) {
            return count;
        }

        // New total
        let mut count = 0;

        // Next group to place
        let group = self.groups[group_idx];
        let valid_group_positions = (cond_idx..self.conditions.len())
            .filter(|&idx| validity[group_idx][idx] && idx <= next_damaged[cond_idx]);

        // For each position
        for test_cond_idx in valid_group_positions {
            let new_state = CountState {
                group_idx: group_idx + 1,
                cond_idx: test_cond_idx + group.size + 1,
            };

            let valid_positions = self.count_valid_positions(new_state, validity, next_damaged, cache);
            count += valid_positions;
            cache.insert(new_state, valid_positions);
        }

        count
    }

    pub fn count_valid_positions_base(&self) -> usize {
        let validity = self.build_valid_pos_cache();
        let next_damaged = self.build_next_damaged_cache();
        self.count_valid_positions(CountState { group_idx: 0, cond_idx: 0 }, &validity, &next_damaged, &mut HashMap::new())
    }
}

impl Debug for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for condition in &self.conditions {
            write!(f, "{:?}", condition)?;
        }

        write!(f, " ")?;
        
        for (i, group) in self.groups.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, "{:?}", group)?;
        }

        Ok(())
    }
}

impl FromStr for Record {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((conditions, groups)) = s.split_once(' ') else {
            return Err("missing space to separate conditions and groups".to_string());
        };

        let conditions = conditions.char_indices()
            .map(
                |(idx, c)| conditions[idx..idx+c.len_utf8()]
                    .parse()
                    .map_err(|_| format!("invalid condition: {}", c))
            )
            .collect::<Result<Vec<Condition>, _>>()?;

        let groups = groups.split(',')
            .map(|group| group.trim())
            .map(|group| group.parse().map_err(|_| format!("invalid group: {}", group)))
            .collect::<Result<Vec<Group>, _>>()?;

        Ok(Self { conditions, groups })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CountState {
    group_idx: usize,
    cond_idx: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    Damaged,
    Operational,
    Unknown,
}
impl Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::Damaged => write!(f, "#"),
            Condition::Operational => write!(f, "."),
            Condition::Unknown => write!(f, "?"),
        }
    }
}
impl FromStr for Condition {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Condition::Damaged),
            "." => Ok(Condition::Operational),
            "?" => Ok(Condition::Unknown),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Group {
    size: usize,
}
impl Group {
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    pub fn can_be_at(&self, idx: usize, conditions: &[Condition]) -> bool {
        if conditions.len() < idx + self.size {
            return false;
        }

        if idx != 0 && conditions[idx - 1] == Condition::Damaged {
            let previous = conditions[idx - 1];
            if previous == Condition::Damaged {
                return false;
            }
        }
        if idx + self.size < conditions.len() {
            let next = conditions[idx + self.size];
            if next == Condition::Damaged {
                return false;
            }
        }

        for i in idx..idx + self.size {
            if conditions[i] == Condition::Operational {
                return false;
            }
        }

        true
    }
}

impl Debug for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.size)
    }
}
impl FromStr for Group {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<usize>()
            .map_err(|e| e.to_string())
            .map(Group::new)
    }
}
