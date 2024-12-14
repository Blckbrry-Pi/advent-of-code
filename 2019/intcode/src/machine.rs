use std::sync::mpsc::TryRecvError;

use crate::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct Machine {
    pub pc: usize,
    pub input: (Vec<isize>, usize),
    pub output: Vec<isize>,
    pub halt: bool,
    pub offset: isize,
    pub debug: bool,
}


impl Machine {
    pub fn new(input: Vec<isize>) -> Self {
        Self {
            pc: 0,
            input: (input, 0),
            output: vec![],
            halt: false,
            offset: 0,
            debug: false,
        }
    }
    pub fn new_ascii(input: &str) -> Self {
        Self::new(
            input.chars().map(|c| c as u8 as isize).collect()
        )
    }
    pub fn input_is_empty(&self) -> bool {
        self.input.1 >= self.input.0.len()
    }
    pub fn input(&mut self) -> isize {
        if self.input.1 >= self.input.0.len() {
            -1
        } else {
            let output = self.input.0[self.input.1];
            self.input.1 += 1;
            if self.input.1 >= 100 {
                self.input.0.drain(0..self.input.1).count();
                self.input.1 = 0;
            }
            output
        }
    }
    pub fn output(&mut self, val: isize) {
        self.output.push(val);
    }

    pub fn decode(&self, data: &[isize]) -> Option<Instruction> {
        Instruction::parse(self, data)
    }
    pub fn exec(&mut self, instruction: Instruction, data: &mut [isize]) {
        instruction.exec(self, data);
    }

    pub fn step(&mut self, data: &mut [isize]) -> Result<(), ()> {
        if self.halt { return Err(()) }

        if let Some(instruction) = self.decode(data) {
            self.exec(instruction, data);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn run(
        &mut self,
        data: &mut [isize],
        mut input: impl std::io::Read + Send,
        mut output: impl std::io::Write,
    ) -> std::io::Result<()> {
        std::thread::scope(
            move |s| {
                let (sender, receiver) = std::sync::mpsc::channel();
                let reader_handle = s.spawn(move || {
                    loop {
                        let mut buffer = [0];
                        match input.read(&mut buffer) {
                            Ok(1) => match sender.send(Ok(buffer[0])) {
                                Ok(()) => (),
                                Err(_) => break,
                            },
                            Ok(0) => (),
                            Ok(_) => unreachable!("Read more than 1 byte into a 1 byte buffer"),
                            Err(e) => {
                                sender.send(Err(e)).unwrap();
                            },
                        }
                    }
                });

                while let Some(instruction) = Instruction::parse(&self, &data) {
                    instruction.exec(self, data);
                    
                    while let Some(&ch) = self.output.first() {
                        self.output.remove(0);
                        output.write(&[ch as u8])?;
                        output.flush()?;
                    }
        
                    // Read in
                    loop {
                        match receiver.try_recv() {
                            Ok(Ok(byte)) => self.input.0.push(byte as isize),
                            Ok(Err(e)) => {
                                reader_handle.join().unwrap();
                                return Err(e);
                            },
                            Err(TryRecvError::Empty) => break,
                            Err(TryRecvError::Disconnected) => {
                                reader_handle.join().unwrap();
                                return Err(std::io::Error::last_os_error());
                            }
                        }
                    }
                }

                Ok(())
            }
        )
    }
}
