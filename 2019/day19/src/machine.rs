#[derive(Clone)]
pub struct Machine {
    pub pc: usize,
    pub input: Vec<isize>,
    pub output: Vec<isize>,
    pub halt: bool,
    pub offset: isize,
}


impl Machine {
    pub fn new(input: Vec<isize>) -> Self {
        Self {
            pc: 0,
            input,
            output: vec![],
            halt: false,
            offset: 0,
        }
    }
    pub fn input(&mut self) -> isize {
        if self.input.is_empty() {
            0
        } else {
            self.input.remove(0)
        }
    }
    pub fn output(&mut self, val: isize) {
        self.output.push(val);
    }
}