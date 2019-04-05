pub mod lexer;
pub mod parser;
mod token;

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::object;
use parser::{Parser, ParserError};

pub fn compile_file(filepath: &Path) -> Result<object::Program, parser::ParserError> {
    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => return Err(ParserError::FileNotFound(format!("{}", err))),
    };

    let buf = BufReader::new(file);
    let mut reader = buf.bytes();
    let mut lex = lexer::Lexer::new(&mut reader, filepath.to_str().unwrap_or_default());
    Parser::new(&mut lex).parse()
}

pub fn compile_string(code: &str, filename: &str) -> Result<object::Program, parser::ParserError> {
    let mut iter = code.bytes();
    let mut reader = lexer::ByteIter::new(&mut iter);
    let mut lex = lexer::Lexer::new(&mut reader, filename);
    Parser::new(&mut lex).parse()
}
