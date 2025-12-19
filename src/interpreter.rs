use std::{
    collections::HashMap,
    io::{Stdout, Write},
    string::{String, ToString},
};

use crate::{
    core::FIXED_TOKENS_MAP,
    error::RuntimeError,
    r#loop::LoopSys,
    stack::{DataStack, ReturnStack},
    token_iterator::TokenIterator,
    types::{Cell, CellPair, Number, UnsignedInteger},
};

#[derive(Debug, PartialEq, Eq)]
pub enum InterpreterState {
    Interpret,
    Loop(LoopSys),
}

#[derive(Debug)]
pub struct Interpreter {
    pub stack: DataStack,
    pub return_stack: ReturnStack,
    pub state: InterpreterState,
    pub definitions: HashMap<String, String>,
    stdout: Stdout,
}

impl Interpreter {
    pub fn init(stdout: Stdout) -> Self {
        Self {
            stack: DataStack::default(),
            return_stack: ReturnStack::default(),
            state: InterpreterState::Interpret,
            definitions: HashMap::new(),
            stdout,
        }
    }

    pub fn execute_tokens(&mut self, tokens: String) -> Result<(), RuntimeError> {
        let mut tokens = TokenIterator::new(&tokens);
        while let Some(token) = tokens.next() {
            if let Some(fixed_token_exec_routine) = FIXED_TOKENS_MAP.get(token) {
                // Core word
                fixed_token_exec_routine(self, &mut tokens)?;
            } else if let Ok(number) = token.parse::<Number>() {
                // Signed number
                self.stack.push(number as Cell);
            } else if let Ok(number) = token.parse::<UnsignedInteger>() {
                // Unsigned number
                self.stack.push(number as Cell);
            } else if let Some(tokens) = self.definitions.get(token).cloned() {
                // Word defined at runtime
                self.execute_tokens(tokens)?;
            } else {
                return Err(RuntimeError::UnknownToken(token.to_string()));
            }
        }

        Ok(())
    }

    #[inline]
    pub fn pop_last_stack(&mut self) -> Result<Cell, RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::EmptyStack)
    }

    #[inline]
    pub fn pop_last_return_stack(&mut self) -> Result<Cell, RuntimeError> {
        self.return_stack.pop().ok_or(RuntimeError::EmptyStack)
    }

    #[inline]
    pub fn pop_pair_last_stack(&mut self) -> Result<CellPair, RuntimeError> {
        let c2: Cell = self.pop_last_stack()?;
        let c1: Cell = self.pop_last_stack()?;

        Ok((c1, c2))
    }

    #[inline]
    pub fn push_pair_stack(&mut self, pair: CellPair) {
        self.stack.push(pair.0);
        self.stack.push(pair.1);
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
