//! 6.1.2490 XOR
//! x-or CORE
//!
//!     ( x1 x2 -- x3 )
//!
//! x3 is the bit-by-bit exclusive-or of x1 with x2.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.2490`]

use crate::types::Cell;

crate::define_word!(Xor, "XOR", |it, _tks| {
    let x2: Cell = it.pop_last_stack()?;
    let x1: Cell = it.pop_last_stack()?;

    it.stack.push(x1 ^ x2);

    Ok(())
});
