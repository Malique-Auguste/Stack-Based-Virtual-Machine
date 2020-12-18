extern crate stack_based_virtual_machine;
use stack_based_virtual_machine::cpu::*;
use stack_based_virtual_machine::instruction::*;

fn main() {
    //Subtracts the user input from 120 until the difference = 0
    let program = vec![
        Opcode::encode(Opcode::PUSH, 120, 0),
        Opcode::encode(Opcode::STDOUT, 0, 0),
        Opcode::encode(Opcode::PUSH, 62, 0), //62 is utf8 for right arrow
        Opcode::encode(Opcode::STDOUT, 0, 3),
        Opcode::encode(Opcode::POP, 0, 0),
        Opcode::encode(Opcode::PUSH, 32, 0), //32 is utf8 for right arrow
        Opcode::encode(Opcode::STDOUT, 0, 3),
        Opcode::encode(Opcode::POP, 0, 0),
        Opcode::encode(Opcode::STDIN, 0, 0),
        Opcode::encode(Opcode::CMP, 0, 0),
        Opcode::encode(Opcode::JG, 1, 0),
        Opcode::encode(Opcode::HALT, 0, 0),
    ];
    let mut cpu = CPU::new(program);

    cpu.run();
}
