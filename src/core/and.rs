//! 6.1.0720 AND
//! and CORE
//!
//!     ( x1 x2 -- x3 )
//!
//! x3 is the bit-by-bit logical and of x1 with x2.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0720`]

use crate::types::Cell;

crate::define_word!(And, "AND", |it, _tks| {
    let x2: Cell = it.pop_last_stack()?;
    let x1: Cell = it.pop_last_stack()?;

    it.stack.push(x1 & x2);

    Ok(())
});
