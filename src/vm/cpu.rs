use crate::vm::instruction::*;
use crate::vm::frame::*;
use std::io::{stdin, stdout, Write};

pub struct CPU {
    program: Vec<u32>,
    current_address: usize,

    pub stack: Vec<i16>,
    pub call_stack: Vec<Frame>,

    zero_flag: bool,
    sign_flag: bool,
}

impl CPU {
    pub fn new(program: Vec<u32>) -> CPU {
        CPU {
            program,
            current_address: 0,
            stack: Vec::new(),
            call_stack: vec![Frame::new(usize::MAX)],
            sign_flag: false,
            zero_flag: false,
        }
    }

    pub fn run(&mut self) -> Result<i16, String> {
        loop {
            if let Some(e) = self.execute_instruction() {
                if e == "jumped" {
                    continue;
                }
                else if e == "halt" {
                    if self.stack.len() > 0 {
                        return Ok(self.stack.pop().unwrap())
                    } else {
                        return Ok(0)
                    }
                }
                return Err(format!("{} at instruction {} ({:?})", e, self.current_address, Opcode::decode(self.program[self.current_address])));
            }
            self.current_address += 1;
        }
    }

    pub fn execute_instruction(&mut self) -> Option<String> {
        let (opcode, operand1, operand2) = Opcode::decode(self.program[self.current_address]);
        match opcode {
            Opcode::ILG => return Some("Illegal character".into()),
            Opcode::HALT => return Some("halt".into()),
            Opcode::LEN => self.stack.push(self.stack.len() as i16),

            Opcode::POP => {
                match self.stack.pop() {
                    Some(_) => (),
                    None => return Some("no character to pop".into()),
                };
            },
            Opcode::PUSH => self.stack.push(operand1),
            Opcode::DUP => {
                let temp = match self.stack.pop() {
                    Some(val) => val,
                    None => return Some("no character to pop".into())
                };

                self.stack.push(temp.clone());
                self.stack.push(temp.clone());
            },

            Opcode::ADD => {
                let n1 = match self.stack.pop() {
                    Some(n) => n,
                    None => return Some("no character to pop".into()),
                };
                let n2 = match self.stack.pop() {
                    Some(n) => n,
                    None => return Some("no character to pop".into()),
                };

                self.stack.push(n2 + n1);
                self.set_flags();
            },
            Opcode::SUB => {
                let n1 = match self.stack.pop() {
                    Some(n) => n,
                    None => return Some("no character to pop".into()),
                };
                let n2 = match self.stack.pop() {
                    Some(n) => n,
                    None => return Some("no character to pop".into()),
                };

                self.stack.push(n2 - n1);
                self.set_flags();
            },

            Opcode::MUL => {
                let n1 = match self.stack.pop() {
                    Some(n) => n,
                    None => return Some("no character to pop".into()),
                };
                let n2 = match self.stack.pop() {
                    Some(n) => n,
                    None => return Some("no character to pop".into()),
                };

                self.stack.push(n2 * n1);
                self.set_flags();
            },
            Opcode::DIV => {
                let n1 = match self.stack.pop() {
                    Some(n) => n,
                    None => return Some("no character to pop".into()),
                };
                let n2 = match self.stack.pop() {
                    Some(n) => n,
                    None => return Some("no character to pop".into()),
                };

                self.stack.push(n2 / n1);
                self.set_flags();
            },
            Opcode::MOD => {
                let n1 = match self.stack.pop() {
                    Some(n) => n,
                    None => return Some("no character to pop".into()),
                };
                let n2 = match self.stack.pop() {
                    Some(n) => n,
                    None => return Some("no character to pop".into()),
                };

                self.stack.push(n2 % n1);
                self.set_flags();
            },

            Opcode::CMP => {
                let n1 = match self.stack.pop() {
                    Some(n) => n,
                    None => return Some("no character to pop".into()),
                };
                let n2 = match self.stack.pop() {
                    Some(n) => n,
                    None => return Some("no character to pop".into()),
                };

                self.stack.push(n2 - n1);
                self.set_flags();
            },
            Opcode::JMP => {
                if operand2 == 0 {
                    self.current_address = operand1 as usize;
                } else {
                    self.current_address += operand1 as usize;
                }

                return Some("jumped".into());
            },

            Opcode::JE => {
                if self.zero_flag {
                    if operand2 == 2 {
                        self.current_address -= operand1 as usize;
                    }
                    else if operand2 == 1 {
                        self.current_address += operand1 as usize;
                    }
                    else {
                        self.current_address = operand1 as usize;
                    }
                    return Some("jumped".into());
                }
            },
            Opcode::JNE => {
                if !self.zero_flag {
                    if operand2 == 2 {
                        self.current_address -= operand1 as usize;
                    }
                    else if operand2 == 1 {
                        self.current_address += operand1 as usize;
                    }
                    else {
                        self.current_address = operand1 as usize;
                    }
                    return Some("jumped".into());
                }
            },

            Opcode::JG => {
                if self.sign_flag {
                    if operand2 == 2 {
                        self.current_address -= operand1 as usize;
                    }
                    else if operand2 == 1 {
                        self.current_address += operand1 as usize;
                    }
                    else {
                        self.current_address = operand1 as usize;
                    }
                    return Some("jumped".into());
                }
            },
            Opcode::JL => {
                if !self.sign_flag {
                    if operand2 == 2 {
                        self.current_address -= operand1 as usize;
                    }
                    else if operand2 == 1 {
                        self.current_address += operand1 as usize;
                    }
                    else {
                        self.current_address = operand1 as usize;
                    }
                    return Some("jumped".into());
                }
            },

            Opcode::JGE => {
                if self.sign_flag || self.zero_flag {
                    if operand2 == 2 {
                        self.current_address -= operand1 as usize;
                    }
                    else if operand2 == 1 {
                        self.current_address += operand1 as usize;
                    }
                    else {
                        self.current_address = operand1 as usize;
                    }
                    return Some("jumped".into());
                }
            },
            Opcode::JLE => {
                if !self.sign_flag || self.zero_flag {
                    if operand2 == 2 {
                        self.current_address -= operand1 as usize;
                    }
                    else if operand2 == 1 {
                        self.current_address += operand1 as usize;
                    }
                    else {
                        self.current_address += operand1 as usize;
                    }
                   
                    return Some("jumped".into());
                }
            },

            Opcode::STDIN => {
                let mut c = String::new();
                stdin().read_line(&mut c);
                self.stack.push(match c.trim().parse::<i16>() {
                    Ok(val) => val,
                    Err(e) => return Some(format!("Couldn't parse string, Err: {}", e)),
                });
            },

            Opcode::STDOUT => {
                let num1 = match self.stack.pop() {
                    Some(n) => n,
                    None => return Some("no character to pop".into()),
                };

                match operand2 {
                    3 => {
                        print!("{}", num1 as u8 as char);
                        let _ = stdout().flush();
                    }
                    2 => {
                        print!("{}", num1 as u8);
                        let _ = stdout().flush();
                    }
                    1 => println!("{}", num1 as u8 as char),
                    _ => println!("{}", num1),
                }

                self.stack.push(num1);
            },

            Opcode::LOAD => {
                self.stack.push(match self.call_stack.last().unwrap().load(&operand1) {
                    Some(n) => *n,
                    None => return Some(format!("{} is not a variable", operand1))
                });
            },

            Opcode::STORE => {
                let num1 = match self.stack.pop() {
                    Some(n) => n,
                    None => return Some("no character to pop".into()),
                };

                let mut call = self.call_stack.pop().unwrap();
                call.store(operand1, num1);
                self.call_stack.push(call);
            },

            Opcode::CALL => {
                self.call_stack.push(Frame::new(self.current_address));

                if operand2 == 2 {
                    self.current_address -= operand1 as usize;
                }
                else if operand2 == 1 {
                    self.current_address += operand1 as usize;
                }
                else {
                    self.current_address = operand1 as usize;
                }

                return Some("jumped".into());
            },

            Opcode::RETURN => {
                self.current_address = self.call_stack.pop().unwrap().return_address;
            }
        }

        None
    }

    pub fn set_flags(&mut self) {
        let result = self.stack.pop().unwrap();

        match result {
            0 => self.zero_flag = true,
            _ => {
                self.zero_flag = false;
                match result > 0 {
                    true => self.sign_flag = true,
                    false => self.sign_flag = false,
                }
            }
        }

        self.stack.push(result);
    }
}
