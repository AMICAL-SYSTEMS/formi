//! 6.1.0320 2*
//! two-star CORE
//!
//!     ( x1 -- x2 )
//!
//! x2 is the result of shifting x1 one bit toward the most-significant bit,
//! filling the vacated least-significant bit with zero.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0320`]

use crate::types::Cell;

crate::define_word!(TwoStar, "2*", |it, _tks| {
    let x1: Cell = it.pop_last_stack()?;
    let x2: Cell = x1.wrapping_shl(1);
    it.stack.push(x2);

    Ok(())
});
