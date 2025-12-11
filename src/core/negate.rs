//! 6.1.1910 NEGATE
//! negate CORE
//!
//!     ( n1 -- n2 )
//!
//! Negate n1, giving its arithmetic inverse n2.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.1910`]

use crate::types::{Cell, Number};

crate::define_word!(Negate, "NEGATE", |it, _tks| {
    let n1: Number = it.pop_last_stack()? as _;
    let n2: Number = -n1;

    it.stack.push(n2 as Cell);

    Ok(())
});
