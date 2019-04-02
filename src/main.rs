mod terminal;

use std::io::Write as IoWrite;

use terminal::Terminal;

const PROMPT: &str = "lish$ ";

fn main() {
    let mut term = Terminal::new();

    loop {
        let line = term.readline(&PROMPT);

        if line == "quit" || line == "exit" {
            break;
        }

        writeln!(term, "{}", line).unwrap();
    }
}
