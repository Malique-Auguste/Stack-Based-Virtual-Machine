use crate::parser::tokens::*;

pub struct Lexer {
    source: Vec<char>,
    index: usize,
    line: usize,

    pub tokens: Vec<Token>
}

impl Lexer {
    pub fn new<S: Into<String>>(source: S) -> Lexer {
        Lexer { source: source.into().chars().collect(), index: 0, tokens: Vec::new(), line: 1 }
    }

    pub fn advance(&mut self) -> Option<char> {
        self.index += 1;
        if self.index <= self.source.len() {
            return Some(self.source[self.index - 1])
        }
        else {
            return None
        }
    }

    pub fn peek(&self) -> Option<char> {
        if self.index + 1 <= self.source.len() {
            return Some(self.source[self.index])
        }
        else {
            return None
        }
    }

    pub fn lex(&mut self) {
        loop {
            let current_index = self.index;
            let char_type = Lexer::get_char_type(match self.advance(){
                Some(c) => c,
                None => break
            });

            match char_type {
                CharType::Comma | CharType::WhiteSpace | CharType::Return | CharType::Tab => continue,
                CharType::Colon => panic!("Colon encountered outside of identifier"),
                CharType::Illegal => panic!("Illegal character ({}) encountered at line: {}", self.source[current_index], self.line),

                CharType::Newline => self.line += 1,

                CharType::Letter => {
                    loop {
                        let end_index = self.index;
                        let c = Lexer::get_char_type(match self.peek(){
                            Some(c) => c,
                            None => {
                                self.tokens.push(Token::new( TokenType::Str(self.source[current_index..end_index].iter().collect()), self.line ));
                                break;
                            }
                        });

                        if CharType::Colon == c {
                            self.advance();
                            self.tokens.push(Token::new( TokenType::Identifier(self.source[current_index..end_index].iter().collect()), self.line ));
                            break;
                        }
                        else if CharType::Letter != c {
                            self.tokens.push(Token::new( TokenType::Str(self.source[current_index..end_index].iter().collect()), self.line ));
                            break;
                        }

                        self.advance();
                    }
                },

                CharType::Num => {
                    loop {
                        let end_index = self.index;
                        let c = Lexer::get_char_type(match self.peek(){
                            Some(c) => c,
                            None => {
                                let num = self.source[current_index..end_index].iter().collect::<String>().parse::<i8>().unwrap();
                                self.tokens.push(Token::new(TokenType::Num(num), self.line));
                                break;
                            }
                        });

                        if c != CharType::Num {
                            let num = self.source[current_index..end_index].iter().collect::<String>().parse::<i8>().unwrap();
                            self.tokens.push(Token::new(TokenType::Num(num), self.line));
                            break;
                        }

                        self.advance();
                    }
                }
            }
        }
    }

    pub fn get_char_type(input: char) -> CharType {
        match input {
            'a'..='z' | 'A'..='Z' => CharType::Letter,
            '0'..='9' => CharType::Num,
            ':' => CharType::Colon,
            ',' => CharType::Comma,
            ' ' => CharType::WhiteSpace,

            '\n' => CharType::Newline,
            '\r' => CharType::Return,
            '\t' => CharType::Tab,

            _ => CharType::Illegal
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CharType {
    Letter,
    Num,
    Colon,
    Comma,
    WhiteSpace,

    Newline,
    Return,
    Tab,

    Illegal,
}