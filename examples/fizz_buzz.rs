extern crate stack_based_virtual_machine;
use stack_based_virtual_machine::instruction::*;
use stack_based_virtual_machine::cpu::*;

fn main() {
    let program = vec![
        Opcode::encode(Opcode::PUSH, 0, 0),

        //Increments number
        Opcode::encode(Opcode::PUSH, 1, 0),
        Opcode::encode(Opcode::ADD, 0, 0),

        //Checks if it is divisible by 3
        Opcode::encode(Opcode::DUP, 0, 0),
        Opcode::encode(Opcode::PUSH, 3, 0),
        Opcode::encode(Opcode::MOD, 0, 0),
        Opcode::encode(Opcode::JE, 15, 1),

        //Checks if it is divisible by 5
        Opcode::encode(Opcode::POP, 0, 0),
        Opcode::encode(Opcode::DUP, 0, 0),
        Opcode::encode(Opcode::PUSH, 5, 0),
        Opcode::encode(Opcode::MOD, 0, 0),
        Opcode::encode(Opcode::JE, 14, 1),

        Opcode::encode(Opcode::POP, 0, 0),
        Opcode::encode(Opcode::STDOUT, 0, 0),
        Opcode::encode(Opcode::JMP, 15, 1),

        //Checks if it is divisible by 5
        Opcode::encode(Opcode::POP, 0, 0),
        Opcode::encode(Opcode::DUP, 0, 0),
        Opcode::encode(Opcode::PUSH, 5, 0),
        Opcode::encode(Opcode::MOD, 0, 0),
        Opcode::encode(Opcode::JE, 6, 1),
        Opcode::encode(Opcode::JNE, 8, 1),

        //prints f
        Opcode::encode(Opcode::PUSH, 102, 0),        //46 is the utf8 code for f
        Opcode::encode(Opcode::STDOUT, 0, 1),
        Opcode::encode(Opcode::POP, 0, 0),
        Opcode::encode(Opcode::JMP, 15, 0),

        //prints b
        Opcode::encode(Opcode::PUSH, 98, 0),        //98 is the utf8 code for b
        Opcode::encode(Opcode::STDOUT, 0, 1),
        Opcode::encode(Opcode::POP, 0, 0),

        //if number == 100 program quits
        Opcode::encode(Opcode::POP, 0, 0),
        Opcode::encode(Opcode::DUP, 0, 0),
        Opcode::encode(Opcode::PUSH, 100, 0),
        Opcode::encode(Opcode::CMP, 0, 0),
        Opcode::encode(Opcode::JE, 3, 1),

        //continues
        Opcode::encode(Opcode::POP, 0, 0),
        Opcode::encode(Opcode::JMP, 1, 0),

        Opcode::encode(Opcode::HALT, 0, 0),
    ];

    let mut cpu = CPU::new(program);
    cpu.run();
}