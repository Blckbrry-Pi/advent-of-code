use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Count {
    counts: HashMap<i32, i32>,
}

impl Count {
    pub fn from_list(list: &[i32]) -> Self {
        let mut counts = HashMap::new();

        for &v in list {
            *counts.entry(v).or_insert(0) += 1;
        }

        Count { counts }
    }

    pub fn count(&self, value: i32) -> i32 {
        *self.counts.get(&value).unwrap_or(&0)
    }
}
