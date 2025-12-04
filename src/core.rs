use core::str::SplitAsciiWhitespace;

use crate::{error::RuntimeError, interpreter::Interpreter};

pub mod colon;
pub mod cr;
pub mod r#do;
pub mod dot;
pub mod dot_quote;
pub mod i;
pub mod r#loop;
pub mod minus;
pub mod modulo;
pub mod plus;
pub mod quote;
pub mod semi_colon;
pub mod slash;
pub mod star;

pub trait FixedToken {
    fn execute(
        interpreter: &mut Interpreter,
        tokens: &mut SplitAsciiWhitespace<'_>,
    ) -> Result<(), RuntimeError>;
}

pub type FixedTokenExecutionRoutine =
    fn(&mut Interpreter, &mut SplitAsciiWhitespace<'_>) -> Result<(), RuntimeError>;
pub type FixedTokensMap = phf::Map<&'static str, FixedTokenExecutionRoutine>;

#[macro_export]
macro_rules! define_word {
    ($ty:ident, $token:literal, |$it:ident, $tks: ident| $body:block) => {
        pub struct $ty;

        impl $crate::core::FixedToken for $ty {
            fn execute<'a>(
                $it: &mut $crate::interpreter::Interpreter,
                $tks: &mut core::str::SplitAsciiWhitespace<'_>,
            ) -> Result<(), $crate::error::RuntimeError> {
                $body
            }
        }
    };
}

// Import build-time FIXED_TOKENS_MAP
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
