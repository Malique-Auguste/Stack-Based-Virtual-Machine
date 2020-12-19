#[derive(Copy, Clone, PartialEq, std::fmt::Debug)]
pub enum Opcode {
    HALT,
    ILG,
    LEN,

    POP,
    PUSH,
    DUP,

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
            4 => Opcode::DUP,

            5 => Opcode::ADD,
            6 => Opcode::SUB,
            7 => Opcode::MUL,
            8 => Opcode::DIV,
            9 => Opcode::MOD,

            10 => Opcode::CMP,
            11 => Opcode::JMP,
            12 => Opcode::JE,
            13 => Opcode::JNE,
            14 => Opcode::JG,
            15 => Opcode::JL,
            16 => Opcode::JGE,
            17 => Opcode::JLE,

            18 => Opcode::STDIN,
            19 => Opcode::STDOUT,

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
            Opcode::DUP => 4,

            Opcode::ADD => 5,
            Opcode::SUB => 6,
            Opcode::MUL => 7,
            Opcode::DIV => 8,
            Opcode::MOD => 9,

            Opcode::CMP => 10,
            Opcode::JMP => 11,
            Opcode::JE => 12,
            Opcode::JNE => 13,
            Opcode::JG => 14,
            Opcode::JL => 15,
            Opcode::JGE => 16,
            Opcode::JLE => 17,

            Opcode::STDIN => 18,
            Opcode::STDOUT => 19,
        }
    }
}
