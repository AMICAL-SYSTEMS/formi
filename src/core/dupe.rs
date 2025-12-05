//! 6.1.1290 DUP
//! dupe CORE
//!
//!     ( x -- x x )
//!
//! Duplicate x.
//!
//! [https://www.taygeta.com/forth/dpans6.htm#6.1.1290]

use crate::error::RuntimeError;
crate::define_word!(Dupe, "DUPE", |it, _tks| {
    it.stack
        .push(it.stack.peek().ok_or(RuntimeError::EmptyStack)?);

    Ok(())
});
