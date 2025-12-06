//! 6.1.0630 ?DUP
//! question-dupe CORE
//!
//!     ( x -- 0 | x x )
//!
//! Duplicate x if it is non-zero.  
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0630`]
use crate::error::RuntimeError;
crate::define_word!(QuestionDupe, "?DUP", |it, _tks| {
    let x = it.stack.peek().ok_or(RuntimeError::EmptyStack)?;

    if x != 0 {
        it.stack.push(x);
    }

    Ok(())
});
