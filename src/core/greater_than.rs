//! 6.1.0540 >
//! greater-than CORE
//! 
//!     ( n1 n2 -- flag )
//! 
//! flag is true if and only if n1 is greater than n2. 
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0540`]
use crate::types::{FALSE_FLAG, Number, TRUE_FLAG};
crate::define_word!(GreaterThan, ">", |it, _tks| {
    let n2: Number = it.pop_last_stack()? as Number;
    let n1: Number = it.pop_last_stack()? as Number;

    if n1 > n2 {
        it.stack.push(TRUE_FLAG);
    } else {
        it.stack.push(FALSE_FLAG);
    }

    Ok(())
});
