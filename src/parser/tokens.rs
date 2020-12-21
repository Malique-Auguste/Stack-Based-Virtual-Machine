#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier(String),
    Str(String),
    Num(i16)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize
}

impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Token {
        Token { token_type, line }
    }
}