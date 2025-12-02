use core::ops::Range;
use std::{
    collections::HashMap,
    io::{Stdout, Write},
    string::{String, ToString},
    vec,
};

use alloc::format;

use crate::{
    error::RuntimeError,
    stack::Stack,
    tokens::{Token, Tokens},
};

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
    stack: Stack,
    state: InterpreterState,
    definitions: HashMap<String, Tokens>,
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

    pub fn execute_tokens(&mut self, mut tokens: Tokens) -> Result<(), RuntimeError> {
        while let Some(token) = tokens.pop_front() {
            match token {
                Token::Plus => {
                    let a = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    let b = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    self.stack.push(a + b);
                }
                Token::Minus => {
                    let a = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    let b = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    self.stack.push(a - b);
                }
                Token::Mul => {
                    let a = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    let b = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    self.stack.push(a * b);
                }
                Token::Div => {
                    let a = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    let b = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    if b == 0 {
                        return Err(RuntimeError::DivisionByZero);
                    } else {
                        self.stack.push(a / b);
                    }
                }
                Token::Mod => {
                    let a = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    let b = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    if b == 0 {
                        return Err(RuntimeError::DivisionByZero);
                    } else {
                        self.stack.push(a.rem_euclid(b));
                    }
                }
                Token::Dot => {
                    let a = self.stack.pop().ok_or(RuntimeError::EmptyStack)?;
                    self.stdout_write(a.to_string().as_bytes())?;
                }
                Token::Colon => {
                    self.state = InterpreterState::Word;
                    let word_token = tokens.pop_front().ok_or(RuntimeError::EmptyStack)?;
                    if let Token::Word(word) = word_token {
                        let mut def_token = vec![];
                        while let Some(token) = tokens.pop_front() {
                            match token {
                                Token::SemiColon => break,
                                other => def_token.push(other),
                            }
                        }

                        if self
                            .definitions
                            .insert(word.clone(), def_token.into())
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
                Token::SemiColon => {
                    debug_assert_eq!(self.state, InterpreterState::Word);
                    self.state = InterpreterState::Normal;
                }
                Token::Cr => {
                    self.stdout_write(b"\n")?;
                }
                Token::Number(nb) => {
                    self.stack.push(nb);
                }
                Token::Word(word) => {
                    if let Some(tokens) = self.definitions.get(&word).cloned() {
                        self.state = InterpreterState::Word;
                        self.execute_tokens(tokens)?;
                    }
                }
                Token::DotQuote => {
                    let mut vec_token = vec![];
                    while let Some(token) = tokens.pop_front() {
                        match token {
                            Token::Quote => break,
                            other => vec_token.push(other),
                        }
                    }

                    self.stdout_write(format!("{:?}", vec_token).as_bytes())?;
                }
                Token::Do => {
                    let min_bound = self.pop_last_stack()?;
                    let max_bound = self.pop_last_stack()?;
                    let range = min_bound..max_bound;

                    let mut vec_token = vec![];
                    while let Some(token) = tokens.pop_front() {
                        match token {
                            Token::Loop => break,
                            other => vec_token.push(other),
                        }
                    }

                    let looping_tokens: Tokens = vec_token.into();
                    for i in range.clone() {
                        self.state = InterpreterState::Loop(range.clone(), i);
                        self.execute_tokens(looping_tokens.clone())?;
                    }
                    self.state = InterpreterState::Normal
                }
                Token::I => {
                    if let InterpreterState::Loop(_, i) = self.state {
                        self.stack.push(i);
                    } else {
                        return Err(RuntimeError::NotInLoop);
                    }
                }
                Token::Quote | Token::Loop => { /* no-op */ }
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
