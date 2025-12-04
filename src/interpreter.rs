use core::ops::Range;
use std::{
    collections::HashMap,
    io::{Stdout, Write},
    string::{String, ToString},
};

use crate::{core::FIXED_TOKENS_MAP, error::RuntimeError, stack::Stack};

#[derive(Debug, PartialEq, Eq)]
pub enum InterpreterState {
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
    pub state: InterpreterState,
    pub definitions: HashMap<String, String>,
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
                fixed_token_exec_routine(self, &mut tokens)?;
            } else {
                match token.parse::<u64>() {
                    Ok(number) => self.stack.push(number),
                    Err(_) => {
                        // Could be a word
                        if let Some(tokens) = self.definitions.get(token).cloned() {
                            self.state = InterpreterState::Word;
                            self.execute_tokens(tokens)?;
                        } else {
                            return Err(RuntimeError::UnknownToken(token.to_string()));
                        }
                    }
                }
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
