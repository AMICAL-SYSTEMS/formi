use core::ops::Range;
use std::{
    collections::HashMap,
    io::{Stdout, Write},
    string::{String, ToString},
    vec,
};

use alloc::format;

use crate::{core::FIXED_TOKENS_MAP, error::RuntimeError, stack::Stack};

#[derive(Debug, PartialEq, Eq)]
enum InterpreterState {
    Normal,
    /// Interpreting a word definition or creating one
    Word,
    Loop(
        Range<u64>, /* iterations */
        u64,        /* index of current iteration */
    ),
}

pub struct Interpreter {
    pub stack: Stack,
    state: InterpreterState,
    definitions: HashMap<String, String>,
    stdout: Stdout,
}

impl Interpreter {
    pub fn init(stdout: Stdout) -> Self {
        Self {
            stack: Stack::default(),
            state: InterpreterState::Normal,
            definitions: HashMap::new(),
            stdout,
        }
    }

    pub fn execute_tokens(&mut self, tokens: String) -> Result<(), RuntimeError> {
        let mut tokens = tokens.split_ascii_whitespace();
        while let Some(token) = tokens.next() {
            if let Some(fixed_token_exec_routine) = FIXED_TOKENS_MAP.get(token) {
                fixed_token_exec_routine(self)?
            }

            match token {
                "-" => {
                    let a = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    let b = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    self.stack.push(a - b);
                }
                "*" => {
                    let a = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    let b = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    self.stack.push(a * b);
                }
                "/" => {
                    let a = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    let b = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    if b == 0 {
                        return Err(RuntimeError::DivisionByZero);
                    } else {
                        self.stack.push(a / b);
                    }
                }
                "mod" => {
                    let a = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    let b = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    if b == 0 {
                        return Err(RuntimeError::DivisionByZero);
                    } else {
                        self.stack.push(a.rem_euclid(b));
                    }
                }
                "." => {
                    let a = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    self.stdout_write(a.to_string().as_bytes())?;
                }
                ":" => {
                    self.state = InterpreterState::Word;
                    let word_token = tokens.next().ok_or(RuntimeError::EmptyStack)?;
                    // Not a known word, not a number
                    let is_word = !FIXED_TOKENS_MAP.contains_key(word_token)
                        && word_token.parse::<u64>().is_err();

                    if is_word {
                        let word = word_token;

                        let mut def_token = vec![];
                        for token in tokens.by_ref() {
                            match token {
                                ";" => break,
                                other => def_token.push(other),
                            }
                        }

                        if self
                            .definitions
                            .insert(word.to_string(), def_token.join(" "))
                            .is_some()
                        {
                            self.stdout_write(
                                format!("? warning: definition of {word} has been overwritten")
                                    .as_bytes(),
                            )?
                        }
                    } else {
                        return Err(RuntimeError::ExpectedWord);
                    }
                }
                ";" => {
                    debug_assert_eq!(self.state, InterpreterState::Word);
                    self.state = InterpreterState::Normal;
                }
                "cr" => {
                    self.stdout_write(b"\n")?;
                }
                ".\"" => {
                    let mut vec_token = vec![];
                    for token in tokens.by_ref() {
                        match token {
                            "\"" => break,
                            other => vec_token.push(other),
                        }
                    }

                    self.stdout_write(format!("{:?}", vec_token).as_bytes())?;
                }
                "DO" => {
                    let min_bound = self.pop_last_stack()?;
                    let max_bound = self.pop_last_stack()?;
                    let range = min_bound..max_bound;

                    let mut vec_token = vec![];
                    for token in tokens.by_ref() {
                        match token {
                            "LOOP" => break,
                            other => vec_token.push(other),
                        }
                    }

                    let looping_tokens = vec_token.join(" ");
                    for i in range.clone() {
                        self.state = InterpreterState::Loop(range.clone(), i);
                        self.execute_tokens(looping_tokens.clone())?;
                    }
                    self.state = InterpreterState::Normal
                }
                "I" => {
                    if let InterpreterState::Loop(_, i) = self.state {
                        self.stack.push(i);
                    } else {
                        return Err(RuntimeError::NotInLoop);
                    }
                }
                "\"" | "LOOP" => { /* no-op */ }
                unknown => match unknown.parse::<u64>() {
                    Ok(number) => self.stack.push(number),
                    Err(_) => {
                        // Could be a word
                        if let Some(tokens) = self.definitions.get(unknown).cloned() {
                            self.state = InterpreterState::Word;
                            self.execute_tokens(tokens)?;
                        }
                    }
                },
            }
        }

        Ok(())
    }

    #[inline]
    pub fn pop_last_stack(&mut self) -> Result<u64, RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::EmptyStack)
    }

    #[inline]
    pub fn stdout_write(&mut self, bytes: &[u8]) -> Result<(), RuntimeError> {
        self.stdout
            .write_all(bytes)
            .map_err(|_| RuntimeError::IOError)?;
        self.stdout.flush().map_err(|_| RuntimeError::IOError)?;

        Ok(())
    }
}
