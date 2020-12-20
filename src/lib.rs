pub mod cpu;
pub mod instruction;
pub mod frame;

use cpu::*;
use instruction::*;
use frame::*;

#[cfg(test)]
mod test_instruction {
    //use crate::instruction::*;
    use super::*;

    #[test]
    fn operation_decode_encode() {
        let opcode: Opcode = 1.into();
        let operand1: i8 = 1;
        let operand2: i8 = 2;

        let instruction: u32 = Opcode::encode(opcode.clone(), 1, 2);
        assert_eq!((opcode, operand1, operand2), Opcode::decode(instruction));
    }
}

#[cfg(test)]
mod test_cpu {
    use super::*;

    #[test]
    fn test_arithmetic() {
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
    fn test_logic() {
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
    fn test_functions() {
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
