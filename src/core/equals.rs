//! 6.1.0530 =
//! equals CORE
//!
//!     ( x1 x2 -- flag )
//!
//! flag is true if and only if x1 is bit-for-bit the same as x2.
//! 
//! AMICAL NOTE: Unsigned integers can be compared with signed integers
//! and be equal. For instance, `true -1 =` returns `true`.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0530`]

use crate::types::{FALSE_FLAG, TRUE_FLAG};
crate::define_word!(Equals, "=", |it, _tks| {
    let x2 = it.pop_last_stack()?;
    let x1 = it.pop_last_stack()?;

    if x1.to_le_bytes() == x2.to_le_bytes() {
        it.stack.push(TRUE_FLAG);
    } else {
        it.stack.push(FALSE_FLAG);
    }

    Ok(())
});
