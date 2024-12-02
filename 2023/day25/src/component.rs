#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Component {
    name: &'static str,
}

impl Component {
    pub fn new(name: &'static str) -> Self {
        Component { name }
    }

    pub fn name(&self) -> &'static str {
        &self.name
    }
}
