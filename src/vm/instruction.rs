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

    LOAD,
    STORE,
    CALL,
    RETURN
}

impl Opcode {
    pub fn encode(opcode: Opcode, operand1: i16, operand2: i8) -> u32 {
        (u8::from(opcode) as u32) << 24 | (operand1 as u32) << 16 | (operand2 as u32)
    }

    pub fn decode(instruction: u32) -> (Opcode, i16, i8) {
        let [opcode1, operand1_lower, operand1_upper, operand2] = instruction.to_be_bytes();
        (
            Opcode::from(opcode1),
            ((operand1_upper as u16) << 8 | operand1_lower as u16) as i16,
            operand2 as i8,
        )
    }

    pub fn instruction_to_byte_array(instruction: u32) -> [u8; 4] {
        instruction.to_be_bytes()
    }

    pub fn byte_array_to_instruction(array: [u8; 4]) -> u32 {
        (array[0] as u32) << 24 | (array[1] as u32) << 16 | (array[2] as u32) << 8 | array[3] as u32
    }
}

impl From<u8> for Opcode {
    fn from(o: u8) -> Opcode {
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

            20 => Opcode::LOAD,
            21 => Opcode::STORE,
            22 => Opcode::CALL,
            23 => Opcode::RETURN,

            u8::MAX | _ => Opcode::ILG,
        }
    }
}

impl From<Opcode> for u8 {
    fn from(v: Opcode) -> u8 {
        match v {
            Opcode::ILG => u8::MAX,
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

            Opcode::LOAD => 20,
            Opcode::STORE => 21,
            Opcode::CALL => 22,
            Opcode::RETURN => 23,
        }
    }
}

impl From<&String> for Opcode {
    fn from(o: &String) -> Opcode {
        match o.as_str() {
            "HALT" => Opcode::HALT,
            "LEN" => Opcode::LEN,

            "POP" => Opcode::POP,
            "PUSH" => Opcode::PUSH,
            "DUP" => Opcode::DUP,

            "ADD" => Opcode::ADD,
            "SUB" => Opcode::SUB,
            "MUL" => Opcode::MUL,
            "DIV" => Opcode::DIV,
            "MOD" => Opcode::MOD,

            "CMP" => Opcode::CMP,
            "JMP" => Opcode::JMP,
            "JE" => Opcode::JE,
            "JNE" => Opcode::JNE,
            "JG" => Opcode::JG,
            "JL" => Opcode::JL,
            "JGE" => Opcode::JGE,
            "JLE" => Opcode::JLE,

            "STDIN" => Opcode::STDIN,
            "STDOUT" => Opcode::STDOUT,

            "LOAD" => Opcode::LOAD,
            "STORE" => Opcode::STORE,
            "CALL" => Opcode::CALL,
            "RETURN" => Opcode::RETURN,

            "ILG" | _ => Opcode::ILG,
        }
    }
}