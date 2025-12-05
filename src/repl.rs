use std::{
    eprintln,
    io::{self, Write},
    string::{String, ToString},
};

use crate::interpreter::Interpreter;

pub const REPL_HEADER_TEXT: &str = "Formi REPL -- Crafted by AMICAL SYSTEMS.\n";

pub fn repl() -> ! {
    let mut stdout = io::stdout();

    stdout.write_all(REPL_HEADER_TEXT.as_bytes()).unwrap();
    stdout.flush().unwrap();

    let mut interpreter = Interpreter::init(io::stdout());

    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        stdout.write_all(b"> ").unwrap();
        stdout.flush().unwrap();

        stdin.read_line(&mut line).unwrap();
        if let Err(e) = interpreter.execute_tokens(line.to_string()) {
            eprintln!("? runtime error: {e}");
        }
        line.clear();
    }
}
