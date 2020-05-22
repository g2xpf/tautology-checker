use super::lexer::{Lexer, Token};
use std::error::Error;
use std::fmt;
use std::result;

type Result<T> = result::Result<T, ParseError>;

trait Parse: Sized {
    fn parse(lexer: &mut Lexer) -> Result<Self>;
}

trait Bite {
    fn bite(&mut self, token: Token) -> Result<()>;
}

trait ParseStream {
    fn parse<T>(&mut self) -> Result<T>
    where
        T: Parse;
}

impl Bite for Lexer {
    fn bite(&mut self, token: Token) -> Result<()> {
        if let Some(t) = self.peek() {
            if t == token {
                Ok(())
            } else {
                Err(ParseError::UnexpectedToken(token))
            }
        } else {
            Err(ParseError::NoTokensLeft)
        }
    }
}

impl ParseStream for Lexer {
    fn parse<T>(&mut self) -> Result<T>
    where
        T: Parse,
    {
        match self.lookahead1() {
            Some(_) => T::parse(self),
            None => Err(ParseError::NoTokensLeft),
        }
    }
}

#[derive(Debug)]
pub struct Parser(pub Expr);

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self> {
        let expr = lexer.parse()?;
        if !lexer.is_empty() {
            return Err(ParseError::RedundantToken(lexer.peek().unwrap()));
        }
        Ok(Parser(expr))
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    NoTokensLeft,
    RedundantToken(Token),
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ParseError::*;
        match self {
            UnexpectedToken(token) => write!(f, "Unexpected token: {:?}", token),
            NoTokensLeft => write!(f, "No tokens left"),
            RedundantToken(token) => write!(f, "Redundant token: {:?}", token),
        }
    }
}

impl Error for ParseError {}

#[derive(Debug)]
pub enum Expr {
    Sole(SoleExpr),
    UnOp(UnOpExpr),
    BinOp(BinOpExpr),
}

impl Parse for Expr {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        use Expr::*;
        match lexer.lookahead1().ok_or(ParseError::NoTokensLeft)? {
            Token::Var(_) => lexer.parse().map(Sole),
            Token::Not => lexer.parse().map(UnOp),
            Token::LParen => {
                lexer.bite(Token::LParen)?;
                let ret = lexer.parse().map(BinOp);
                lexer.bite(Token::RParen)?;
                ret
            }
            e => Err(ParseError::UnexpectedToken(*e)),
        }
    }
}

#[derive(Debug)]
pub struct SoleExpr(pub char);

impl Parse for SoleExpr {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        use Token::*;
        if let Some(token) = lexer.peek() {
            if let Var(c) = token {
                Ok(SoleExpr(c))
            } else {
                Err(ParseError::UnexpectedToken(token))
            }
        } else {
            Err(ParseError::NoTokensLeft)
        }
    }
}

#[derive(Debug)]
pub struct UnOpExpr {
    pub op: UnOp,
    pub expr: Box<Expr>,
}

impl Parse for UnOpExpr {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        let op = lexer.parse()?;
        let expr = lexer.parse()?;
        Ok(UnOpExpr {
            op,
            expr: Box::new(expr),
        })
    }
}

#[derive(Debug)]
pub struct BinOpExpr {
    pub lhs: Box<Expr>,
    pub op: BinOp,
    pub rhs: Box<Expr>,
}

impl Parse for BinOpExpr {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        let lhs = lexer.parse()?;
        let op = lexer.parse()?;
        let rhs = lexer.parse()?;
        Ok(BinOpExpr {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        })
    }
}

#[derive(Debug)]
pub enum UnOp {
    Not,
}

impl Parse for UnOp {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        use Token::*;
        match lexer.peek().ok_or(ParseError::NoTokensLeft)? {
            Not => Ok(UnOp::Not),
            e => Err(ParseError::UnexpectedToken(e)),
        }
    }
}

#[derive(Debug)]
pub enum BinOp {
    And,
    Or,
    Imp,
}

impl Parse for BinOp {
    fn parse(lexer: &mut Lexer) -> Result<Self> {
        use Token::*;
        match lexer.peek().ok_or(ParseError::NoTokensLeft)? {
            And => Ok(BinOp::And),
            Or => Ok(BinOp::Or),
            Arrow => Ok(BinOp::Imp),
            e => Err(ParseError::UnexpectedToken(e)),
        }
    }
}

pub fn generate(input: &str) -> result::Result<Parser, Box<dyn Error>> {
    let s = input.chars();
    let lexer = Lexer::new(s)?;
    let parser = Parser::new(lexer)?;
    Ok(parser)
}

#[test]
fn a() {
    generate("(¬(a → b) → (a → ¬b))").unwrap();
}

#[test]
fn b() {
    generate("(((p → q) ∨ (q → r)) → (p → (q ∨ r)))").unwrap();
}

#[test]
fn c() {
    generate("¬a").unwrap();
}

#[test]
fn d() {
    generate("a").unwrap();
}
