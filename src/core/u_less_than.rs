//! 6.1.2340 U<
//! u-less-than CORE
//!
//!     ( u1 u2 -- flag )
//!
//! flag is true if and only if u1 is less than u2.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.2340`]
use crate::types::{FALSE_FLAG, TRUE_FLAG, UnsignedInteger};
crate::define_word!(ULessThan, "U<", |it, _tks| {
    let u2: UnsignedInteger = it.pop_last_stack()? as UnsignedInteger;
    let u1: UnsignedInteger = it.pop_last_stack()? as UnsignedInteger;

    if u1 < u2 {
        it.stack.push(TRUE_FLAG);
    } else {
        it.stack.push(FALSE_FLAG);
    }

    Ok(())
});
