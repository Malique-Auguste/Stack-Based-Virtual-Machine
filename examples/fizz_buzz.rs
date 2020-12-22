extern crate stack_based_virtual_machine;
use stack_based_virtual_machine::vm::instruction::*;
use stack_based_virtual_machine::vm::cpu::*;
use stack_based_virtual_machine::parser::lexer::*;
use stack_based_virtual_machine::parser::assembler::*;
use stack_based_virtual_machine::parser::reader::*;

use std::fs::read_to_string;

fn main() {
    let file = read_to_string("nar_files/fizz_buzz.nar").unwrap() as String;
    let mut lexer = Lexer::new(file);
    lexer.lex();

    let mut assembler = Assembler::new(lexer.tokens, "binaries/fizz_buzz.bin");
    assembler.assemble();
    assembler.write();

    let program: Vec<u32> = Reader::read("binaries/fizz_buzz.bin");
    program.iter().for_each(|x|
        println!("{:#?}", Opcode::decode(x.clone()))
    );

    let mut cpu = CPU::new(program);
    println!("{:?}", cpu.run());
}