#[derive(Copy, Clone, PartialEq, std::fmt::Debug)]
pub enum Opcode {
    HALT,
    ILG,
    LEN,

    POP,
    PUSH,

    ADD,
    SUB,
    MUL,
    DIV,
    MOD,

    CMP,
    JMP,

    JE,
    JNE,

    JG,
    JL,

    JGE,
    JLE,

    STDIN,
    STDOUT,
}

impl Opcode {
    pub fn encode(opcode: Opcode, operand1: i8, operand2: i8) -> u32 {
        (u16::from(opcode) as u32) << 16 | (operand1 as u32) << 8 | (operand2 as u32)
    }

    pub fn decode(instruction: u32) -> (Opcode, i8, i8) {
        let [opcode1, opcode2, operand1, operand2] = instruction.to_be_bytes();
        (
            Opcode::from((opcode1 as u16) << 8 | (opcode2 as u16)),
            operand1 as i8,
            operand2 as i8,
        )
    }
}

impl From<u16> for Opcode {
    fn from(o: u16) -> Opcode {
        match o {
            0 => Opcode::HALT,
            1 => Opcode::LEN,
            2 => Opcode::POP,
            3 => Opcode::PUSH,

            4 => Opcode::ADD,
            5 => Opcode::SUB,
            6 => Opcode::MUL,
            7 => Opcode::DIV,
            8 => Opcode::MOD,

            9 => Opcode::CMP,
            10 => Opcode::JMP,
            11 => Opcode::JE,
            12 => Opcode::JNE,
            13 => Opcode::JG,
            14 => Opcode::JL,
            15 => Opcode::JGE,
            16 => Opcode::JLE,

            17 => Opcode::STDIN,
            18 => Opcode::STDOUT,

            u16::MAX | _ => Opcode::ILG,
        }
    }
}

impl From<Opcode> for u16 {
    fn from(v: Opcode) -> u16 {
        match v {
            Opcode::ILG => u16::MAX,
            Opcode::HALT => 0,
            Opcode::LEN => 1,
            Opcode::POP => 2,
            Opcode::PUSH => 3,

            Opcode::ADD => 4,
            Opcode::SUB => 5,
            Opcode::MUL => 6,
            Opcode::DIV => 7,
            Opcode::MOD => 8,

            Opcode::CMP => 9,
            Opcode::JMP => 10,
            Opcode::JE => 11,
            Opcode::JNE => 12,
            Opcode::JG => 13,
            Opcode::JL => 14,
            Opcode::JGE => 15,
            Opcode::JLE => 16,

            Opcode::STDIN => 17,
            Opcode::STDOUT => 18,
        }
    }
}
