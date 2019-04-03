use std::fmt::Write;
use std::io;

use super::token::Token;
use super::token::TokenType;

// Convience wrapping iter to convert iter of u8 to iter of Result<u8, io::Error>
pub struct ByteIter<'a> {
    src: Box<&'a mut dyn Iterator<Item = u8>>,
}

impl<'a> ByteIter<'a> {
    pub fn new<I>(src: &'a mut I) -> ByteIter<'a>
    where
        I: Iterator<Item = u8>,
    {
        ByteIter { src: Box::new(src) }
    }
}

impl<'a> Iterator for ByteIter<'a> {
    type Item = Result<u8, ::std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.src.next().map(Ok)
    }
}

pub struct Lexer<'a> {
    reader: Box<&'a mut dyn Iterator<Item = Result<u8, io::Error>>>,
    cur_ch: u8,
    peek_ch: u8,
    line: u32,
    col: u32,
    name: String,
}

impl<'a> Lexer<'a> {
    pub fn new<I>(src: &'a mut I, name: &str) -> Lexer<'a>
    where
        I: Iterator<Item = Result<u8, io::Error>>,
    {
        let mut l = Lexer {
            reader: Box::new(src),
            cur_ch: 0,
            peek_ch: 0,
            line: 1,
            col: 0,
            name: name.to_owned(),
        };

        l.read_char();
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        self.cur_ch = self.peek_ch;
        self.peek_ch = self.reader.next().unwrap_or(Ok(0)).unwrap_or(0);

        if self.peek_ch == b'\r' {
            self.peek_ch = self.reader.next().unwrap_or(Ok(0)).unwrap_or(0);
        }

        self.col += 1;
    }

    fn reset_pos(&mut self) {
        self.line += 1;
        self.col = 0;
    }

    fn devour_whitespace(&mut self) {
        while is_whitespace(self.cur_ch) {
            self.read_char();
        }
    }

    fn read_symbol(&mut self) -> String {
        let mut ident = String::new();
        while is_symbol(self.cur_ch) {
            ident.write_char(char::from(self.cur_ch)).unwrap();
            self.read_char();
        }
        ident
    }

    fn read_string(&mut self) -> String {
        // TODO: should probably support escape sequences

        self.read_char(); // Go over opening quote
        let mut ident = String::new();
        while self.cur_ch != b'"' {
            ident.write_char(char::from(self.cur_ch)).unwrap();
            self.read_char();
        }
        self.read_char(); // Skip closing double quote
        ident
    }

    fn read_single_line_comment(&mut self) -> String {
        self.read_char(); // Go over semicolon
        let mut comm = String::new();
        while self.cur_ch != b'\n' && self.cur_ch != 0 {
            comm.write_char(char::from(self.cur_ch)).unwrap();
            self.read_char();
        }
        comm.trim().to_owned()
    }

    fn read_number(&mut self) -> String {
        let mut num = String::new();
        while is_digit(self.cur_ch) || is_hex_digit(self.cur_ch) {
            num.write_char(char::from(self.cur_ch)).unwrap();
            self.read_char();
        }
        num
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! some_token {
            ($inst:expr) => {{
                Some(Token::simple($inst, self.line, self.col, &self.name))
            }};

            ($inst:expr, $s:expr) => {{
                Some(Token::with_literal(
                    $inst, $s, self.line, self.col, &self.name,
                ))
            }};

            ($inst:expr, $s:expr, $col:expr) => {{
                Some(Token::with_literal($inst, $s, self.line, $col, &self.name))
            }};
        }

        if self.cur_ch == b'\n' {
            self.reset_pos();
            self.read_char();
        }

        self.devour_whitespace();

        let tok = match self.cur_ch {
            b'(' => some_token!(TokenType::LPAREN),
            b')' => some_token!(TokenType::RPAREN),
            b',' => some_token!(TokenType::COMMA),
            b'\'' => some_token!(TokenType::QUOTE),
            b'"' => {
                let col = self.col;
                return some_token!(TokenType::STRING, self.read_string(), col);
            }
            b';' => {
                let col = self.col;
                return some_token!(TokenType::COMMENT, self.read_single_line_comment(), col);
            }
            0 => None,
            _ => {
                if is_digit(self.cur_ch) {
                    return some_token!(TokenType::NUMBER, self.read_number());
                } else if is_symbol(self.cur_ch) {
                    let col = self.col;
                    let lit = self.read_symbol();
                    return some_token!(TokenType::SYMBOL, lit, col);
                } else {
                    some_token!(TokenType::ILLEGAL)
                }
            }
        };

        self.read_char();
        tok
    }
}

fn is_whitespace(ch: u8) -> bool {
    ch == b' ' || ch == b'\t' || ch == b'\n' || ch == b'\r'
}

fn is_symbol(ch: u8) -> bool {
    is_letter(ch)
        || is_digit(ch)
        || ch == b'-'
        || ch == b'+'
        || ch == b'_'
        || ch == b'$'
        || ch == b'*'
        || ch == b'/'
        || ch == b'\\'
        || ch == b'='
        || ch == b'<'
        || ch == b'>'
        || ch == b'!'
        || ch == b'&'
        || ch == b'%'
        || ch == b'.'
}

fn is_letter(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z'
}

fn is_hex_digit(ch: u8) -> bool {
    b'a' <= ch && ch <= b'f' || b'A' <= ch && ch <= b'F' || ch == b'x'
}

fn is_digit(ch: u8) -> bool {
    b'0' <= ch && ch <= b'9'
}
