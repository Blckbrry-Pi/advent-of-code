use std::sync::mpsc::TryRecvError;

use crate::instruction::Instruction;

#[derive(Debug, Clone)]
pub struct Machine {
    pub pc: usize,
    pub input: Vec<isize>,
    pub output: Vec<isize>,
    pub halt: bool,
    pub offset: isize,
    pub debug: bool,
}


impl Machine {
    pub fn new(input: Vec<isize>) -> Self {
        Self {
            pc: 0,
            input,
            output: vec![],
            halt: false,
            offset: 0,
            debug: false,
        }
    }
    pub fn input(&mut self) -> isize {
        if self.input.is_empty() {
            -1
        } else {
            self.input.remove(0)
        }
    }
    pub fn output(&mut self, val: isize) {
        self.output.push(val);
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
                                sender.send(Err(e));
                            },
                        }
                    }
                });

                let mut input_str = String::new();

                while let Some(instruction) = Instruction::parse(&self, &data) {
                    instruction.exec(self, data);
                    
                    while let Some(&ch) = self.output.first() {
                        self.output.remove(0);
                        output.write(&[ch as u8])?;
                        output.flush()?;
                    }
        
                    // Read in
                    let mut some_read = false;
                    loop {
                        match receiver.try_recv() {
                            Ok(Ok(byte)) => {
                                self.input.push(byte as isize);
                                input_str.push(byte as char);
                                // let bytes = self.input.
                                // some_read = true;
                                // println!("{:?}", input_str);
                            },
                            Ok(Err(e)) => {
                                reader_handle.join().unwrap();
                                return Err(e);
                            },
                            Err(TryRecvError::Empty) => {
                                // if some_read { self.input.push(b' ' as isize); }
                                break
                            },
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
        // while let Some(instruction) = Instruction::parse(&self, &data) {
        //     instruction.exec(self, data);
        //     println!("Executing: {instruction:?}");
        //     // 
        //     if let Some(&ch) = self.output.first() {
        //         self.output.remove(0);
        //         output.write(&[ch as u8])?;
        //         output.flush()?;
        //     }

        //     // Read in
        //     use std::io::Read;
        //     let mut buffer = [0];
        //     if input.has_data_left()? {
        //         assert_eq!(input.read(&mut buffer)?, 1);
        //         self.input.push(buffer[0] as isize);
        //     }
        // }

        // Ok(())
    }
}
