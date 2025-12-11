//! 6.1.1720 INVERT
//! invert CORE
//!
//!     ( x1 -- x2 )
//!
//! Invert all bits of x1, giving its logical inverse x2.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.1720`]

use crate::types::Cell;

crate::define_word!(Invert, "INVERT", |it, _tks| {
    let x1: Cell = it.pop_last_stack()?;
    let x2: Cell = !x1;
    it.stack.push(x2);

    Ok(())
});
