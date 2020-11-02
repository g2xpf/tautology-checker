use std::collections::VecDeque;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Var(char),
    Bottom,
    Top,
    Not,
    LParen,
    RParen,
    Arrow,
    LRArrow,
    And,
    Or,
}

#[derive(Debug)]
pub struct InvalidCharacterError(char);
impl fmt::Display for InvalidCharacterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unexpected token: {}", self.0)
    }
}
impl Error for InvalidCharacterError {}

#[derive(Debug)]
pub struct Lexer(pub VecDeque<Token>);

impl Lexer {
    pub fn new<I>(iter: I) -> Result<Self, InvalidCharacterError>
    where
        I: IntoIterator<Item = char>,
    {
        use Token::*;

        let mut v = VecDeque::new();

        for c in iter {
            let token = match c {
                ' ' => continue,
                '¬' => Not,
                '∧' => And,
                '∨' => Or,
                '(' => LParen,
                ')' => RParen,
                '→' => Arrow,
                '⊥' => Bottom,
                'T' => Top,
                '↔' => LRArrow,
                'a'..='z' => Var(c),
                _ => return Err(InvalidCharacterError(c)),
            };
            v.push_back(token);
        }
        Ok(Lexer(v))
    }

    pub fn lookahead1(&self) -> Option<&Token> {
        if !self.0.is_empty() {
            Some(&self.0[0])
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn peek(&mut self) -> Option<Token> {
        self.0.pop_front()
    }
}
