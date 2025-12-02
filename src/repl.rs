use std::{
    eprintln,
    io::{self, Write},
    string::String,
};

use crate::{interpreter::Interpreter, tokens::Token};

pub const REPL_HEADER_TEXT: &str = "Formi 0.1.0 REPL\n\nCrafted by AMICAL SYSTEMS.\n";

pub fn repl() -> ! {
    let mut stdout = io::stdout();

    stdout.write_all(REPL_HEADER_TEXT.as_bytes()).unwrap();
    stdout.flush().unwrap();

    let mut interpreter = Interpreter::init(io::stdout());

    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        stdout.write(b"> ").unwrap();
        stdout.flush().unwrap();

        stdin.read_line(&mut line).unwrap();
        let tokens = line.split_whitespace().map(|s| Token::from(s)).collect();
        if let Err(e) = interpreter.execute_tokens(tokens) {
            eprintln!(" ? runtime error: {e}");
        }
        line.clear();
    }
}
