mod terminal;

use std::io::Write as IoWrite;

use terminal::Terminal;

use lisp::parser::lexer::{ByteIter, Lexer};
use lisp::parser::parser::Parser;

const PROMPT: &str = "lish$ ";

fn main() {
    let mut term = Terminal::new();

    loop {
        let mut line = term.readline(&PROMPT);
        if !line.starts_with('(') {
            line = format!("({})", line);
        }
        let mut line_iter = line.bytes();
        let mut str_iter = ByteIter::new(&mut line_iter);
        let mut lex = Lexer::new(&mut str_iter, "<shell>");
        let parser = Parser::new(&mut lex);
        let tree = match parser.parse() {
            Ok(t) => t,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        println!("{:?}", tree);

        if line == "(quit)" || line == "(exit)" {
            break;
        }

        writeln!(term, "{}", line).unwrap();
    }
}
