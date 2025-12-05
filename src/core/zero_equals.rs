//! 6.1.0250 0<
//! zero-less CORE
//!
//!     ( n -- flag )
//!
//! flag is true if and only if n is less than zero.
//!
//! [https://www.taygeta.com/forth/dpans6.htm#6.2.0250]

use crate::types::{FALSE_FLAG, Number, TRUE_FLAG};
crate::define_word!(ZeroEquals, "0=", |it, _tks| {
    let n: Number = it.pop_last_stack()? as Number;

    if n == 0 {
        it.stack.push(TRUE_FLAG);
    } else {
        it.stack.push(FALSE_FLAG);
    }

    Ok(())
});
