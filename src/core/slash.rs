//! 6.1.0230 /
//! slash CORE
//!
//!     ( n1 n2 -- n3 )
//!
//! Divide n1 by n2, giving the single-cell quotient n3. An ambiguous
//! condition exists if n2 is zero. If n1 and n2 differ in sign,
//! the implementation-defined result returned will be the same as
//! that returned by either the phrase >R S>D R> FM/MOD SWAP DROP or
//! the phrase >R S>D R> SM/REM SWAP DROP .
//!
//! AMICAL NOTE: Returns a runtime error if n2 is zero.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0230`]

use crate::{
    error::RuntimeError,
    types::{Cell, Number},
};

crate::define_word!(Slash, "/", |it, _tks| {
    let n2: Number = it.pop_last_stack()? as Number;
    let n1: Number = it.pop_last_stack()? as Number;

    if n2 == 0 {
        return Err(RuntimeError::DivisionByZero);
    } else {
        it.stack.push((n1 / n2) as Cell);
    }

    Ok(())
});
