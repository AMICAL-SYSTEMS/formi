use phf::phf_map;

pub mod plus;

use crate::{error::RuntimeError, interpreter::Interpreter};

pub trait FixedToken {
    fn execute(interpreter: &mut Interpreter) -> Result<(), RuntimeError>;
}

pub type FixedTokenExecutionRoutine = fn(&mut Interpreter) -> Result<(), RuntimeError>;

pub static FIXED_TOKENS_MAP: phf::Map<&'static str, FixedTokenExecutionRoutine> = phf_map!(
    "+" => <plus::Plus as FixedToken>::execute,
);
