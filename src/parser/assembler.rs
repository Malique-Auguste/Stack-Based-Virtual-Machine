use crate::parser::tokens::*;
use crate::vm::instruction::Opcode;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub struct Assembler {
    source: Vec<Token>,
    pub output: Vec<u32>,

    file_path: String
}

impl Assembler {
    pub fn new<S: Into<String>>(source: Vec<Token>, file_path: S) -> Assembler {
        Assembler { source, output: Vec::new(), file_path: file_path.into() }
    }

    pub fn assemble(&mut self) {
        let mut identifiers: HashMap<String, usize> = HashMap::new();

        let mut index = 0;
        loop {
            if let TokenType::Identifier(i) = self.source[index].token_type.clone() {
                identifiers.insert(i, index / 3);
                self.source.remove(index);
                continue;
            }
            index += 1;
            if index >= self.source.len() {
                break;
            }
        }

        loop {
            let current_token = self.source.remove(0);
            match current_token.token_type {
                TokenType::Identifier(_) => unreachable!(),
                TokenType::Str(val) => {
                    let opcode = Opcode::from(&val);
                    if opcode == Opcode::ILG {
                        panic!("Illegal opcode encountered: {}", val);
                    }

                    let operand2 = match self.source.remove(1).token_type {
                        TokenType::Identifier(_) => panic!("Identifier encountered as an operand: {:?}", self.source[1]),
                        TokenType::Str(s) => panic!("String encountered as 2nd operand: {:?}", self.source[1]),
                        TokenType::Num(n) => n
                    };

                    match self.source.remove(0).token_type {
                        TokenType::Identifier(_) => unreachable!(),
                        TokenType::Str(s) => {
                            match opcode {
                                Opcode::JMP | Opcode::JE | Opcode::JNE| Opcode::JL | Opcode::JG | Opcode::JLE | Opcode::JGE | Opcode::CALL => {
                                    let operand1 = match identifiers.get(&s) {
                                        Some(i) => i,
                                        None => panic!("Identifier doesn't exist")
                                    };

                                    self.output.push(Opcode::encode(opcode, *operand1 as i8, operand2))
                                },

                                Opcode::PUSH => {
                                    if s.len() > 1 { panic!("only a character can be pushed: {}", s); }
                                    self.output.push(Opcode::encode(opcode, s.chars().next().unwrap() as u8 as i8, operand2))
                                },

                                _ => panic!("Opcode cannot take string as an input: {:?}", opcode)
                            }
                        },
                        TokenType::Num(operand1) => self.output.push(Opcode::encode(opcode, operand1, operand2))
                    };

                },

                TokenType::Num(_) => panic!("Number encountered outside of being an operand or as an extra operand: {:?}", current_token)
            }

            if self.source.len() == 0 { break }
        }
    }

    pub fn write(&self) -> Result<(), std::io::Error> {
        let mut buffer =  File::create(self.file_path.clone()).unwrap();
        let mut file_data: Vec<u8> = Vec::new();

        self.output.iter().for_each(|x| {
            let arr = Opcode::instruction_to_byte_array(*x);
            file_data.extend_from_slice(&arr);
        });

        buffer.write_all(&file_data)
    }
}