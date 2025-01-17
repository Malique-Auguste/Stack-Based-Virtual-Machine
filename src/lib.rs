pub mod vm;
pub mod parser;

use vm::cpu::*;
use vm::frame::*;
use vm::instruction::*;

use parser::assembler::*;
use parser::reader::*;
use parser::lexer::*;
use parser::tokens::*;


#[cfg(test)]
mod test_instruction {
    //use crate::instruction::*;
    use super::*;

    #[test]
    fn operation_decode_encode() {
        let opcode: Opcode = 1.into();
        let operand1: i16 = 1;
        let operand2: i8 = 2;

        let instruction: u32 = Opcode::encode(opcode.clone(), 1, 2);
        assert_eq!((opcode, operand1, operand2), Opcode::decode(instruction));
    }

    #[test]
    fn instruction_byte() {
        let instruction = 0xb32 as u32;
        println!("{}", instruction);
        assert_eq!(instruction, Opcode::byte_array_to_instruction(Opcode::instruction_to_byte_array(instruction)));
    }
}

#[cfg(test)]
mod test_cpu {
    use super::*;

    #[test]
    fn arithmetic() {
        let program = vec![
            Opcode::encode(Opcode::PUSH, 2, 0),
            Opcode::encode(Opcode::PUSH, 5, 0),
            Opcode::encode(Opcode::ADD, 0, 0),
            Opcode::encode(Opcode::PUSH, 3, 0),
            Opcode::encode(Opcode::SUB, 0, 0),
            Opcode::encode(Opcode::PUSH, 4, 0),
            Opcode::encode(Opcode::MUL, 0, 0),
            Opcode::encode(Opcode::PUSH, 2, 0),
            Opcode::encode(Opcode::DIV, 0, 0),
            Opcode::encode(Opcode::PUSH, 2, 0),
            Opcode::encode(Opcode::POP, 0, 0),
            Opcode::encode(Opcode::PUSH, 5, 0),
            Opcode::encode(Opcode::MOD, 0, 0),
            Opcode::encode(Opcode::HALT, 0, 0),
        ];
        let mut cpu = CPU::new(program);

        assert_eq!(3, cpu.run().unwrap());
    }

    #[test]
    fn logic() {
        let program = vec![
            Opcode::encode(Opcode::PUSH, 10, 0),
            Opcode::encode(Opcode::PUSH, 2, 0),
            Opcode::encode(Opcode::CMP, 0, 0),
            Opcode::encode(Opcode::JNE, 1, 0),
            Opcode::encode(Opcode::HALT, 0, 0),
        ];
        let mut cpu = CPU::new(program);

        assert_eq!(0, cpu.run().unwrap());
    }

    #[test]
    fn functions() {
        let program = vec![
            Opcode::encode(Opcode::PUSH, 10, 0),
            Opcode::encode(Opcode::PUSH, 5, 0),
            Opcode::encode(Opcode::CALL, 3, 1),
            Opcode::encode(Opcode::STDOUT, 0, 0),
            Opcode::encode(Opcode::HALT, 1, 0),


            //function that returns the largest number
            Opcode::encode(Opcode::STORE, 0, 0),
            Opcode::encode(Opcode::STORE, 1, 0),
            Opcode::encode(Opcode::LOAD, 0, 0),
            Opcode::encode(Opcode::LOAD, 1, 0),
            Opcode::encode(Opcode::CMP, 0, 0),
            Opcode::encode(Opcode::JL, 3, 1),
            Opcode::encode(Opcode::LOAD, 0, 0),
            Opcode::encode(Opcode::RETURN, 1, 0),

            Opcode::encode(Opcode::LOAD, 1, 0),
            Opcode::encode(Opcode::RETURN, 1, 0),
        ];

        let mut cpu = CPU::new(program);
        assert_eq!(10, cpu.run().unwrap());
    }
}

#[cfg(test)]
mod test_lexer {
    use super::*;

    #[test]
    fn lex_single_line() {
        let mut lexer = Lexer::new("PUSH 12");
        lexer.lex();
        assert_eq!(vec![
                Token::new(TokenType::Str("PUSH".into()), 1),
                Token::new(TokenType::Num(12), 1)
            ], 
            lexer.tokens);
    }

    #[test]
    fn lex_multiple_lines() {
        let mut lexer = Lexer::new("Start:\n PUSH 12\nPUSH 15");
        lexer.lex();
        assert_eq!(vec![
                Token::new(TokenType::Identifier("Start".into()), 1),
                Token::new(TokenType::Str("PUSH".into()), 2),
                Token::new(TokenType::Num(12), 2),
                Token::new(TokenType::Str("PUSH".into()), 3),
                Token::new(TokenType::Num(15), 3)
            ], 
            lexer.tokens);
    }

}

#[cfg(test)]
mod test_parsing {
    use super::*;

    #[test]
    fn assemble() {
        let tokens = vec![Token::new(TokenType::Identifier("Start".into()), 1),
                Token::new(TokenType::Str("PUSH".into()), 2),
                Token::new(TokenType::Num(12), 2),
                Token::new(TokenType::Num(0), 2),
                Token::new(TokenType::Str("PUSH".into()), 3),
                Token::new(TokenType::Num(15), 3),
                Token::new(TokenType::Num(0), 2),];

        let code = vec![Opcode::encode(Opcode::PUSH, 12, 0),
                Opcode::encode(Opcode::PUSH, 15, 0)];
        let mut assembler = Assembler::new(tokens, "");
        assembler.assemble();
        
        assert_eq!(code, assembler.output);
    }

    #[test]
    fn write_to_file() {
        let tokens = vec![Token::new(TokenType::Identifier("Start".into()), 1),
                Token::new(TokenType::Str("PUSH".into()), 2),
                Token::new(TokenType::Num(12), 2),
                Token::new(TokenType::Num(0), 2),
                Token::new(TokenType::Str("POP".into()), 3),
                Token::new(TokenType::Num(0), 3),
                Token::new(TokenType::Num(0), 3),
                Token::new(TokenType::Str("HALT".into()), 4),
                Token::new(TokenType::Num(0), 4),
                Token::new(TokenType::Num(0), 4),];

        let mut assembler = Assembler::new(tokens, "binaries/test1.bin");
        assembler.assemble();
        assembler.write();
    }

    #[test]
    fn read_from_file() {
        let instructions = vec![Opcode::encode(Opcode::PUSH, 12, 0),
                            Opcode::encode(Opcode::POP, 0, 0),
                            Opcode::encode(Opcode::HALT, 0, 0)];
        assert_eq!(instructions, Reader::read("binaries/test1.bin"));
    }

    #[test]
    fn execute_file() {
        let instructions = Reader::read("binaries/test1.bin");

        let mut cpu = CPU::new(instructions);
        cpu.run();
    }
}