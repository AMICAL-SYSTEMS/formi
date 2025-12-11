//! 6.1.0330 2/
//! two-slash CORE
//!
//!     ( x1 -- x2 )
//!
//! x2 is the result of shifting x1 one bit toward the least-significant
//! bit, leaving the most-significant bit unchanged.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0330`]

use crate::types::{Cell, Number};

crate::define_word!(TwoSlash, "2/", |it, _tks| {
    let x1: Number = it.pop_last_stack()? as _;
    let x2: Number = x1 >> 1;
    it.stack.push(x2 as Cell);

    Ok(())
});
