extern crate stack_based_virtual_machine;
use stack_based_virtual_machine::vm::instruction::*;
use stack_based_virtual_machine::vm::cpu::*;
use stack_based_virtual_machine::parser::lexer::*;
use stack_based_virtual_machine::parser::assembler::*;
use stack_based_virtual_machine::parser::reader::*;

use std::fs::read_to_string;

pub fn main() {
    let file = read_to_string("nar_files/minus.nar").unwrap() as String;
    let mut lexer = Lexer::new(file);
    lexer.lex();

    let mut assembler = Assembler::new(lexer.tokens, "binaries/minus.bin");
    assembler.assemble();
    assembler.write();

    let program = Reader::read("binaries/minus.bin");
    let mut cpu = CPU::new(program);
    cpu.run();
}