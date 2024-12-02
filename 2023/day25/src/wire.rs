use std::hash::Hash;


#[derive(Debug, Clone, Copy)]
pub struct Wire {
    a: &'static str,
    b: &'static str,
}

impl Wire {
    pub fn new(a: &'static str, b: &'static str) -> Self {
        Wire { a, b }
    }
    pub fn a(&self) -> &'static str {
        &self.a
    }
    pub fn b(&self) -> &'static str {
        &self.b
    }
    pub fn contains(&self, name: &str) -> bool {
        self.a == name || self.b == name
    }

    pub fn other(&self, name: &str) -> Option<&'static str> {
        if self.a == name {
            Some(&self.b)
        } else if self.b == name {
            Some(&self.a)
        } else {
            None
        }
    }
}

impl PartialEq for Wire {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}
impl Eq for Wire {}

impl Hash for Wire {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (&self.a).min(&self.b).hash(state);
        (&self.a).max(&self.b).hash(state);
    }
}
