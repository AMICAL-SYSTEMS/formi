use crate::{error::RuntimeError, interpreter::Interpreter, token_iterator::TokenIterator};

// Import build-time module declarations and FIXED_TOKENS_MAP
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub trait FixedToken {
    fn execute(
        interpreter: &mut Interpreter,
        tokens: &mut TokenIterator<'_>,
    ) -> Result<(), RuntimeError>;
}

pub type FixedTokenExecutionRoutine =
    fn(&mut Interpreter, &mut TokenIterator<'_>) -> Result<(), RuntimeError>;
pub type FixedTokensMap = phf::Map<&'static str, FixedTokenExecutionRoutine>;

#[macro_export]
macro_rules! define_word {
    ($ty:ident, $token:literal, |$it:ident, $tks: ident| $body:block) => {
        pub struct $ty;

        impl $crate::core::FixedToken for $ty {
            #[inline(always)]
            fn execute<'a>(
                $it: &mut $crate::interpreter::Interpreter,
                $tks: &mut $crate::token_iterator::TokenIterator<'_>,
            ) -> Result<(), $crate::error::RuntimeError> {
                $body
            }
        }
    };
}
