//! 6.1.1880 MIN
//! min CORE
//!
//!     ( n1 n2 -- n3 )
//!
//! n3 is the lesser of n1 and n2.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.1880`]

use crate::types::{Cell, Number};

crate::define_word!(Min, "MIN", |it, _tks| {
    let n2: Number = it.pop_last_stack()? as _;
    let n1: Number = it.pop_last_stack()? as _;

    if n1 < n2 {
        it.stack.push(n1 as Cell);
    } else {
        it.stack.push(n2 as Cell);
    }

    Ok(())
});
