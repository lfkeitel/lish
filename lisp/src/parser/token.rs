#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    COMMENT,
    COMMA,
    SYMBOL,
    NUMBER,
    STRING,
    RPAREN,
    LPAREN,
    QUOTE,
}

impl ::std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenType::ILLEGAL => "ILLEGAL",
                TokenType::EOF => "EOF",
                TokenType::COMMENT => "COMMENT",
                TokenType::COMMA => "COMMA",
                TokenType::SYMBOL => "SYMBOL",
                TokenType::NUMBER => "NUMBER",
                TokenType::STRING => "STRING",
                TokenType::RPAREN => "RPAREN",
                TokenType::LPAREN => "LPAREN",
                TokenType::QUOTE => "QUOTE",
            }
        )
    }
}

impl ::std::fmt::Debug for TokenType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String,
    pub line: u32,
    pub col: u32,
    pub file: String,
}

impl Token {
    pub fn with_literal(t: TokenType, lit: String, line: u32, col: u32, file: &str) -> Self {
        Token {
            ttype: t,
            literal: lit,
            line,
            col,
            file: file.to_owned(),
        }
    }

    pub fn simple(t: TokenType, line: u32, col: u32, file: &str) -> Self {
        Self::with_literal(t, "".to_string(), line, col, file)
    }
}
