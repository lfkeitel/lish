use std::io::{self, stdin, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const INPUT_BUF_SIZE: usize = 1024;

pub struct Terminal {
    history: Vec<String>,
    history_item: usize, // Index into history
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            history: Vec::with_capacity(10),
            history_item: 0,
        }
    }

    pub fn readline(&mut self, prompt: &str) -> String {
        let mut stdout = stdout()
            .into_raw_mode()
            .expect("Failed to enable raw mode on std input");

        let mut buf = vec![0 as char; INPUT_BUF_SIZE];
        let mut i = 0;

        write!(stdout, "{}", prompt).unwrap();
        stdout.flush().unwrap();

        for c in stdin().keys() {
            match c.unwrap() {
                Key::Char(c) => {
                    if (c as u8) == 0x0A || (c as u8) == 0x0D {
                        write!(stdout, "\n\r").unwrap();
                        stdout.flush().unwrap();
                        break;
                    }

                    buf[i] = c;
                    if i < INPUT_BUF_SIZE {
                        i += 1;
                    }
                    write!(stdout, "{}", c).unwrap();
                    stdout.flush().unwrap();
                }
                Key::Backspace => {
                    if i > 0 {
                        i -= 1;
                        buf[i] = 0 as char;
                        write!(
                            stdout,
                            "{} {}",
                            termion::cursor::Left(1),
                            termion::cursor::Left(1)
                        )
                        .unwrap();
                        stdout.flush().unwrap();
                    }
                }
                _ => {}
            }
        }

        let line: String = buf[..i].iter().collect();

        self.history.push(line.clone());
        self.history_item += 1;

        line
    }
}

impl io::Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        stdout().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        stdout().flush()
    }
}
