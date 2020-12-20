use crate::assembler::tokens::*;

pub struct Assembler {
    source: Vec<Token>,
    index: usize
}