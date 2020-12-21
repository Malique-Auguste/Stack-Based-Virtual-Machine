use crate::vm::instruction::Opcode;

use std::fs::read;

pub struct Reader;

impl Reader {
    pub fn read<S: Into<String>>(file_path: S) -> Vec<u32> {

        let mut bytes = read(file_path.into()).unwrap();
        let mut return_val: Vec<u32> = Vec::new();

        if bytes.len() % 4 != 0 { panic!("bytes len not a multiple of 4") }

        loop {
            return_val.push(Opcode::byte_array_to_instruction([
                bytes.remove(0),
                bytes.remove(0),
                bytes.remove(0),
                bytes.remove(0)
            ]));

            if bytes.len() == 0 { break return_val }
        }
    }
}