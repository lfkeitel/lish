use std::fmt;
use std::rc::Rc;

use super::token::{self, Token, TokenType};
use crate::ast;

pub enum ParserError {
    InvalidCode(String),
    ExpectedToken(String),
    ValidationError(String),
    FileNotFound(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::InvalidCode(s) => write!(f, "{}", s),
            ParserError::ExpectedToken(s) => write!(f, "{}", s),
            ParserError::ValidationError(s) => write!(f, "{}", s),
            ParserError::FileNotFound(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::InvalidCode(s) => write!(f, "{}", s),
            ParserError::ExpectedToken(s) => write!(f, "{}", s),
            ParserError::ValidationError(s) => write!(f, "{}", s),
            ParserError::FileNotFound(s) => write!(f, "{}", s),
        }
    }
}

pub struct Parser<'a> {
    lexer: Box<&'a mut Iterator<Item = Token>>,
    cur_tok: Token,
    peek_tok: Token,
}

impl<'a> Parser<'a> {
    pub fn new<I>(lexer: &'a mut I) -> Self
    where
        I: Iterator<Item = Token>,
    {
        let cur = lexer
            .next()
            .unwrap_or_else(|| token::Token::simple(TokenType::EOF, 0, 0, ""));
        let peek = lexer
            .next()
            .unwrap_or_else(|| token::Token::simple(TokenType::EOF, 0, 0, ""));

        Parser {
            lexer: Box::new(lexer),
            cur_tok: cur,
            peek_tok: peek,
        }
    }

    pub fn parse(mut self) -> Result<ast::Node, ParserError> {
        let mut forms = ast::list::List::new();

        while self.cur_tok.ttype != TokenType::EOF {
            let res: Result<ast::Node, ParserError> = match self.cur_tok.ttype {
                // Skip empty lines
                TokenType::COMMENT => {
                    self.read_token();
                    continue;
                }

                TokenType::LPAREN => self.parse_list(),

                _ => Err(ParserError::InvalidCode(format!(
                    "{}: line {}, col {} Unknown token {}",
                    self.cur_tok.file, self.cur_tok.line, self.cur_tok.col, self.cur_tok.ttype
                ))),
            };

            match res {
                Ok(node) => forms = forms.append(node),
                Err(e) => return Err(e),
            };

            self.read_token()
        }

        Ok(ast::Node::List(Rc::new(forms)))
    }

    fn read_token(&mut self) {
        self.cur_tok = self.peek_tok.clone();
        self.peek_tok = self
            .lexer
            .next()
            .unwrap_or_else(|| token::Token::simple(TokenType::EOF, 0, 0, ""));

        while self.peek_tok.ttype == TokenType::COMMENT {
            self.peek_tok = self
                .lexer
                .next()
                .unwrap_or_else(|| token::Token::simple(TokenType::EOF, 0, 0, ""));
        }
    }

    // Utility methods
    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_tok.ttype == t
    }

    fn parse_err(&self, msg: &str) -> ParserError {
        ParserError::InvalidCode(format!(
            "{} on line {} in {}",
            msg, self.cur_tok.line, self.cur_tok.file
        ))
    }

    fn token_err(&self, t: TokenType) -> ParserError {
        ParserError::ExpectedToken(format!(
            "expected {} on line {} in {}, got {}",
            t, self.cur_tok.line, self.cur_tok.file, self.cur_tok.ttype
        ))
    }

    fn tokens_err(&self, t: &[TokenType]) -> ParserError {
        ParserError::ExpectedToken(format!(
            "expected {:?} on line {} in {}, got {}",
            t, self.cur_tok.line, self.cur_tok.file, self.cur_tok.ttype
        ))
    }

    fn expect_token(&mut self, t: TokenType) -> Result<(), ParserError> {
        self.read_token();
        if !self.cur_token_is(t) {
            Err(self.token_err(t))
        } else {
            Ok(())
        }
    }

    fn parse_list(&mut self) -> Result<ast::Node, ParserError> {
        let mut form = ast::list::List::new();
        self.read_token();

        while !self.cur_token_is(TokenType::RPAREN) {
            match self.cur_tok.ttype {
                TokenType::SYMBOL => {
                    let s = ast::Symbol::new(&self.cur_tok.literal);
                    form = form.append(ast::Node::Symbol(Rc::new(s)));
                    self.read_token();
                }

                TokenType::NUMBER => {
                    let n = parse_u64(&self.cur_tok.literal).ok_or_else(|| {
                        ParserError::InvalidCode(format!(
                            "{}: line {}, col {} Failed parsing number",
                            self.cur_tok.file, self.cur_tok.line, self.cur_tok.col,
                        ))
                    })?;
                    form = form.append(ast::Node::Number(n));
                    self.read_token();
                }

                TokenType::STRING => {
                    form = form.append(ast::Node::String(self.cur_tok.literal.clone()));
                    self.read_token();
                }

                _ => {
                    return Err(self.tokens_err(&[
                        TokenType::LPAREN,
                        TokenType::NUMBER,
                        TokenType::STRING,
                        TokenType::SYMBOL,
                    ]));
                }
            };
        }

        Ok(ast::Node::List(Rc::new(form)))
    }
}

fn parse_u64(s: &str) -> Option<u64> {
    if s.starts_with("0x") {
        match u64::from_str_radix(s.trim_start_matches("0x"), 16) {
            Ok(n) => Some(n),
            Err(_) => None,
        }
    } else {
        match s.parse::<u64>() {
            Ok(n) => Some(n),
            Err(_) => None,
        }
    }
}
